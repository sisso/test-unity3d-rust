using System.Runtime.InteropServices;
using System;
using FlatBuffers;

namespace Domain
{
    static class FFI
    {
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern Int32 add_numbers(Int32 a, Int32 b);

        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler context_create();
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern void context_close(IntPtr ptr);

        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool free_string(IntPtr ptr);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_string(ContextHandler ptr, string str);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern FFIStringHandler context_get_string(ContextHandler ptr);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_struct(ContextHandler ptr, V2 v2);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern V2 context_get_struct(ContextHandler ptr);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_array(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_array(ContextHandler ptr, Action<IntPtr, UInt32> callback);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_struct_array(ContextHandler ptr, V2[] buffer, UInt32 len);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_struct_array(ContextHandler ptr, Action<IntPtr, UInt32> callback);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_people(ContextHandler ptr, [In] FFIPerson[] buffer, UInt32 len);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_people(ContextHandler ptr, Action<IntPtr, UInt32> callback);

        [DllImport("librustlib.so")]
        internal static extern bool context_set_flatbuffer(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_flatbuffer(ContextHandler ptr, Action<IntPtr, UInt32> callback);
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct FFIPerson
    {
        public UInt32 id;
        public string name;
        public Int32 number;

        public override string ToString()
        {
            return string.Format("[FFIPerson: id={0}, name={1}, number={2}]", id, name, number);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct V2
    {
        public Int32 x;
        public Int32 y;

        public override string ToString()
        {
            return string.Format("[V2: x={0}, y={1}]", x, y);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    struct FFIArray
    {
        public IntPtr ptr;
        public UInt32 len;
    }

    internal class FFIStringHandler : SafeHandle
    {
        public FFIStringHandler() : base(IntPtr.Zero, true)
        {
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return System.Text.Encoding.UTF8.GetString(buffer);
        }

        public override bool IsInvalid
        {
            get
            {
                return false;
            }
        }

        protected override bool ReleaseHandle()
        {
            FFI.free_string(handle);
            return true;
        }
    }

    internal class ContextHandler : SafeHandle
    {
        public ContextHandler() : base(IntPtr.Zero, true)
        {
        }

        public override bool IsInvalid
        {
            get
            {
                return false;
            }
        }

        protected override bool ReleaseHandle()
        {
            FFI.context_close(handle);
            return true;
        }
    }

    public class Context : IDisposable
    {
        private ContextHandler handler;

        public Context()
        {
            this.handler = FFI.context_create();
        }

        public void Dispose()
        {
            handler.Dispose();
        }

        public void SetString(String str)
        {
            FFI.context_set_string(this.handler, str);
        }

        public string GetString()
        {
            var str = FFI.context_get_string(this.handler);
            return str.AsString();
        }

        public void SetV2(V2 v2)
        {
            FFI.context_set_struct(this.handler, v2);
        }

        public V2 GetV2()
        {
            return FFI.context_get_struct(this.handler);
        }

        public void SetArray(byte[] bytes)
        {
            FFI.context_set_array(this.handler, bytes, Convert.ToUInt32(bytes.Length));
        }

        public byte[] GetArray()
        {
            byte[] bytes = null;

            FFI.context_get_array(this.handler, (ptr, length) =>
            {
                bytes = ToByteArray(ptr, length);
            });

            if (bytes == null)
            {
                throw new Exception("Null");
            }

            return bytes;
        }

        public void SetStructArray(V2[] array)
        {
            FFI.context_set_struct_array(this.handler, array, Convert.ToUInt32(array.Length));
        }

        public V2[] GetStructArray()
        {
            V2[] array = new V2[0] { };

            FFI.context_get_struct_array(this.handler, (ptr, length) =>
            {
                var size = Marshal.SizeOf<V2>();
                array = new V2[length];

                for (int i = 0; i < length; i++)
                {
                    V2 value = Marshal.PtrToStructure<V2>(ptr);
                    array[i] = value;
                    ptr += size;
                }
            });

            return array;
        }

        public void SetPeople(FFIPerson[] people)
        {
            FFI.context_set_people(this.handler, people, Convert.ToUInt32(people.Length));
        }

        public FFIPerson[] GetPeople()
        {
            FFIPerson[] array = null;

            FFI.context_get_people(this.handler, (ptr, length) =>
            {
                var size = Marshal.SizeOf<FFIPerson>();
                array = new FFIPerson[length];

                for (int i = 0; i < length; i++)
                {
                    FFIPerson value = Marshal.PtrToStructure<FFIPerson>(ptr);
                    array[i] = value;
                    ptr += size;
                }
            });

            if (array == null)
                throw new Exception();

            return array;
        }

        public void SetFlatBuffers(int[][] vectors)
        {
            var builder = new FlatBufferBuilder(1024);
            messages.Messages.StartInputVector(builder, vectors.Length);
            for (int i = 0; i < vectors.Length; i++)
            {
                messages.V2.CreateV2(builder, vectors[i][0], vectors[i][1]);
            }
            var vecs = builder.EndVector();

            messages.Messages.StartMessages(builder);
            messages.Messages.AddInput(builder, vecs);
            var msg = messages.Messages.EndMessages(builder);
            builder.Finish(msg.Value);

            // TODO: remove copy
            var bytes = builder.SizedByteArray();
            FFI.context_set_flatbuffer(this.handler, bytes, Convert.ToUInt32(bytes.Length));
        }

        public int[][] GetFlatBuffers()
        {
            int[][] result = null;

            FFI.context_get_flatbuffer(this.handler, (ptr, length) =>
            {
                // copy bytes
                var bytes = ToByteArray(ptr, length);

                // unmarshlar
                var buffer = new ByteBuffer(bytes);
                var msg = messages.Messages.GetRootAsMessages(buffer);

                result = new int[msg.OutputLength][];
                for (int i = 0; i < msg.OutputLength; i++)
                {
                    var v = msg.Output(i);
                    result[i] = new int[] {
                        v.Value.X, v.Value.Y
                    };
                }
            });

            return result;
        }

        private static byte[] ToByteArray(IntPtr ptr, uint length)
        {
            int len = Convert.ToInt32(length);
            var bytes = new byte[len];
            Marshal.Copy(ptr, bytes, 0, len);
            return bytes;
        }
    }
}
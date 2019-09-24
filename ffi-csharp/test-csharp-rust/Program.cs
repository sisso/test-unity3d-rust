using System.Collections;
using System.Collections.Generic;

using System.Runtime.InteropServices;
using System;

namespace testcsharprust
{
    static class FFI
    {
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern Int32 add_numbers(Int32 a, Int32 b);
            
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler context_create();
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern void context_close(IntPtr ptr);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool free_string(IntPtr ptr);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so")]
        internal static extern bool context_set_string(ContextHandler ptr, string str);
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern FFIStringHandler context_get_string(ContextHandler ptr);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so")]
        internal static extern bool context_set_struct(ContextHandler ptr, V2 v2);
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern V2 context_get_struct(ContextHandler ptr);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so")]
        internal static extern bool context_set_array(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_array(ContextHandler ptr, Action<IntPtr, UInt32> callback);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so")]
        internal static extern bool context_set_struct_array(ContextHandler ptr, V2[] buffer, UInt32 len);
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_struct_array(ContextHandler ptr, Action<IntPtr, UInt32> callback);

        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so")]
        internal static extern bool context_set_people(ContextHandler ptr, [In] FFIPerson[] buffer, UInt32 len);
        [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
        internal static extern bool context_get_people(ContextHandler ptr, Action<IntPtr, UInt32> callback);
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct FFIPerson
    {
        public UInt32 id;
        [MarshalAs(UnmanagedType.LPWStr)]
        public string name;
        public FFIAddress[] addresses;

        public override string ToString()
        {
            return string.Format("[FFIPerson: id={0}, name={1}, addresses={2}]", id, name, addresses);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct FFIAddress
    {
        [MarshalAs(UnmanagedType.LPWStr)]
        public string name;

        public override string ToString()
        {
            return string.Format("[FFIAddress: name={0}]", name);
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
            byte[] bytes = new byte[0] { };

            FFI.context_get_array(this.handler, (ptr, length) =>
            {
                bytes = new byte[length];

                for (int i = 0; i < length; i++)
                {
                    byte b = Marshal.ReadByte(ptr);
                    bytes[i] = b;
                    ptr += 1;
                }
            });

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
            FFIPerson[] array = new FFIPerson[0] { };

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

            return array;
        }
    }

    class MainClass
    {
        static void Assert(bool condition)
        {
            if (!condition)
                throw new Exception("Assertion fail");
        }

        // TODO: move to automatic tests
        static void AddNumberTest()
        {
            var result = FFI.add_numbers(1, 3);
            Console.WriteLine("AddNumberTest " + result);
            Assert(result == 4);
        }

        static void SendAndReceiveStringTest(Context context)
        {
            context.SetString("schönes");
            var str = context.GetString();

            Console.WriteLine("SendAndReceiveStringTest receive " + str);
            Assert(str == "schönes");
        }

        static void SendAndReceiveStructTest(Context context)
        {
            V2 v2 = new V2 { x = 9, y = 8 };
            context.SetV2(v2);

            var value = context.GetV2();

            Console.WriteLine("SendAndReceiveStructTest receive " + value);
            Assert(value.x == 9);
            Assert(value.y == 8);
        }

        static void SendAndReceiveArrayTest(Context context)
        {
            byte[] bytes = new byte[] { 3, 4, 5, 6, 7 };
            context.SetArray(bytes);

            var value = context.GetArray();
            var str = "[";
            for (int i = 0; i < value.Length; i++)
            {
                str += value[i] + (i == value.Length - 1 ? "" : ",");
            }
            str += "]";

            Console.WriteLine("SendAndReceiveArrayTest receive " + str);
            Assert(value.Length == 5);
            Assert(value[0] == 3);
            Assert(value[4] == 7);
        }

        static void SendAndReceiveStructArrayTest(Context context)
        {
            V2[] array = new V2[] { new V2 { x = 1, y = 2 }, new V2 { x = 3, y = 4 } };
            context.SetStructArray(array);

            var value = context.GetStructArray();
            var str = "[";
            for (int i = 0; i < value.Length; i++)
            {
                str += value[i] + (i == value.Length - 1 ? "" : ",");
            }
            str += "]";

            Console.WriteLine("SendAndReceiveArrayTest receive " + str);
            Assert(value.Length == 2);
            Assert(value[0].x == 1);
            Assert(value[1].y == 4);
        }

        static void SendAndReceivePeopleTest(Context context)
        {
            FFIPerson[] array = new FFIPerson[] {
                new FFIPerson { id = 0, name = "Roger", addresses = new FFIAddress[] {
                    new FFIAddress { name = "Street one" },
                }}
                //,
                //new FFIPerson { id = 1, name = "John", addresses = new FFIAddress[] {
                //    new FFIAddress { name = "Strasse" },
                //    new FFIAddress { name = "Street two" },
                //}},
            };
            context.SetPeople(array);

            var value = context.GetPeople();
            var str = "[";
            for (int i = 0; i < value.Length; i++)
            {
                str += value[i] + (i == value.Length - 1 ? "" : ",");
            }
            str += "]";

            Console.WriteLine("SendAndReceivePeopleTest receive " + str);
            Assert(value.Length == 2);
            Assert(value[0].id == 0);
            Assert(value[1].name == "John");
        }

        public static void Main (string[] args)
		{
            AddNumberTest();

            var context = new Context();
            SendAndReceiveStringTest(context);
            SendAndReceiveStructTest(context);
            SendAndReceiveArrayTest(context);
            SendAndReceiveStructArrayTest(context);
            SendAndReceivePeopleTest(context);

            Console.WriteLine("Done");
        }
    }
}

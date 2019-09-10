using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System.Runtime.InteropServices;
using System;

namespace Rust
{
    public interface ContextInput
    {
        //    int GetKindId();
        //    int GetMessageType();
        //    int GetRequestId();
    }

    public class MessageDefinition
    {
        private int messageId;
        private int messageKind;

        public MessageDefinition(int messageId, int messageKind)
        {
            this.messageId = messageId;
            this.messageKind = messageKind;
        }
    }


    public static class MessageKind
    {
    }

    public static class MessageType
    {
        public const int KIND_EMPTY = 0;
        public const int KIND_STRING = 1;

        private static MessageDefinition N(int messageId, int messageKind)
        {
            return new MessageDefinition(messageId, messageKind);
        }

        public static readonly MessageDefinition LOGIN = N(0, KIND_EMPTY);
        public static readonly MessageDefinition LOGOUT = N(1, KIND_EMPTY);
        public static readonly MessageDefinition SET_INPUT = N(2, KIND_STRING);
        public static readonly MessageDefinition REFRESH = N(3, KIND_EMPTY);
    }

    public class Login : ContextInput
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct V2
    {
        public Int32 x;
        public Int32 y;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct Buffer
    {
        public Int32 len;
        public IntPtr ptr;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct ByteBuffer
    {
        public Int32 len;
        public IntPtr ptr;
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
            Proxy.free_string(handle);
            return true;
        }
    }

    //class FfiString
    //{
    //    private FFIStringHandler handler;
    //    private string str;

    //    public FfiString(byte length)
    //    {
    //        song = Native.theme_song_generate(length);
    //    }

    //    public override string ToString()
    //    {
    //        if (songString == null)
    //        {
    //            songString = song.AsString();
    //        }
    //        return songString;
    //    }

    //    public void Dispose()
    //    {
    //        song.Dispose();
    //    }

    //    static public void Main()
    //    {
    //        var song = new ThemeSong(5);
    //        Console.WriteLine("{0}", song);
    //    }
    //}


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
            Proxy.context_close(handle);
            return true;
        }
    }

    public class Context : IDisposable
    {
        private ContextHandler handler;

        public Context()
        {
            this.handler = Proxy.context_create();
        }

        public void SetInput(int value)
        {
            Proxy.context_set_input(handler, value);
        }

        public int GetInput()
        {
            return Proxy.context_get_input(handler);
        }

        public void AddRequest(string str)
        {
            if (!Proxy.context_add_request(handler, str))
            {
                Debug.LogWarning("Failed returned from FFI method");
            }
        }

        public string GetResponses()
        {
            var handle = Proxy.context_get_responses(handler);
            var str = handle.AsString();
            return str;
        }

        public void Execute()
        {
            if (!Proxy.context_execute(handler))
            {
                Debug.LogWarning("Failed returned from FFI method");
            }
        }

        public void AddByteRequest(byte[] data)
        {
            IntPtr unmanagedArray = Marshal.AllocHGlobal(data.Length);
            try
            {
                Marshal.Copy(data, 0, unmanagedArray, data.Length);

                var buffer = new ByteBuffer();
                buffer.len = data.Length;
                buffer.ptr = unmanagedArray;
                Proxy.context_add_byte_request(handler, buffer);
            }
            finally
            {
                Marshal.FreeHGlobal(unmanagedArray);
            }
        }

        public byte[] GetByteRequest()
        {
            byte[] data = null;

            var result = Proxy.context_get_byte_responses(handler, (buffer) => {
                data = new byte[buffer.len];

                var pointer = buffer.ptr;
                for (int i = 0; i < buffer.len; i++)
                {
                    byte b = Marshal.ReadByte(pointer);
                    data[i] = b;
                    pointer += 1;
                }
            });

            if (result && data != null)
                return data;
            else
                return new byte[0];
        }

        public void Dispose()
        {
            handler.Dispose();
        }
    }

    public static class Proxy
    {
        [DllImport("librustlib")]
        private static extern V2 get_simple_struct();

        [DllImport("librustlib")]
        private static extern Buffer generate_data();

        [DllImport("librustlib")]
        private static extern void free_buf(Buffer buffer);

        [DllImport("librustlib")]
        internal static extern ContextHandler context_create();

        [DllImport("librustlib")]
        internal static extern void context_close(IntPtr ptr);

        [DllImport("librustlib")]
        internal static extern Int32 context_get_input(ContextHandler ptr);

        [DllImport("librustlib")]
        internal static extern bool context_set_input(ContextHandler ptr, Int32 value);

        [DllImport("librustlib")]
        internal static extern bool context_execute(ContextHandler ptr);
        [DllImport("librustlib")]
        internal static extern bool context_add_request(ContextHandler ptr, string value);
        [DllImport("librustlib")]
        internal static extern FFIStringHandler context_get_responses(ContextHandler ptr);
        [DllImport("librustlib")]
        internal static extern bool free_string(IntPtr ptr);

        [DllImport("librustlib")]
        internal static extern bool context_add_byte_request(ContextHandler ptr, ByteBuffer bytes);
        [DllImport("librustlib")]
        internal static extern bool context_get_byte_responses(ContextHandler ptr, Action<ByteBuffer> callback);

        public static V2 Get()
        {
            return get_simple_struct();
        }

        public static List<V2> GetBuffer()
        {
            var result = new List<V2>();
            int size = Marshal.SizeOf<V2>();
            var buffer = generate_data();
            try
            {
                var pointer = buffer.ptr;

                for (int i = 0; i < buffer.len; i++)
                {
                    var v2 = Marshal.PtrToStructure<V2>(pointer);
                    result.Add(v2);
                    pointer += size;
                }

            }
            finally
            {
                free_buf(buffer);
            }

            return result;
        }
    }

}
using System.Runtime.InteropServices;
using System;
using System.Collections.Generic;
using FlatBuffers;

namespace Server.Ffi
{
    static class Native
    {
        [DllImport("ffi_server.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler server_ffi_context_create();
        [DllImport("ffi_server.so", CharSet = CharSet.Unicode)]
        internal static extern void server_ffi_context_close(IntPtr ptr);
        [DllImport("ffi_server.so")]
        internal static extern bool server_ffi_push(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("ffi_server.so", CharSet = CharSet.Unicode)]
        internal static extern bool server_ffi_take(ContextHandler ptr, Action<IntPtr, UInt32> callback);
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
            Native.server_ffi_context_close(handle);
            return true;
        }
    }

    public class Context : IDisposable
    {
        private ContextHandler handler;

        public Context()
        {
            this.handler = Native.server_ffi_context_create();
        }

        public void Dispose()
        {
            handler.Dispose();
        }

        public void Send(byte[] bytes)
        {
            var result = Native.server_ffi_push(this.handler, bytes, Convert.ToUInt32(bytes.Length));
            if (!result)
            {
                throw new Exception($"Fail to send {bytes}");
            }
        }

        public List<byte[]> GetArray()
        {
            List<byte[]> bytes = new List<byte[]>();

            var result = Native.server_ffi_take(this.handler, (ptr, length) =>
            {
                bytes.Add(ToByteArray(ptr, length));
            });

            if (!result)
            {
                throw new Exception($"Fail to take");
            }
            
            return bytes;
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
﻿using System.Runtime.InteropServices;
using System;
using System.Collections.Generic;

namespace Domain.Ffi
{
    static class Native
    {
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler server_ffi_context_create();
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern void server_ffi_context_close(IntPtr ptr);
        [DllImport("ffi_domain.so")]
        internal static extern bool server_ffi_push(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
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

    ///
    /// FFI connector
    /// 
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

        public void Execute(Action<byte[]> caller)
        {
            var result = Native.server_ffi_take(this.handler, (ptr, length) =>
            {
                caller.Invoke(ToByteArray(ptr, length));
            });

            if (!result)
            {
                throw new Exception($"Fail to take");
            }
        }

        // TODO: remove copy 
        private static byte[] ToByteArray(IntPtr ptr, uint length)
        {
            int len = Convert.ToInt32(length);
            var bytes = new byte[len];
            Marshal.Copy(ptr, bytes, 0, len);
            return bytes;
        }
    }
}
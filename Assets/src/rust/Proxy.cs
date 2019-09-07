using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System.Runtime.InteropServices;
using System;

namespace Rust
{
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

        public void Dispose()
        {
            handler.Dispose();
        }
    }

    public static class Proxy
    {
        private static Context context = null;

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
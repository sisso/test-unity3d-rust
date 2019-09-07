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
            Proxy.context_close_2(handle);
            return true;
        }
    }

    public class Context : IDisposable
    {
        private ContextHandler handler;

        public Context()
        {
            this.handler = Proxy.context_create_2();
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
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        [DllImport("librustlib")]
        private static extern V2 get_simple_struct();

        [DllImport("librustlib")]
        private static extern Buffer generate_data();

        [DllImport("librustlib")]
        private static extern void free_buf(Buffer buffer);

        //[DllImport("librustlib")]
        //private static extern bool context_create(out IntPtr ptr);

        //[DllImport("librustlib")]
        //private static extern bool context_close(out IntPtr ptr);

        [DllImport("librustlib")]
        internal static extern ContextHandler context_create_2();

        [DllImport("librustlib")]
        internal static extern void context_close_2(IntPtr ptr);

        [DllImport("librustlib")]
        internal static extern Int32 context_get_input(ContextHandler ptr);

        [DllImport("librustlib")]
        internal static extern bool context_set_input(ContextHandler ptr, Int32 value);

        //public static Context GetContext()
        //{
        //    if (context != null)
        //        return context;

        //    //IntPtr ptr;
        //    //context_create(out ptr);

        //    IntPtr ptr = context_create_2();

        //    Debug.Log("Creating context: " + ptr.ToString());

        //    context = new Context(ptr);
        //    return context;
        //}

        //public static void CloseContext()
        //{
        //    if (context == null)
        //        return;

        //    Debug.Log("Closing context");

        //    context_close_2(out context.ptr);
        //    context = null;
        //}

        //public static void SetInput(Context ctx, int value)
        //{
        //    var result = context_set_input(out ctx.ptr, value);
        //    if (!result)
        //    {
        //        Debug.LogError("Failed to set value " + value + ".");
        //    }
        //}

        //public static int GetInput(Context ctx)
        //{
        //    return context_get_input(out ctx.ptr);
        //}

        public static int Sum(int a, int b)
        {
            return add_numbers(a, b);
        }

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
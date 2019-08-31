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

    public static class Proxy {
        [DllImport("librustlib")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        [DllImport("librustlib")]
        private static extern V2 get_simple_struct();

        [DllImport("librustlib")]
        private static extern Buffer generate_data();

        [DllImport("librustlib")]
        private static extern void free_buf(Buffer buffer);

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
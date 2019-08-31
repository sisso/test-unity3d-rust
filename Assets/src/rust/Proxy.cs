using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System.Runtime.InteropServices;
using System;

namespace Rust
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SharedStuff
    {
        public Int32 x;
        public Int32 y;
    }

    public static class Proxy {
        [DllImport("librustlib")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        [DllImport("librustlib")]
        private static extern SharedStuff get_simple_struct();

        public static int Sum(int a, int b)
        {
            return add_numbers(a, b);
        }

        public static SharedStuff Get()
        {
            return get_simple_struct();
        }
    }

}
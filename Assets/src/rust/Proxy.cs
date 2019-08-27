using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System.Runtime.InteropServices;
using System;

namespace Rust
{
    public static class Proxy {
        [DllImport("librustlib")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        public static int Sum(int a, int b) {
            return add_numbers(a,b);
        }
    }

}
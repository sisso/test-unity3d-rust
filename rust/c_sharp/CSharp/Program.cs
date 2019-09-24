using System;

using Rust;

namespace CSharp
{
    class MainClass
    {
        public static void Main(string[] args)
        {
            //Console.WriteLine("Receive: " + Rust.FFI.add_numbers(1, 2));
            Console.WriteLine("Press a key to continue.");
            Console.ReadLine();
            Console.WriteLine("Done");
        }
    }
}

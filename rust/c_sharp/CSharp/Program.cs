using System;

using Rust;

namespace CSharp
{
    class MainClass
    {
        public static void Main(string[] args)
        {
            var ctx = new Context();
            var bytes = ctx.GetByteRequest();
            Console.WriteLine("Receive bytes: " + bytes.Length);
            for (int i = 0; i < bytes.Length ; i++)
            {
                Console.WriteLine(" "+bytes[i]);
            }

            Console.WriteLine("Done");
        }
    }
}

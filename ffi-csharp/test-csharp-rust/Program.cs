using System;
using System.Runtime.InteropServices;

namespace testcsharprust
{
	class MainClass
	{
         [DllImport("/home/sisso/workspace/test-unity3d-rust/rust/target/debug/librustlib.so", CharSet = CharSet.Unicode)]
         internal static extern Int32 add_numbers(Int32 a, Int32 b);            

        public static void Main (string[] args)
		{
			var result = add_numbers(1, 3);
			Console.WriteLine("result: "+result);
		}
	}
}

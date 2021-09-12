
// Generated by flapigen. Do not edit.

// This warning occurs, because both Rust and C# have mehtod `ToString()`.
#pragma warning disable CS0114

using System;
using System.Runtime.InteropServices;

namespace ffi_domain_2
{
    internal static class RustString {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void c_string_delete(IntPtr c_char_ptr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* *mut RustString */ IntPtr c_str_u16_to_string(/* *const u16 */ IntPtr c_string_ptr);

        internal static string rust_to_dotnet(/* *const u16 */ IntPtr c_string_ptr)
        {
            var dotnet_str = Marshal.PtrToStringUni(c_string_ptr);
            RustString.c_string_delete(c_string_ptr);
            return dotnet_str;
        }

        internal static /* *mut RustString */ IntPtr dotnet_to_rust(string dotnet_str)
        {
            var c_string_ptr = Marshal.StringToHGlobalUni(dotnet_str);
            var rust_string_ptr = c_str_u16_to_string(c_string_ptr);
            Marshal.FreeHGlobal(c_string_ptr);
            return rust_string_ptr;
        }
    }

    [System.Serializable]
    public class Error : System.Exception
    {
        public Error(string message) : base(message) { }
    }

    ///  Test enum with A and B.
    public enum TestEnum {
        A = 0,B = 1
    }
    ///  Test class containing standard methods.
    public class TestClass: IDisposable {
        internal IntPtr nativePtr;

        internal TestClass(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                TestClass_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestClass_delete(IntPtr __this);

        ~TestClass() {
            DoDispose();
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestClass */ IntPtr TestClass_new();

        ///  Documentation for constructor
        public  TestClass () {
            
            this.nativePtr = TestClass_new();
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestClass_increment(/* TestClass */ IntPtr __this);

        ///  increment method documentation
        public  void Increment() {
            var __this_0 = this.nativePtr;

            TestClass_increment(__this_0);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestClass_print(/* TestClass */ IntPtr __this);

        
        public  void Print() {
            var __this_0 = this.nativePtr;

            TestClass_print(__this_0);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestClass_add(/* TestClass */ IntPtr __this, int i);

        
        public  void Add(int i_0) {
            var __this_0 = this.nativePtr;
var i_1 = i_0;
            TestClass_add(__this_0, i_1);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern int TestClass_get(/* TestClass */ IntPtr __this);

        
        public  int Get() {
            var __this_0 = this.nativePtr;

            var __ret_0 = TestClass_get(__this_0);
            var __ret_1 = __ret_0;
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestClass */ IntPtr TestClass_clone(/* TestClass */ IntPtr __this);

        
        public  TestClass Clone() {
            var __this_0 = this.nativePtr;

            var __ret_0 = TestClass_clone(__this_0);
            var __ret_1 = new TestClass(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte TestClass_eq(/* TestClass */ IntPtr __this, /* TestClass */ IntPtr other);

        
        public  bool Eq(/* ref */ TestClass other_0) {
            var __this_0 = this.nativePtr;
var other_1 = other_0.nativePtr;
            var __ret_0 = TestClass_eq(__this_0, other_1);
            var __ret_1 = (__ret_0 != 0);
            return __ret_1;
        }
} // class

    
    public class TemplateI32: IDisposable {
        internal IntPtr nativePtr;

        internal TemplateI32(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                TemplateI32_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TemplateI32_delete(IntPtr __this);

        ~TemplateI32() {
            DoDispose();
        }
} // class
///  Test class containing static methods only.
public static class TestStaticClass {

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestStaticClass_hello();

        ///  Documentation for `TestStaticClass::hello`()
        public static void Hello() {
            
            TestStaticClass_hello();
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestStaticClass_print_number(int n);

        
        public static void PrintNumber(int n_0) {
            var n_1 = n_0;
            TestStaticClass_print_number(n_1);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern int TestStaticClass_add(int a, int b);

        
        public static int Add(int a_0, int b_0) {
            var a_1 = a_0;
            var b_1 = b_0;
            var __ret_0 = TestStaticClass_add(a_1, b_1);
            var __ret_1 = __ret_0;
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestStaticClass_concat(/* RustString */ IntPtr a, /* RustString */ IntPtr b);

        
        public static string Concat(string a_0, string b_0) {
            var a_1 = RustString.dotnet_to_rust(a_0);
            var b_1 = RustString.dotnet_to_rust(b_0);
            var __ret_0 = TestStaticClass_concat(a_1, b_1);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestStaticClass_concat_str(/* RustString */ IntPtr a, /* RustString */ IntPtr b);

        
        public static string ConcatStr(string a_0, string b_0) {
            var a_1 = RustString.dotnet_to_rust(a_0);
            var b_1 = RustString.dotnet_to_rust(b_0);
            var __ret_0 = TestStaticClass_concat_str(a_1, b_1);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestStaticClass_print_vec_len(/* Option */ IntPtr vec);

        
        public static void PrintVecLen(System.Collections.Generic.List<int> vec_0) {
            var vec_1 = RustVecint.dotnet_to_rust(vec_0);
            TestStaticClass_print_vec_len(vec_1);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestStaticClass_print_slice_len(/* Option */ IntPtr vec);

        
        public static void PrintSliceLen(/* Slice */ System.Collections.Generic.List<int> vec_0) {
            var vec_1 = RustVecint.dotnet_to_rust(vec_0);
            TestStaticClass_print_slice_len(vec_1);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_get_vec();

        
        public static System.Collections.Generic.List<int> GetVec() {
            
            var __ret_0 = TestStaticClass_get_vec();
            var __ret_1 = RustVecint.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_maybe_return_class(/* Option */ IntPtr str);

        
        public static Option<TestClass> MaybeReturnClass(Option<string> str_0) {
            var str_1 = RustOptionstring.dotnet_to_rust(str_0);
            var __ret_0 = TestStaticClass_maybe_return_class(str_1);
            var __ret_1 = RustOptionTestClass.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestClass */ IntPtr TestStaticClass_return_class_ref(/* TestClass */ IntPtr obj);

        
        public static /* ref */ TestClass ReturnClassRef(/* ref */ TestClass obj_0) {
            var obj_1 = obj_0.nativePtr;
            var __ret_0 = TestStaticClass_return_class_ref(obj_1);
            var __ret_1 = new TestClass(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_maybe_add_one(/* Option */ IntPtr i);

        
        public static Option<int> MaybeAddOne(Option<int> i_0) {
            var i_1 = RustOptionint.dotnet_to_rust(i_0);
            var __ret_0 = TestStaticClass_maybe_add_one(i_1);
            var __ret_1 = RustOptionint.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_maybe_return_template();

        
        public static Option<TemplateI32> MaybeReturnTemplate() {
            
            var __ret_0 = TestStaticClass_maybe_return_template();
            var __ret_1 = RustOptionTemplateI32.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestStaticClass_test_obj_by_value(/* TestClass */ IntPtr obj);

        
        public static void TestObjByValue(TestClass obj_0) {
            var obj_1 = obj_0.nativePtr;
            TestStaticClass_test_obj_by_value(obj_1);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_try_create_object_template();

        
        public static /* Result<Template < i32 >, TestError> */ TemplateI32 TryCreateObjectTemplate() {
            
            var __ret_0 = TestStaticClass_try_create_object_template();
            var __ret_1 = RustResultTemplateI32.unwrap(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_try_empty_result();

        
        public static /* ResultVoid<TestError> */ void TryEmptyResult() {
            
            var __ret_0 = TestStaticClass_try_empty_result();
            RustResultVoid.unwrap(__ret_0);
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_try_create_object_ok();

        
        public static /* Result<TestClass, TestError> */ TestClass TryCreateObjectOk() {
            
            var __ret_0 = TestStaticClass_try_create_object_ok();
            var __ret_1 = RustResultTestClass.unwrap(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_try_create_object_err();

        
        public static /* Result<TestClass, TestError> */ TestClass TryCreateObjectErr() {
            
            var __ret_0 = TestStaticClass_try_create_object_err();
            var __ret_1 = RustResultTestClass.unwrap(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern uint TestStaticClass_reverse_enum(uint e);

        
        public static TestEnum ReverseEnum(TestEnum e_0) {
            var e_1 = (uint)e_0;
            var __ret_0 = TestStaticClass_reverse_enum(e_1);
            var __ret_1 = (TestEnum)__ret_0;
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option */ IntPtr TestStaticClass_get_tuple();

        
        public static Tuple<int, string> GetTuple() {
            
            var __ret_0 = TestStaticClass_get_tuple();
            var __ret_1 = RustTuple2Tintstring.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class TestArc: IDisposable {
        internal IntPtr nativePtr;

        internal TestArc(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                TestArc_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestArc_delete(IntPtr __this);

        ~TestArc() {
            DoDispose();
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestArc */ IntPtr TestArc_new();

        
        public  TestArc () {
            
            this.nativePtr = TestArc_new();
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestArc_to_string(/* TestArc */ IntPtr __this);

        
        public  string ToString() {
            var __this_0 = this.nativePtr;

            var __ret_0 = TestArc_to_string(__this_0);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestArc_to_string_arc(/* TestArc */ IntPtr a0);

        
        public static string ToStringArc(TestArc a0_0) {
            var a0_1 = a0_0.nativePtr;
            var __ret_0 = TestArc_to_string_arc(a0_1);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestArc_to_string_ref_arc(/* TestArc */ IntPtr a0);

        
        public static string ToStringRefArc(/* ref */ TestArc a0_0) {
            var a0_1 = a0_0.nativePtr;
            var __ret_0 = TestArc_to_string_ref_arc(a0_1);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class TestArcMutex: IDisposable {
        internal IntPtr nativePtr;

        internal TestArcMutex(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                TestArcMutex_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestArcMutex_delete(IntPtr __this);

        ~TestArcMutex() {
            DoDispose();
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestArcMutex */ IntPtr TestArcMutex_new();

        
        public  TestArcMutex () {
            
            this.nativePtr = TestArcMutex_new();
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestArcMutex_to_string(/* TestArcMutex */ IntPtr __this);

        
        public  string ToString() {
            var __this_0 = this.nativePtr;

            var __ret_0 = TestArcMutex_to_string(__this_0);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestArcMutex_inc(/* TestArcMutex */ IntPtr __this);

        
        public  void Inc() {
            var __this_0 = this.nativePtr;

            TestArcMutex_inc(__this_0);
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestArcMutex_to_string_arc(/* TestArcMutex */ IntPtr a0);

        
        public static string ToStringArc(TestArcMutex a0_0) {
            var a0_1 = a0_0.nativePtr;
            var __ret_0 = TestArcMutex_to_string_arc(a0_1);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    
    public class TestBox: IDisposable {
        internal IntPtr nativePtr;

        internal TestBox(IntPtr nativePtr) {
            this.nativePtr = nativePtr;
        }

        public void Dispose() {
            DoDispose();
            GC.SuppressFinalize(this);
        }

        private void DoDispose() {
            if (nativePtr != IntPtr.Zero) {
                TestBox_delete(nativePtr);
                nativePtr = IntPtr.Zero;
            }
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void TestBox_delete(IntPtr __this);

        ~TestBox() {
            DoDispose();
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestBox */ IntPtr TestBox_new();

        
        public  TestBox () {
            
            this.nativePtr = TestBox_new();
            
            
        }

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr TestBox_to_string(/* TestBox */ IntPtr __this);

        
        public  string ToString() {
            var __this_0 = this.nativePtr;

            var __ret_0 = TestBox_to_string(__this_0);
            var __ret_1 = RustString.rust_to_dotnet(__ret_0);
            return __ret_1;
        }
} // class

    public static class RustVecint {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustVecint_new();
        
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecint_push(IntPtr vecPtr, int element);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Option<i_type> */ IntPtr RustVecint_iter_next(IntPtr iterPtr);
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustVecint_iter_delete(IntPtr iterPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern int RustVecint_option_take(IntPtr optPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustVecint_option_is_some(IntPtr optPtr);


        internal static System.Collections.Generic.List<int> rust_to_dotnet(IntPtr iterPtr) {
            var list = new System.Collections.Generic.List<int>();
            while (true)
            {
                var next_rust_opt = RustVecint.RustVecint_iter_next(iterPtr);
                if (RustVecint_option_is_some(next_rust_opt) == 0)
                {
                    break;
                }
                var value_rust = RustVecint_option_take(next_rust_opt);
                var value = value_rust;
                list.Add(value);
            }
            RustVecint_iter_delete(iterPtr);
            return list;
        }

        internal static IntPtr dotnet_to_rust(System.Collections.Generic.List<int> list) {
            var vec = RustVecint_new();
            foreach (var element in list)
            {
                var i_element = element;
                RustVecint.RustVecint_push(vec, i_element);
            }
            return vec;
        }
    }
        
    internal static class RustOptionint {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionint_new_none();

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionint_new_some(int value);
        
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern int RustOptionint_take(IntPtr optPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustOptionint_is_some(IntPtr optPtr);

        internal static Option<int> rust_to_dotnet(IntPtr optPtr)
        {
            if (RustOptionint_is_some(optPtr) != 0)
            {
                var value_0 = RustOptionint_take(optPtr);
                var value_1 = value_0;
                return new Option<int>(value_1);
            }
            else
            {
                return new Option<int>();
            }
        }

        internal static IntPtr dotnet_to_rust(Option<int> opt)
        {
            if (opt.IsSome)
            {
                var value_0 = opt.Value;
                return RustOptionint_new_some(value_0);
            }
            else
            {
                return RustOptionint_new_none();
            }
        }
    }
    
    internal static class RustResultTestClass {

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustResultTestClass_is_ok(IntPtr resultPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestClass */ IntPtr RustResultTestClass_take_ok(IntPtr resultPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* mut c_str_u16 */ IntPtr RustResultTestClass_take_error(IntPtr resultPtr);

        internal static TestClass unwrap(IntPtr resultPtr)
        {
            if (RustResultTestClass_is_ok(resultPtr) != 0)
            {
                var value_0 = RustResultTestClass_take_ok(resultPtr);
                var value_1 = new TestClass(value_0);
                return value_1;
            }
            else
            {
                var messagePtr = RustResultTestClass_take_error(resultPtr);
                var message = RustString.rust_to_dotnet(messagePtr);
                throw new Error(message);
            }
        }
    }
    
    internal static class RustTuple2Tintstring {

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* Tuple */ IntPtr RustTuple2Tintstring_new(int t_1, /* const c_str_u16 */ IntPtr t_2);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern int RustTuple2Tintstring_take_1(IntPtr tuple);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* const c_str_u16 */ IntPtr RustTuple2Tintstring_take_2(IntPtr tuple);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void RustTuple2Tintstring_delete(IntPtr tuple);

        internal static Tuple<int, string> rust_to_dotnet(IntPtr rustTuple)
        {
            var t_1_rust = RustTuple2Tintstring_take_1(rustTuple);
            var t_1 = t_1_rust;
            var t_2_rust = RustTuple2Tintstring_take_2(rustTuple);
            var t_2 = RustString.rust_to_dotnet(t_2_rust);
            var ret = Tuple.Create(t_1, t_2);
            RustTuple2Tintstring_delete(rustTuple);
            return ret;
        }
        internal static /* Tuple */ IntPtr dotnet_to_rust(Tuple<int, string> tuple)
        {
            var t_1 = tuple.Item1;
            var t_1_rust = t_1;
            var t_2 = tuple.Item2;
            var t_2_rust = RustString.dotnet_to_rust(t_2);
            // We don't call delete in `Input` scenario. Rust-side conversion code will take care of it.
            return RustTuple2Tintstring_new(t_1_rust, t_2_rust);            
        }
    }
    
    internal static class RustResultVoid {

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustResultVoid_is_ok(IntPtr resultPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* mut c_str_u16 */ IntPtr RustResultVoid_take_err(IntPtr resultPtr);

        internal static void unwrap(IntPtr resultPtr)
        {
            if (RustResultVoid_is_ok(resultPtr) != 0)
            {
                return;
            }
            else
            {
                var messagePtr = RustResultVoid_take_err(resultPtr);
                var message = RustString.rust_to_dotnet(messagePtr);
                throw new Error(message);
            }
        }
    }
    

        public class Option<T> {
        
            [System.Serializable]
            public class OptionNoneException : System.Exception
            {
                public OptionNoneException() :
                    base("Trying to get the value of an `Option` that is `None`") 
                {
                }
            }
        
            private T value;
            private bool isSome;
        
            public bool IsSome
            {
                get
                {
                    return isSome;
                }
            }
        
            public T Value
            {
                get {
                    if (!isSome) {
                        throw new OptionNoneException();
                    }
                    return value;
                }
            }
        
            public Option()
            {
                value = default(T);
                isSome = false;
            }
        
            public Option(T value)
            {
                if (value == null) 
                {
                    this.value = value;
                    this.isSome = false;
                }
                else
                {
                    this.value = value;
                    this.isSome = true;
                }
            }
        }        
        
    internal static class RustResultTemplateI32 {

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustResultTemplateI32_is_ok(IntPtr resultPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TemplateI32 */ IntPtr RustResultTemplateI32_take_ok(IntPtr resultPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* mut c_str_u16 */ IntPtr RustResultTemplateI32_take_error(IntPtr resultPtr);

        internal static TemplateI32 unwrap(IntPtr resultPtr)
        {
            if (RustResultTemplateI32_is_ok(resultPtr) != 0)
            {
                var value_0 = RustResultTemplateI32_take_ok(resultPtr);
                var value_1 = new TemplateI32(value_0);
                return value_1;
            }
            else
            {
                var messagePtr = RustResultTemplateI32_take_error(resultPtr);
                var message = RustString.rust_to_dotnet(messagePtr);
                throw new Error(message);
            }
        }
    }
    
    internal static class RustOptionTemplateI32 {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionTemplateI32_new_none();

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionTemplateI32_new_some(/* TemplateI32 */ IntPtr value);
        
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TemplateI32 */ IntPtr RustOptionTemplateI32_take(IntPtr optPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustOptionTemplateI32_is_some(IntPtr optPtr);

        internal static Option<TemplateI32> rust_to_dotnet(IntPtr optPtr)
        {
            if (RustOptionTemplateI32_is_some(optPtr) != 0)
            {
                var value_0 = RustOptionTemplateI32_take(optPtr);
                var value_1 = new TemplateI32(value_0);
                return new Option<TemplateI32>(value_1);
            }
            else
            {
                return new Option<TemplateI32>();
            }
        }

        internal static IntPtr dotnet_to_rust(Option<TemplateI32> opt)
        {
            if (opt.IsSome)
            {
                var value_0 = opt.Value.nativePtr;
                return RustOptionTemplateI32_new_some(value_0);
            }
            else
            {
                return RustOptionTemplateI32_new_none();
            }
        }
    }
    
    internal static class RustOptionTestClass {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionTestClass_new_none();

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionTestClass_new_some(/* TestClass */ IntPtr value);
        
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* TestClass */ IntPtr RustOptionTestClass_take(IntPtr optPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustOptionTestClass_is_some(IntPtr optPtr);

        internal static Option<TestClass> rust_to_dotnet(IntPtr optPtr)
        {
            if (RustOptionTestClass_is_some(optPtr) != 0)
            {
                var value_0 = RustOptionTestClass_take(optPtr);
                var value_1 = new TestClass(value_0);
                return new Option<TestClass>(value_1);
            }
            else
            {
                return new Option<TestClass>();
            }
        }

        internal static IntPtr dotnet_to_rust(Option<TestClass> opt)
        {
            if (opt.IsSome)
            {
                var value_0 = opt.Value.nativePtr;
                return RustOptionTestClass_new_some(value_0);
            }
            else
            {
                return RustOptionTestClass_new_none();
            }
        }
    }
    
    internal static class RustOptionstring {
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionstring_new_none();

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr RustOptionstring_new_some(/* RustString */ IntPtr value);
        
        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern /* RustString */ IntPtr RustOptionstring_take(IntPtr optPtr);

        [DllImport("ffi_domain_2_native", CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte RustOptionstring_is_some(IntPtr optPtr);

        internal static Option<string> rust_to_dotnet(IntPtr optPtr)
        {
            if (RustOptionstring_is_some(optPtr) != 0)
            {
                var value_0 = RustOptionstring_take(optPtr);
                var value_1 = RustString.rust_to_dotnet(value_0);
                return new Option<string>(value_1);
            }
            else
            {
                return new Option<string>();
            }
        }

        internal static IntPtr dotnet_to_rust(Option<string> opt)
        {
            if (opt.IsSome)
            {
                var value_0 = RustString.dotnet_to_rust(opt.Value);
                return RustOptionstring_new_some(value_0);
            }
            else
            {
                return RustOptionstring_new_none();
            }
        }
    }
    } // namespace
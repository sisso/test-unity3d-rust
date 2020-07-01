#define NO_STATIC_BIND

using System.Runtime.InteropServices;
using System;
using System.Collections.Generic;
using UnityEngine;


namespace Domain.Ffi
{
    #if STATIC_BIND
    static class Native
    {
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler ffi_context_create_embedded();
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern ContextHandler ffi_context_connect(string address);
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern void ffi_context_close(IntPtr ptr);
        [DllImport("ffi_domain.so")]
        internal static extern bool ffi_context_push(ContextHandler ptr, byte[] buffer, UInt32 len);
        [DllImport("ffi_domain.so", CharSet = CharSet.Unicode)]
        internal static extern bool ffi_context_take(ContextHandler ptr, Action<IntPtr, UInt32> callback);
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
            Native.ffi_context_close(handle);
            return true;
        }
    }
    #else

    #endif
    
    ///
    /// FFI connector
    /// 
    /// https://jacksondunstan.com/articles/3945
    public class Context : IDisposable
    {
        #if STATIC_BIND
        private ContextHandler handler;
        #else
        
        [DllImport("__Internal")]
        public static extern IntPtr dlopen(
            string path,
            int flag);
        
        [DllImport("__Internal")]
        public static extern IntPtr dlerror();
 
        [DllImport("__Internal")]
        public static extern IntPtr dlsym(
            IntPtr handle,
            string symbolName);
 
        [DllImport("__Internal")]
        public static extern int dlclose(
            IntPtr handle);

        public static IntPtr LoadLibrary(string path)
        {
            Debug.Log($"Loading native library at {path}");
            IntPtr handle = dlopen(path, 2); // 1 lazy, 2 now, 0 ??
            if (handle == IntPtr.Zero)
            {
                var error = GetDLError();
                throw new Exception("Couldn't open native library at [" + path + "]: " + error);
            }

            return handle;
        }

        private static string GetDLError()
        {
            var errPtr = dlerror();

            if (errPtr == IntPtr.Zero)
            {
                return "Null pointer was returned from dlerror.";
            }
            else
            {
                return Marshal.PtrToStringAnsi(errPtr);
            }
        }

        public static void CloseLibrary(IntPtr libraryHandle)
        {
            if (libraryHandle != IntPtr.Zero)
            {
                Debug.Log("Closing native library");
                dlclose(libraryHandle);
            }
        } 
        
        public static T GetLibraryFunction<T>(
            IntPtr libraryHandle,
            string functionName) where T : class
        {
        
            Debug.Log("Load native function "+functionName);
            
            IntPtr symbol = dlsym(libraryHandle, functionName);
            if (symbol == IntPtr.Zero)
            {
                var error = GetDLError();
                throw new Exception("Couldn't get function: " + functionName + ": "+ error);
            }
            return Marshal.GetDelegateForFunctionPointer(
                symbol,
                typeof(T)) as T;
        }
        
        delegate IntPtr CreateContext();
        delegate void CloseContext(IntPtr contextHandle);
        delegate bool ContextPush(IntPtr contextHandle, byte[] buffer, UInt32 len);
        delegate bool ContextTake(IntPtr ptr, Action<IntPtr, UInt32> callback);

        private CloseContext nativeCloseContext;
        private ContextPush nativeContextPush;
        private ContextTake nativeContextTake;
        
        private IntPtr libraryHandle;
        private IntPtr contextHandle;
        
        #endif

        public Context(string address)
        {
        #if STATIC_BIND
            if (address == null || address.Length == 0)
            {
                this.handler = Native.ffi_context_create_embedded();
            }
            else
            {
                this.handler = Native.ffi_context_connect(address);
            }
        #else
            // load library
            var libName = "libffi_domain.so";
            var dataPath = Application.dataPath + "/Plugins/" + libName;
            libraryHandle = LoadLibrary(dataPath);

            // load methods
            this.nativeCloseContext = GetLibraryFunction<CloseContext>(libraryHandle, "ffi_context_close");
            this.nativeContextPush = GetLibraryFunction<ContextPush>(libraryHandle, "ffi_context_push");
            this.nativeContextTake = GetLibraryFunction<ContextTake>(libraryHandle, "ffi_context_take");
            
            // start ffi context
            CreateContext createContext = GetLibraryFunction<CreateContext>(libraryHandle, "ffi_context_create_embedded");
            contextHandle = createContext.Invoke();
#endif
        }

        public void Dispose()
        {
        #if STATIC_BIND
            handler.Dispose();
        #else
            // close ffi context
            if (contextHandle != IntPtr.Zero && this.nativeCloseContext != null)
            {
                this.nativeCloseContext.Invoke(contextHandle);
                contextHandle = IntPtr.Zero;
            }

            // unload library
            CloseLibrary(libraryHandle);
            libraryHandle = IntPtr.Zero;
#endif
        }

        public void Send(byte[] bytes)
        {
        #if STATIC_BIND
            var result = Native.ffi_context_push(this.handler, bytes, Convert.ToUInt32(bytes.Length));
        #else
            var result = nativeContextPush(this.contextHandle, bytes, Convert.ToUInt32(bytes.Length));
        #endif
            if (!result)
            {
                throw new Exception($"Fail to send {bytes}");
            }
        }

        public void Execute(Action<byte[]> caller)
        {
        #if STATIC_BIND
            var result = Native.ffi_context_take(this.handler, (ptr, length) =>
            {
                caller.Invoke(ToByteArray(ptr, length));
            });
        #else
            var result = nativeContextTake.Invoke(this.contextHandle, (ptr, length) =>
            {
                caller.Invoke(ToByteArray(ptr, length));
            });
        #endif
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
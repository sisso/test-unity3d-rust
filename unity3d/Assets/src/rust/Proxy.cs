using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System.Runtime.InteropServices;
using System;

namespace Rust
{
    public interface ContextInput
    {
        //    int GetKindId();
        //    int GetMessageType();
        //    int GetRequestId();
    }

    public class MessageDefinition
    {
        private int messageId;
        private int messageKind;

        public MessageDefinition(int messageId, int messageKind)
        {
            this.messageId = messageId;
            this.messageKind = messageKind;
        }
    }


    public static class MessageKind
    {
    }

    public static class MessageType
    {
        public const int KIND_EMPTY = 0;
        public const int KIND_STRING = 1;

        private static MessageDefinition N(int messageId, int messageKind)
        {
            return new MessageDefinition(messageId, messageKind);
        }

        public static readonly MessageDefinition LOGIN = N(0, KIND_EMPTY);
        public static readonly MessageDefinition LOGOUT = N(1, KIND_EMPTY);
        public static readonly MessageDefinition SET_INPUT = N(2, KIND_STRING);
        public static readonly MessageDefinition REFRESH = N(3, KIND_EMPTY);
    }

    public class Login : ContextInput
    {
    }

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

    [StructLayout(LayoutKind.Sequential)]
    struct ByteBuffer
    {
        public Int32 len;
        public IntPtr ptr;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct EntityComponent
    {
        public IntPtr label;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct Entity
    {
        public UInt32 id;
        public V2 pos;
        public UInt32 kind;
        public IntPtr components;
        public UInt32 components_length;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct OutputMessages
    {
        public IntPtr new_entities;
        public UInt32 new_entities_length;
        public IntPtr removed_entities;
        public UInt32 removed_entities_length;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct FFIArray
    {
        public IntPtr ptr;
        public UInt32 len;
    }

    internal class FFIStringHandler : SafeHandle
    {
        public FFIStringHandler() : base(IntPtr.Zero, true)
        {
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return System.Text.Encoding.UTF8.GetString(buffer);
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
            Proxy.free_string(handle);
            return true;
        }
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

        public void AddRequest(string str)
        {
            if (!Proxy.context_add_request(handler, str))
            {
                Debug.LogWarning("Failed returned from FFI method");
            }
        }

        public string GetResponses()
        {
            var handle = Proxy.context_get_responses(handler);
            var str = handle.AsString();
            return str;
        }

        public void Execute()
        {
            if (!Proxy.context_execute(handler))
            {
                Debug.LogWarning("Failed returned from FFI method");
            }
        }

        public void AddByteRequest(byte[] data)
        {
            Proxy.context_add_byte_request(handler, data, data.Length);

            //GCHandle handle = GCHandle.Alloc(data, GCHandleType.Pinned);
            //try
            //{
            //    IntPtr ptr = GCHandle.ToIntPtr(handle);
            //    //var buffer = new ByteBuffer();
            //    //buffer.len = data.Length;
            //    //buffer.ptr = ptr;
            //    Proxy.context_add_byte_request(handler, ptr, data.Length);
            //}
            //finally
            //{
            //    handle.Free();
            //}

            //IntPtr unmanagedArray = Marshal.AllocHGlobal(data.Length);
            //try
            //{
            //    Marshal.Copy(data, 0, unmanagedArray, data.Length);

            //    var buffer = new ByteBuffer();
            //    buffer.len = data.Length;
            //    buffer.ptr = unmanagedArray;
            //    Proxy.context_add_byte_request(handler, buffer);
            //}
            //finally
            //{
            //    Marshal.FreeHGlobal(unmanagedArray);
            //}
        }

        public byte[] GetByteRequest()
        {
            byte[] data = null;

            var result = Proxy.context_get_byte_responses(handler, (buffer, len) => {
                data = new byte[len];

                var pointer = buffer;
                for (int i = 0; i < len; i++)
                {
                    byte b = Marshal.ReadByte(pointer);
                    data[i] = b;
                    pointer += 1;
                }
            });

            if (result && data != null)
                return data;
            else
                return new byte[0];
        }

        public void GetMessages()
        {
            Proxy.context_get_output_messages(handler, (messages) =>
            {
                Debug.Log("GetMessages new entities length: " + messages.new_entities_length);
                Debug.Log("GetMessages removed entities length: " + messages.removed_entities_length);
            });
        }

        public List<UInt32> GetDeletedEntities()
        {
            var list = new List<UInt32>();

            Proxy.context_get_removed_entities(handler, (array) =>
            {
                int size = Marshal.SizeOf<UInt32>();
                var pointer = array.ptr;

                for (int i = 0; i < array.len; i++)
                {
                    var v = Marshal.PtrToStructure<UInt32>(pointer);
                    list.Add(v);
                    pointer += size;
                }
            });

            return list;
        }


        public List<String> GetNewEntities()
        {
            var list = new List<String>();

            Proxy.context_get_new_entities(handler, (array) =>
            {
                int size_entity = Marshal.SizeOf<Entity>();
                int size_component = Marshal.SizeOf<EntityComponent>();
                var pointer = array.ptr;

                for (int i = 0; i < array.len; i++)
                {
                    var v = Marshal.PtrToStructure<Entity>(pointer);
                    Debug.Log("GetNewEntities - " + v.id + "/" + v.kind + " (" + v.pos.x + ", "+ v.pos.y + ")");
                    Debug.Log(" - components length: " + v.components_length);
                    var pointer_components = v.components;
                    for (int j = 0; j < v.components_length ; j++)
                    {
                        var c = Marshal.PtrToStructure<EntityComponent>(pointer_components);
                        var str = Marshal.PtrToStringAuto(c.label);
                        Debug.Log("   - " + str);
                        pointer_components += size_component;
                    }
                    pointer += size_entity;
                }
            });

            return list;
        }


        public void Dispose()
        {
            handler.Dispose();
        }
    }

    public static class Proxy
    {
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

        [DllImport("librustlib")]
        internal static extern bool context_execute(ContextHandler ptr);
        [DllImport("librustlib")]
        internal static extern bool context_add_request(ContextHandler ptr, string value);
        [DllImport("librustlib")]
        internal static extern FFIStringHandler context_get_responses(ContextHandler ptr);
        [DllImport("librustlib")]
        internal static extern bool free_string(IntPtr ptr);

        [DllImport("librustlib")]
        internal static extern bool context_add_byte_request(ContextHandler ptr, byte[] bytes, Int32 length);
        [DllImport("librustlib")]
        internal static extern bool context_get_byte_responses(ContextHandler ptr, Action<IntPtr, Int32> callback);

        [DllImport("librustlib")]
        internal static extern bool context_get_output_messages(ContextHandler ptr, Action<OutputMessages> callback);

        [DllImport("librustlib")]
        internal static extern bool context_get_new_entities(ContextHandler ptr, Action<FFIArray> callback);

        [DllImport("librustlib")]
        internal static extern bool context_get_removed_entities(ContextHandler ptr, Action<FFIArray> callback);

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
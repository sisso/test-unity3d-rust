using System;
using System.Collections.Generic;

namespace Server
{
    /// <summary>
    /// Basic network message
    /// </summary>
    public class Msg
    {
        public server.PackageKind kind;
        public byte[] body;
    }
    
    /// <summary>
    ///  Definition of server, can have a remote tcp/udp or a embedded local ffi server
    /// </summary>
    public interface IServer : IDisposable
    {
        List<Msg> Take();
        void Send(Msg msg);
    }
}
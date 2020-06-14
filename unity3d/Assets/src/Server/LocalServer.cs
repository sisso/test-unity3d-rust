using System;
using System.Collections.Generic;

namespace Server
{
    /// <summary>
    /// Implementation of local server through ffi
    /// </summary>
    public class LocalServer : IServer, IDisposable 
    {
        private Server.Ffi.Context context;
        
        public LocalServer()
        {
            this.context = new Server.Ffi.Context();
        }

        public void Dispose()
        {
            this.context.Dispose();
            this.context = null;
        }

        public List<Msg> Take()
        {
            throw new NotImplementedException();
        }

        public void Send(Msg msg)
        {
            throw new NotImplementedException();
        }
    }
}
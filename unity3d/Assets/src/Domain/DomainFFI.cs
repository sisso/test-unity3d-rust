using System;
using System.Collections.Generic;
using FlatBuffers;
using UnityEngine;

namespace Domain
{
    /// <summary>
    /// Game logic implemented by a Server
    /// </summary>
    public class DomainFFI : MonoBehaviour, IDomain
    {
        private Ffi.Context ffi;

        void OnDestroy()
        {
            Debug.Log("Closing context");
            ffi.Dispose();
            ffi = null;
        }

        public void StartLocalServer() {
            ffi = new Ffi.Context();
        }

        public void ConnectToServer(string remoteAddress)
        {
            ffi = new Ffi.Context();
        }

        public List<IResponse> Execute()
        {
            if (ffi == null)
            {
                Debug.LogWarning("FFI was not started");
                return new List<IResponse>();
            }
            
            List<IResponse> result = new List<IResponse>();
            
            ffi.Execute(bytes =>
            {
                var buffer = new ByteBuffer(bytes);
                var response = FfiResponses.Responses.GetRootAsResponses(buffer);

                for (int i = 0; i < response.SimpleLength ; i++)
                {
                    var kind = response.Simple(i)?.Kind;
                    switch (kind)
                    {
                        case FfiResponses.ResponseKind.StartGame:
                            result.Add(new ResponseStartGame());
                            break;
                        default:
                            Debug.LogWarning($"Unknown kind [${kind}] for simple package");
                            break;
                    }
                }

                for (int i = 0; i < response.CreateObjectLength  ; i++)
                {
                    var package= response.CreateObject(i).Value;

                    result.Add(new ResponseSpawn() { id = package.Id, position = new Vector3(package.X, package.Y, 0f) });
                }

                for (int i = 0; i < response.MoveObjLength; i++)
                {
                    var package= response.MoveObj(i).Value;

                    result.Add(new ResponsePos() { id = package.Id, position = new Vector3(package.X, package.Y, 0f) });
                }
            });
            
            return result;
        }
    }
}
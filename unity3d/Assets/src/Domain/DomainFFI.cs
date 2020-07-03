using System;
using System.Collections.Generic;
using System.Linq;
using FfiRequests;
using FfiResponses;
using FlatBuffers;
using UnityEditor.Graphs;
using UnityEngine;
using UnityEngine.Networking.Match;

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
            if (ffi != null)
            {
                Debug.Log("Closing ffi context");
                ffi.Dispose();
                ffi = null;
            }
        }

        public void StartLocalServer() {
            ffi = new Ffi.Context(null);
        }

        public void ConnectToServer(string remoteAddress)
        {
            ffi = new Ffi.Context(remoteAddress);
        }

        public List<IResponse> Execute(List<IRequest> requests)
        {
            if (ffi == null)
            {
                Debug.LogWarning("FFI was not started");
                return new List<IResponse>();
            }

            SendRequests(requests);
            return GetResponses();
        }

        private List<IResponse> GetResponses()
        {
            IResponse[] result = null;
            ffi.Execute(bytes =>
            {
                // all references to bytes/buffer/response should be released in the end of this scope
                result = Deserialize(bytes);
            });

            if (result == null)
            {
                throw new NullReferenceException();
            }

            return result.ToList();
        }

        private static IResponse[] Deserialize(byte[] bytes)
        {
            var buffer = new ByteBuffer(bytes);
            var response = Responses.GetRootAsResponses(buffer);

            var result = new IResponse[response.TotalMessages];

            Action<uint, IResponse> setResult = (index, item) =>
            {
                if (index >= result.Length)
                {
                    throw new IndexOutOfRangeException();
                }

                if (result[index] != null)
                {
                    throw new ArgumentException();
                }

                result[index] = item;
            };

            for (int i = 0; i < response.EmptyPackagesLength; i++)
            {
                var pack = response.EmptyPackages(i) ?? throw new NullReferenceException();
                var ordering = pack.Ordering;

                switch (pack.Kind)
                {
                    case ResponseKind.GameStarted:
                        setResult(ordering, new ResponseGameStart());
                        break;

                    case ResponseKind.GameStatusIdle:
                        setResult(ordering, new ResponseGameStatus()
                        {
                            status = ResponseGameStatus.GameStatus.Idle
                        });
                        break;

                    case ResponseKind.GameStatusRunning:
                        setResult(ordering, new ResponseGameStatus()
                        {
                            status = ResponseGameStatus.GameStatus.Playing
                        });
                        break;

                    default:
                        Debug.LogWarning($"Unknown kind [${pack.Kind}] for simple package");
                        continue;
                }
            }

            for (int i = 0; i < response.CreatePackagesLength; i++)
            {
                var package = response.CreatePackages(i) ?? throw new NullReferenceException();
                setResult(package.Ordering,
                    new ResponseSpawn() {id = package.Id, position = new Vector3(package.X, package.Y, 0f)});
            }

            for (int i = 0; i < response.PosPackagesLength; i++)
            {
                var package = response.PosPackages(i) ?? throw new NullReferenceException();
                setResult(package.Ordering,
                    new ResponsePos() {id = package.Id, position = new Vector3(package.X, package.Y, 0f)});
            }

            return result;
        }

        private void SendRequests(List<IRequest> requests)
        {
            var bytes = SerializeRequests(requests);
            ffi.Send(bytes);
        }
        
        private byte[] SerializeRequests(List<IRequest> requests)
        {
            var buffer = new ByteBuffer(1024);
            var b = new FlatBufferBuilder(buffer);
            uint ordering = 0;
            
            // aggregate request by package kind
            var emptyPackages = new List<Action>();
            
            foreach (var r in requests)
            {
                if (r is RequestStartNewGame)
                {
                    emptyPackages.Add(() =>
                    {
                        FfiRequests.EmptyPackage.CreateEmptyPackage(b, RequestKind.StartGame, ordering++);
                    });
                } 
                else if (r is RequestFullState)
                {
                    emptyPackages.Add(() =>
                    {
                        FfiRequests.EmptyPackage.CreateEmptyPackage(b, RequestKind.GetAll, ordering++);
                    });
                }
                else if (r is RequestGameStatus)
                {
                    emptyPackages.Add(() =>
                    {
                        FfiRequests.EmptyPackage.CreateEmptyPackage(b, RequestKind.GameStatus, ordering++);
                    });
                }
            }

            Requests.StartEmptyPackagesVector(b, emptyPackages.Count);
            foreach (var p in emptyPackages)
            {
                p();
            }
            var emptyPackagesOffset = b.EndVector();
            
            Requests.StartRequests(b);
            Requests.AddTotalMessages(b, (uint) requests.Count);
            if (emptyPackages.Count > 0)
            {
                Requests.AddEmptyPackages(b, emptyPackagesOffset);
            }
            var requestsAddr = Requests.EndRequests(b);
            b.Finish(requestsAddr.Value);
            
            return b.SizedByteArray();
        }
    }
}
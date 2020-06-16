using System;
using System.Collections.Generic;
using System.IO;
using server;
using UnityEngine;

namespace Domain
{

    /// <summary>
    /// Game logic implemented by a Server
    /// </summary>
    public class DomainFFI : MonoBehaviour, IDomain
    {
        private Ffi.Context ffi;

        private List<IEvent> events = new List<IEvent>();

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
            throw new System.NotImplementedException();
        }

        public void Execute()
        {
            foreach (var package in ffi.Take())
            {
                var buffer = new BinaryReader(new MemoryStream(package));
                var packageKindId = buffer.ReadInt16();
                var packageKind = (PackageKind) packageKindId;
                
                switch (packageKind)
                {
                    case PackageKind.ResponseLogin:
                        break;
                    
                    case PackageKind.ResponseChange:
                        var e = ParseResponseChange(buffer);
                        events.Add(e);
                        break;
                }
            }
        }

        public List<IEvent> TakeEvents()
        {
            var result = this.events;
            this.events = new List<IEvent>();
            return result;
        }

        private IEvent ParseResponseChange(BinaryReader buffer)
        {
            var str = Byte2String(buffer.ReadBytes());
            var args = str.Split('\n');
            switch (args[0])
            {
                case "load_scene":
                {
                    return new EventLoadScene() {sceneName = args[1]};
                }
                
                case "spawn":
                {
                    var id = ParseId(args[1]);
                    var pos = ParsePos(args[2]);
                    return new EventSpawn() {id = id, position = pos, prefab = args[3]};
                }

                case "pos":
                {
                    var id = ParseId(args[1]);
                    var pos = ParsePos(args[2]);
                    return new EventPos() { id = id, position = pos};
                }
                
                default:
                    throw new Exception($"unexpected event type: '{str}'");
            }
        }

        private string Byte2String(byte[] bytes)
        {
            return System.Text.Encoding.UTF8.GetString(bytes);
        }

        private int ParseId(string value)
        {
            return int.Parse(value);
        }
        private float ParseFloat(string value)
        {
            return float.Parse(value);
        }
        private Vector3 ParsePos(string value)
        {
            var args = value.Split(',');
            var x = ParseFloat(args[0]);
            var y = ParseFloat(args[1]);
            var z = ParseFloat(args[2]);
            return new Vector3(x, y, z);
        }
    }
}
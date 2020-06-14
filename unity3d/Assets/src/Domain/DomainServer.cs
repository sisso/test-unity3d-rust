using System;
using System.Collections.Generic;
using server;
using Server;
using UnityEditor;
using UnityEditor.UIElements;
using UnityEngine;
using UnityEngine.Analytics;
using UnityEngine.UIElements.Experimental;

namespace Domain
{
    public interface IEvent
    {
    }

    public class EventLoadScene : IEvent
    {
        public string sceneName;
    }

    public class EventSpawn : IEvent
    {
        public int id;
        public string prefab;
        public Vector3 position;
    }

    public class EventPos : IEvent
    {
        public int id;
        public Vector3 position;
    }

    public class DomainServer : MonoBehaviour, IDomain
    {
        private IServer server;

        private List<IEvent> events = new List<IEvent>();

        void OnDestroy()
        {
            Debug.Log("Closing context");
            server.Dispose();
            server = null;
        }

        public void StartLocalServer()
        {
            server = new Server.LocalServer();
        }

        public void ConnectToServer(string remoteAddress)
        {
            throw new System.NotImplementedException();
        }

        public void Execute()
        {
            foreach (var msg in server.Take())
            {
                switch (msg.kind)
                {
                    case PackageKind.ResponseLogin:
                        break;
                    
                    case PackageKind.ResponseChange:
                        var e = ParseResponseChange(msg.body);
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

        private IEvent ParseResponseChange(byte[] bytes)
        {
            var str = Byte2String(bytes);
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
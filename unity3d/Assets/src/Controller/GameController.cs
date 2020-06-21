using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;
using Server;
using Domain;
using UnityEngine.SceneManagement;
using UnityEngine.Serialization;

namespace Controller {
    public enum RunMode
    {
        Fake,
        Local,
        Remote
    }
    
    public class GameController : MonoBehaviour
    {
        public RunMode mode;

        public string remoteAddress;

        public DomainFake fakePrefab;
        [FormerlySerializedAs("serverPrefab")] public DomainFFI ffiPrefab;
        public GameObject playerPrefab;
        
        private IDomain current;

        void Start()
        {
            switch (mode)
            {
                case RunMode.Fake:
                {
                    var fake = Instantiate(fakePrefab);
                    current = fake;
                    break;
                }
                
                case RunMode.Local:
                {
                    var server = Instantiate(ffiPrefab);
                    server.StartLocalServer();
                    current = server;
                    break;
                }
                
                case RunMode.Remote:
                {
                    var server = Instantiate(ffiPrefab);
                    server.ConnectToServer(remoteAddress);
                    current = server;
                    break;
                }
            }
            
            DontDestroyOnLoad(this);
        }

        void FixedUpdate()
        {
            foreach (var e in current.Execute())
            {
                if (e is ResponseStartGame)
                {
                    throw new System.NotImplementedException();
                }
                else if (e is ResponseSpawn)
                {
                    var ev = e as ResponseSpawn;
                    GameObject obj;
                    if (ev.prefab == responses.PrefabKind.Player)
                        obj = Instantiate(playerPrefab);
                    else 
                        throw new System.NotImplementedException();
                    
                    obj.GetComponent<DomainRef>().id = ev.id;
                    obj.transform.position = ev.position;
                }
                else if (e is ResponsePos)
                {
                    var ev = e as ResponsePos;
                    var obj = GetDomainObjById(ev.id);
                    obj.transform.position = ev.position;
                }
                else
                {
                    throw new System.NotImplementedException();
                }
            }
        }

        void OnGui()
        {
            
        }

        DomainRef GetDomainObjById(uint id)
        {
            // TODO: optimize
            return FindObjectsOfType<DomainRef>()
                .Where(i => i.id == id)
                .First();
        }
    }
}

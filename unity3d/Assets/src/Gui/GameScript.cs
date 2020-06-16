using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Linq;
using Server;
using Domain;
using UnityEngine.SceneManagement;
using UnityEngine.Serialization;

namespace Gui {
    public enum RunMode
    {
        Fake,
        Local,
        Remote
    }
    
    public class GameScript : MonoBehaviour
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
            current.Execute();

            foreach (var e in current.TakeEvents())
            {
                if (e is EventLoadScene)
                {
                    var ev = e as EventLoadScene;
                    SceneManager.LoadScene(ev.sceneName);
                }
                else if (e is EventSpawn)
                {
                    var ev = e as EventSpawn;
                    var obj = Instantiate(playerPrefab);
                    obj.GetComponent<DomainRef>().id = ev.id;
                    obj.transform.position = ev.position;
                }
                else if (e is EventPos)
                {
                    var ev = e as EventPos;
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

        DomainRef GetDomainObjById(int id)
        {
            // TODO: it should be indexed
            return FindObjectsOfType<DomainRef>()
                .Where(i => i.id == id)
                .First();
        }
        
        
    }
}

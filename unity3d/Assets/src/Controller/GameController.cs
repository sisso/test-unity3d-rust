using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;
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
        enum State
        {
            Idle,
            Playing,
        }
        
        public RunMode mode;

        public string remoteAddress;

        public DomainFake fakePrefab;
        public DomainFFI ffiPrefab;
        public GameObject playerPrefab;
        
        private IDomain current;

        private State state = State.Idle;

        private List<IRequest> pendingRequests = new List<IRequest>();

        void Start()
        {
            switch (mode)
            {
                case RunMode.Fake:
                {
                    Debug.Log("Starting GameController in Fake mode.");
                    var fake = Instantiate(fakePrefab);
                    DontDestroyOnLoad(fake.gameObject);
                    current = fake;
                    break;
                }
                
                case RunMode.Local:
                {
                    Debug.Log("Starting GameController in FFI Local mode.");
                    var server = Instantiate(ffiPrefab);
                    DontDestroyOnLoad(server.gameObject);
                    server.StartLocalServer();
                    current = server;
                    break;
                }
                
                case RunMode.Remote:
                {
                    Debug.Log("Starting GameController in FFI Server mode.");
                    var server = Instantiate(ffiPrefab);
                    DontDestroyOnLoad(server.gameObject);
                    server.ConnectToServer(remoteAddress);
                    current = server;
                    break;
                }
            }
            
            DontDestroyOnLoad(this);
            
            LoadStartGameScene();
        }

        void FixedUpdate()
        {
            if (state == State.Playing)
            {
                var responses = current.Execute(pendingRequests);
                pendingRequests.Clear();
                
                foreach (var e in responses)
                {
                    if (e is ResponseGameStart || e is ResponseFullState)
                    {
                        ClearGameState();
                        LoadStartGameScene();
                    } 
                    else if (e is ResponseGameStatus)
                    {
                        var resp = e as ResponseGameStatus;
                        if (resp.status == ResponseGameStatus.GameStatus.Idle)
                        {
                            pendingRequests.Add(new RequestStartNewGame());
                        }
                        else
                        {
                            pendingRequests.Add(new RequestFullState());
                        }
                    } 
                    else if (e is ResponseSpawn)
                    {
                        var ev = e as ResponseSpawn;
                        GameObject obj;
                        if (ev.prefab == FfiResponses.PrefabKind.Player)
                            obj = Instantiate(playerPrefab);
                        else
                            throw new System.NotImplementedException();

                        DomainRef.Add(obj, new RefId(ev.id));
                        obj.transform.position = ev.position;
                    }
                    else if (e is ResponsePos)
                    {
                        var ev = e as ResponsePos;
                        var obj = GetDomainObjById(new RefId(ev.id));
                        obj.transform.position = ev.position;
                    }
                    else
                    {
                        throw new System.NotImplementedException();
                    }
                }
            }
        }

        private void LoadStartGameScene()
        {
            SceneManager.LoadScene(sceneBuildIndex: 1);
            state = State.Playing;
        }

        void OnGui()
        {
            
        }

        void ClearGameState()
        {
            throw new System.NotImplementedException();
        }

        DomainRef GetDomainObjById(RefId id)
        {
            // TODO: optimize
            return FindObjectsOfType<DomainRef>()
                .Where(i => i.id.Equals(id))
                .First();
        }
    }
}

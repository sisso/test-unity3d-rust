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
            Unknown,
            Idle,
            Playing,
        }
        
        public RunMode mode;

        public string remoteAddress;

        public DomainFake fakePrefab;
        public DomainFFI ffiPrefab;
        public GameObject playerPrefab;
        
        private IDomain current;

        private State state = State.Unknown;

        private List<IRequest> pendingRequests = new List<IRequest>();

        void Start()
        {
            DontDestroyOnLoad(this);
            
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

            InitializeGame();
        }

        void FixedUpdate()
        {
            foreach (var req in pendingRequests)
            {
                Debug.Log("sending request "+req.GetType().Name);
            }
            
            var responses = current.Execute(pendingRequests);
            pendingRequests.Clear();
            
            foreach (var e in responses)
            {
                Debug.Log("receive response "+e.GetType().Name);
                
                if (e is ResponseGameStart || e is ResponseFullState)
                {
                    state = State.Playing;
                    ClearGameState();
                    LoadStartGameScene();
                } 
                else if (e is ResponseGameStatus)
                {
                    var resp = e as ResponseGameStatus;
                    if (resp.status == ResponseGameStatus.GameStatus.Idle)
                    {
                        Debug.Log("Receive game status. Game is idling. Request  new game");
                        pendingRequests.Add(new RequestStartNewGame());
                        state = State.Idle;
                    }
                    else
                    {
                        Debug.Log("Receive game status. Game is in progress. Request full state");
                        pendingRequests.Add(new RequestFullState());
                        state = State.Playing;
                    }
                } 
                else if (e is ResponseSpawn)
                {
                    var ev = e as ResponseSpawn;
                    Debug.Log($"Receive new spawn of prefab {ev.prefab} with id {ev.id} at {ev.position}");
                    
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
                    Debug.Log($"Receive new position for {ev.id} at {ev.position}");
                    
                    var obj = GetDomainObjById(new RefId(ev.id));
                    obj.transform.position = ev.position;
                }
                else
                {
                    throw new System.NotImplementedException($"Unsupported server response {e.GetType()}");
                }
            }
        }

        /// <summary>
        /// Initialize game by requesting initial state
        /// </summary>
        void InitializeGame()
        {
            Debug.Log("Requesting game status");
            pendingRequests.Add(new RequestGameStatus());
        }

        void LoadStartGameScene()
        {
            SceneManager.LoadScene(sceneBuildIndex: 1);
        }

        void OnGui()
        {
            
        }

        void ClearGameState()
        {
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

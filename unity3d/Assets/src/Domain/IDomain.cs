using System.Collections.Generic;
using UnityEngine;

namespace Domain
{
    
    /// <summary>
    /// Messages send to the game
    /// </summary>
    public interface IRequest {}

    public class RequestStartNewGame : IRequest
    {
        
    }
    
    public class RequestFullState : IRequest {}
    
    public class RequestGameStatus: IRequest {}
    
    
    /// <summary>
    /// Messages received from the game
    /// </summary>
    public interface IResponse
    {
    }

    public class ResponseGameStart : IResponse
    {
        
    }

    public class ResponseGameStatus: IResponse
    {
        public enum GameStatus
        {
            Idle,
            Playing
        }
        
        public GameStatus status;
    }
    
    public class ResponseFullState: IResponse
    {
        
    }

    public class ResponseSpawn : IResponse
    {
        public uint id;
        public FfiResponses.PrefabKind prefab;
        public Vector3 position;
    }

    public class ResponsePos : IResponse
    {
        public uint id;
        public Vector3 position;
    }
    
    
    /// <summary>
    /// Represent all domain logic for to the game
    /// </summary>
    public interface IDomain
    {
        List<IResponse> Execute(List<IRequest> requests);
    }
}
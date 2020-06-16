using System.Collections.Generic;
using UnityEngine;

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
    
    
    /// <summary>
    /// Represent all domain logic for to the game
    /// </summary>
    public interface IDomain
    {
        void Execute();
        List<IEvent> TakeEvents();
    }
}
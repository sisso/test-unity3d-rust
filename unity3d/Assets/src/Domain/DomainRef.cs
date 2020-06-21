using UnityEngine;

namespace Domain
{
    [System.Serializable]
    public struct RefId
    {
        public RefId(uint id)
        {
            this.id = id;
        }

        public bool Equals(RefId other)
        {
            return id == other.id;
        }

        public override bool Equals(object obj)
        {
            return obj is RefId other && Equals(other);
        }

        public override int GetHashCode()
        {
            return (int) id;
        }

        public uint id;

        public bool Is(uint id)
        {
            return this.id == id;
        }
    } 
    
    public class DomainRef : MonoBehaviour
    {
        public static DomainRef Add(GameObject obj, RefId id)
        {
            var c = obj.AddComponent<DomainRef>();
            c.id = id;
            return c;
        }
        
        public RefId id;
    }
}
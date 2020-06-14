using System.Collections.Generic;
using UnityEngine;

namespace Domain
{
    public class DomainFake : MonoBehaviour, IDomain
    {
        public void Execute()
        {
            throw new System.NotImplementedException();
        }

        public List<IEvent> TakeEvents()
        {
            throw new System.NotImplementedException();
        }
    }
}
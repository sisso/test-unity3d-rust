using System.Collections.Generic;
using UnityEngine;

namespace Domain
{
    /// <summary>
    /// Domain logic implemented fake for local testing
    /// </summary>
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
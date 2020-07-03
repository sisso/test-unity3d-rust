using System.Collections.Generic;
using FfiResponses;
using UnityEngine;

namespace Domain
{
    /// <summary>
    /// Domain logic implemented fake for local testing
    /// </summary>
    public class DomainFake : MonoBehaviour, IDomain
    {
        public int state = 0;

        public List<IResponse> Execute(List<IRequest> requests)
        {
            this.state++;

            var responses = new List<IResponse>();

            if (this.state == 50)
            {
                responses.Add(new ResponseSpawn()
                    {id = 0, position = new Vector3(0f, 0f, 0f), prefab = PrefabKind.Player});
            }
            else if (state > 100)
            {
                responses.Add(new ResponsePos() {id = 0, position = new Vector3((state - 100f) / 100f, 0f, 0f)});
            }

            return responses;
        }

    }
}
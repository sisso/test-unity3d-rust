using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

namespace TestFfi.Gui {
    public class GameScript : MonoBehaviour
    {
        public GameObject target;

        private Server.Ffi.Context context;

        void Start()
        {
            Debug.Log("starting context");
            this.context = new Server.Ffi.Context();
        }

        void OnDestroy()
        {
            Debug.Log("Closing context");
            this.context.Dispose();
            this.context = null;
        }

        void Update()
        {

        }
    }
}

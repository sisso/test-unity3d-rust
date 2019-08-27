using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UseSum : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        var result = Rust.Proxy.Sum(2, 2);
        Debug.Log("Receive: " + result);
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}

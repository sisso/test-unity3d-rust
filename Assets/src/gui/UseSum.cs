using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UseSum : MonoBehaviour
{
    public GameObject target;

    // Start is called before the first frame update
    void Start()
    {
        var result = Rust.Proxy.Sum(2, 2);
        var stru = Rust.Proxy.Get();
        target.transform.Translate(stru.x, result, stru.y);
        Debug.Log("Receive: " + result + " and "+ stru);
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}

using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UseSum : MonoBehaviour
{
    public GameObject target;

    public int state = 0;

    // Start is called before the first frame update
    void Start()
    {
        var result = Rust.Proxy.Sum(2, 2);
        var stru = Rust.Proxy.Get();
        target.transform.Translate(stru.x, result, stru.y);
        Debug.Log("Receive: " + result + " and " + stru);

        var list = Rust.Proxy.GetBuffer();
        Debug.Log("List: " + list.Count);
        foreach (var i in list) {
            Debug.Log("- " + i.x + " " + i.y);
        }
    }

    void Destroy()
    {
        Debug.Log("Closing context");
        Rust.Proxy.CloseContext();
    }

    void OnApplicationQuit()
    {
        Debug.Log("Closing context by OnApplicationQuit");
        // TODO FIXME OMG
        // Rust.Proxy.CloseContext();
    }

    // Update is called once per frame
    void Update()
    {
        // Debug.Log("Get value: " + Rust.Proxy.GetContext().GetInput());
        state += 1;
        Rust.Proxy.GetContext().SetInput(state);
    }
}

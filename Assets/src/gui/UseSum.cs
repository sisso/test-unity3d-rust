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

    // Update is called once per frame
    void Update()
    {
        if (state == 0)
        {
            state = 1;
        }
        else if (state == 1)
        {

            state = 2;
        }
    }
}

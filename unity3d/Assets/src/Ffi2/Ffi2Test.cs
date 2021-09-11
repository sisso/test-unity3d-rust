using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Ffi2Test : MonoBehaviour
{
    private ffi_domain_2.TestClass c;
    // Start is called before the first frame update
    void Start()
    {
        c = new ffi_domain_2.TestClass();
    }

    // Update is called once per frame
    void Update()
    {
        c.Add(1);

        if (c.Get() % 100 == 0)
        {
           Debug.Log($"Value: {c.Get()}"); 
        }
    }
    
    
}

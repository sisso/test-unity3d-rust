using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class UseSum : MonoBehaviour
{
    public GameObject target;

    public int state = 0;

    private Rust.Context context;

    // Start is called before the first frame update
    void Start()
    {
        //var stru = Rust.Proxy.Get();
        //target.transform.Translate(stru.x, result, stru.y);
        //Debug.Log("Receive: " + result + " and " + stru);

        //var list = Rust.Proxy.GetBuffer();
        //Debug.Log("List: " + list.Count);
        //foreach (var i in list) {
        //    Debug.Log("- " + i.x + " " + i.y);
        //}

        Debug.Log("starting context");
        this.context = new Rust.Context();
        //Debug.Log("context created "+this.context);

    }

    void Destroy()
    {
        Debug.Log("Closing context");
        this.context.Dispose();
        this.context = null;
    }

    //void OnApplicationQuit()
    //{
    //    Debug.Log("Closing context by OnApplicationQuit");
    //    // TODO FIXME OMG
    //    Rust.Proxy.CloseContext();
    //}



    // Update is called once per frame
    void Update()
    {
        var buffer = "";

        // Debug.Log("Get value: " + context.GetInput());
        state += 1;
        context.SetInput(state);

        for (int i = 0; i < 3; i++)
        {
            if (Input.GetMouseButtonDown(i))
            {
                context.AddRequest("button "+i+" down AcciÃ³n");
            }

            if (Input.GetMouseButtonUp(i))
            {
                context.AddRequest("button "+i+" up ü");
            }
        }

        context.Execute();

        var responses = context.GetResponses();
        foreach (var line in responses.Split('\n')) {
            if (line.Length == 0) continue;

            Debug.Log("response " + line);
        }

        if (state % 2 == 0)
        {
            Debug.Log("Sending bytes");
            context.AddByteRequest(new byte[] { 5, 3, 1, 3 });
        }
        else
        {
            buffer = "Receive bytes";
            var bytes = context.GetByteRequest();
            for (var i = 0; i < bytes.Length; i++)
            {
                buffer += " " + bytes[i];
            }

            Debug.Log(buffer);
        }

        // this.context.GetMessages();

        buffer = "";
        var list_deleted = this.context.GetDeletedEntities();
        foreach (var line in list_deleted)
        {
            buffer += line + ",";
        }
        Debug.Log("list_deleted: " + buffer);

        //Debug.Log("Running GetNewEntities");
        //this.context.GetNewEntities();
    }
}

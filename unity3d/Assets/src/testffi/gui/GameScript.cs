using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TestFFI.Domain;
using System;

namespace TestFFI.Gui {
public class GameScript : MonoBehaviour
{
    public GameObject target;

    public int state = 0;

    private Context context;

    void Start()
    {
        // Debug.Log("starting context");
        this.context = new Domain.Context();
    }

    void Destroy()
    {
        // Debug.Log("Closing context");
        this.context.Dispose();
        this.context = null;
    }

    static void Assert(bool condition)
    {
        if (!condition)
            throw new Exception("Assertion fail");
    }

    // TODO: move to automatic tests
    static void AddNumberTest()
    {
        var result = FFI.test_ffi_add_numbers(1, 3);
        // Debug.Log("AddNumberTest " + result);
        Assert(result == 4);
    }

    static void SendAndReceiveStringTest(Context context)
    {
        context.SetString("schönes");
        var str = context.GetString();

        // Debug.Log("SendAndReceiveStringTest receive " + str);
        Assert(str == "schönes");
    }

    static void SendAndReceiveStructTest(Context context)
    {
        V2 v2 = new V2 { x = 9, y = 8 };
        context.SetV2(v2);

        var value = context.GetV2();

        // Debug.Log("SendAndReceiveStructTest receive " + value);
        Assert(value.x == 9);
        Assert(value.y == 8);
    }

    static void SendAndReceiveArrayTest(Context context)
    {
        byte[] bytes = new byte[] { 3, 4, 5, 6, 7 };
        context.SetArray(bytes);

        var value = context.GetArray();
        var str = "[";
        for (int i = 0; i < value.Length; i++)
        {
            str += value[i] + (i == value.Length - 1 ? "" : ",");
        }
        str += "]";

        // Debug.Log("SendAndReceiveArrayTest receive " + str);
        Assert(value.Length == 5);
        Assert(value[0] == 3);
        Assert(value[4] == 7);
    }

    static void SendAndReceiveStructArrayTest(Context context)
    {
        V2[] array = new V2[] { new V2 { x = 1, y = 2 }, new V2 { x = 3, y = 4 } };
        context.SetStructArray(array);

        var value = context.GetStructArray();
        var str = "[";
        for (int i = 0; i < value.Length; i++)
        {
            str += value[i] + (i == value.Length - 1 ? "" : ",");
        }
        str += "]";

        // Debug.Log("SendAndReceiveArrayTest receive " + str);
        Assert(value.Length == 2);
        Assert(value[0].x == 1);
        Assert(value[1].y == 4);
    }

    static void SendAndReceivePeopleTest(Context context)
    {
        FFIPerson[] array = new FFIPerson[] {
                new FFIPerson { id = 0, name = "Roger", number = 5 },
                new FFIPerson { id = 1, name = "Borge", number = 6 },
                new FFIPerson { id = 2, name = "Loger", number = 7 },
                new FFIPerson { id = 3, name = "Noger", number = 8 }
            };
        context.SetPeople(array);

        var value = context.GetPeople();
        var str = "[";
        for (int i = 0; i < value.Length; i++)
        {
            str += value[i] + (i == value.Length - 1 ? "" : ",");
        }
        str += "]";

        // Debug.Log("SendAndReceivePeopleTest receive " + str);
        Assert(value.Length == 4);
        Assert(value[0].id == 0);
        Assert(value[2].number == 7);
        Assert(value[3].name == "Noger");
    }

    static void SendAndReceiveFlatBuffersTest(Context context)
    {
        var vectors = new int[][] {
                new int[] {32, 33},
                new int[] {34, 22},
            };

        context.SetFlatBuffers(vectors);
        var value = context.GetFlatBuffers();
        var str = "[";
        for (int i = 0; i < value.Length; i++)
        {
            str += "(" + value[i][0] + "," + value[i][1] + ")" + (i == value.Length - 1 ? "" : ",");
        }
        str += "]";
        // Debug.Log("SendAndReceiveFlatBuffersTest receive " + str);
    }

    void Update()
    {
        this.state += 1;

        if (this.state % 2 == 0) return;

        AddNumberTest();

        SendAndReceiveStringTest(context);
        SendAndReceiveStructTest(context);
        SendAndReceiveArrayTest(context);
        SendAndReceiveStructArrayTest(context);
        SendAndReceivePeopleTest(context);
        SendAndReceiveFlatBuffersTest(context);
    }
}
}

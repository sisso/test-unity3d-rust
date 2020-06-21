using System.Collections;
using System.Collections.Generic;
using NUnit.Framework;
using UnityEngine;
using UnityEngine.TestTools;
using Domain;
using System;
using FlatBuffers;

namespace Tests
{
    public class TestFlatBuffers
    {
        // A Test behaves as an ordinary method
        [Test]
        public void TestParseResponses()
        {
            var bytes = Convert.FromBase64String("BAAEAAQAAAA=");
            var buffer = new ByteBuffer(bytes);
            var response = FfiResponses.Responses.GetRootAsResponses(buffer);
            Console.Out.WriteLine("Lenght: " + response.SimpleLength);

        }
    }
}

// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace FfiResponses
{

using global::System;
using global::FlatBuffers;

public struct V2Package : IFlatbufferObject
{
  private Struct __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public void __init(int _i, ByteBuffer _bb) { __p = new Struct(_i, _bb); }
  public V2Package __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public FfiResponses.ResponseKind Kind { get { return (FfiResponses.ResponseKind)__p.bb.GetUshort(__p.bb_pos + 0); } }
  public uint Ordering { get { return __p.bb.GetUint(__p.bb_pos + 4); } }
  public float X { get { return __p.bb.GetFloat(__p.bb_pos + 8); } }
  public float Y { get { return __p.bb.GetFloat(__p.bb_pos + 12); } }

  public static Offset<FfiResponses.V2Package> CreateV2Package(FlatBufferBuilder builder, FfiResponses.ResponseKind Kind, uint Ordering, float X, float Y) {
    builder.Prep(4, 16);
    builder.PutFloat(Y);
    builder.PutFloat(X);
    builder.PutUint(Ordering);
    builder.Pad(2);
    builder.PutUshort((ushort)Kind);
    return new Offset<FfiResponses.V2Package>(builder.Offset);
  }
};


}

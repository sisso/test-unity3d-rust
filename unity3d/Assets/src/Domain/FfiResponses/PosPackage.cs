// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace FfiResponses
{

using global::System;
using global::FlatBuffers;

public struct PosPackage : IFlatbufferObject
{
  private Struct __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public void __init(int _i, ByteBuffer _bb) { __p = new Struct(_i, _bb); }
  public PosPackage __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public FfiResponses.ResponseKind Kind { get { return (FfiResponses.ResponseKind)__p.bb.GetUshort(__p.bb_pos + 0); } }
  public uint Ordering { get { return __p.bb.GetUint(__p.bb_pos + 4); } }
  public uint Id { get { return __p.bb.GetUint(__p.bb_pos + 8); } }
  public float X { get { return __p.bb.GetFloat(__p.bb_pos + 12); } }
  public float Y { get { return __p.bb.GetFloat(__p.bb_pos + 16); } }

  public static Offset<FfiResponses.PosPackage> CreatePosPackage(FlatBufferBuilder builder, FfiResponses.ResponseKind Kind, uint Ordering, uint Id, float X, float Y) {
    builder.Prep(4, 20);
    builder.PutFloat(Y);
    builder.PutFloat(X);
    builder.PutUint(Id);
    builder.PutUint(Ordering);
    builder.Pad(2);
    builder.PutUshort((ushort)Kind);
    return new Offset<FfiResponses.PosPackage>(builder.Offset);
  }
};


}

// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

using global::System;
using global::FlatBuffers;

public struct IdPackage : IFlatbufferObject
{
  private Struct __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public void __init(int _i, ByteBuffer _bb) { __p = new Struct(_i, _bb); }
  public IdPackage __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public uint Id { get { return __p.bb.GetUint(__p.bb_pos + 0); } }

  public static Offset<IdPackage> CreateIdPackage(FlatBufferBuilder builder, uint Id) {
    builder.Prep(4, 4);
    builder.PutUint(Id);
    return new Offset<IdPackage>(builder.Offset);
  }
};


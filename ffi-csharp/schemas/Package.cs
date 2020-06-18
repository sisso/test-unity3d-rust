// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

using global::System;
using global::FlatBuffers;

public struct Package : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_1_11_1(); }
  public static Package GetRootAsPackage(ByteBuffer _bb) { return GetRootAsPackage(_bb, new Package()); }
  public static Package GetRootAsPackage(ByteBuffer _bb, Package obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public Package __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public MessageKind Kind { get { int o = __p.__offset(4); return o != 0 ? (MessageKind)__p.bb.GetUshort(o + __p.bb_pos) : MessageKind.StartGame; } }
  public IdPackage? Id { get { int o = __p.__offset(6); return o != 0 ? (IdPackage?)(new IdPackage()).__assign(o + __p.bb_pos, __p.bb) : null; } }
  public PosPackage? Pos { get { int o = __p.__offset(8); return o != 0 ? (PosPackage?)(new PosPackage()).__assign(o + __p.bb_pos, __p.bb) : null; } }
  public StringPackage? Str { get { int o = __p.__offset(10); return o != 0 ? (StringPackage?)(new StringPackage()).__assign(__p.__indirect(o + __p.bb_pos), __p.bb) : null; } }
  public BytesPackage? Bytes { get { int o = __p.__offset(12); return o != 0 ? (BytesPackage?)(new BytesPackage()).__assign(__p.__indirect(o + __p.bb_pos), __p.bb) : null; } }

  public static void StartPackage(FlatBufferBuilder builder) { builder.StartTable(5); }
  public static void AddKind(FlatBufferBuilder builder, MessageKind kind) { builder.AddUshort(0, (ushort)kind, 0); }
  public static void AddId(FlatBufferBuilder builder, Offset<IdPackage> idOffset) { builder.AddStruct(1, idOffset.Value, 0); }
  public static void AddPos(FlatBufferBuilder builder, Offset<PosPackage> posOffset) { builder.AddStruct(2, posOffset.Value, 0); }
  public static void AddStr(FlatBufferBuilder builder, Offset<StringPackage> strOffset) { builder.AddOffset(3, strOffset.Value, 0); }
  public static void AddBytes(FlatBufferBuilder builder, Offset<BytesPackage> bytesOffset) { builder.AddOffset(4, bytesOffset.Value, 0); }
  public static Offset<Package> EndPackage(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Package>(o);
  }
  public static void FinishPackageBuffer(FlatBufferBuilder builder, Offset<Package> offset) { builder.Finish(offset.Value); }
  public static void FinishSizePrefixedPackageBuffer(FlatBufferBuilder builder, Offset<Package> offset) { builder.FinishSizePrefixed(offset.Value); }
};


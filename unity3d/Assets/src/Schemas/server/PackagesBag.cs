// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace server
{

using global::System;
using global::FlatBuffers;

public struct PackagesBag : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_1_11_1(); }
  public static PackagesBag GetRootAsPackagesBag(ByteBuffer _bb) { return GetRootAsPackagesBag(_bb, new PackagesBag()); }
  public static PackagesBag GetRootAsPackagesBag(ByteBuffer _bb, PackagesBag obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public PackagesBag __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public server.Package? Packages(int j) { int o = __p.__offset(4); return o != 0 ? (server.Package?)(new server.Package()).__assign(__p.__indirect(__p.__vector(o) + j * 4), __p.bb) : null; }
  public int PackagesLength { get { int o = __p.__offset(4); return o != 0 ? __p.__vector_len(o) : 0; } }

  public static Offset<server.PackagesBag> CreatePackagesBag(FlatBufferBuilder builder,
      VectorOffset packagesOffset = default(VectorOffset)) {
    builder.StartTable(1);
    PackagesBag.AddPackages(builder, packagesOffset);
    return PackagesBag.EndPackagesBag(builder);
  }

  public static void StartPackagesBag(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddPackages(FlatBufferBuilder builder, VectorOffset packagesOffset) { builder.AddOffset(0, packagesOffset.Value, 0); }
  public static VectorOffset CreatePackagesVector(FlatBufferBuilder builder, Offset<server.Package>[] data) { builder.StartVector(4, data.Length, 4); for (int i = data.Length - 1; i >= 0; i--) builder.AddOffset(data[i].Value); return builder.EndVector(); }
  public static VectorOffset CreatePackagesVectorBlock(FlatBufferBuilder builder, Offset<server.Package>[] data) { builder.StartVector(4, data.Length, 4); builder.Add(data); return builder.EndVector(); }
  public static void StartPackagesVector(FlatBufferBuilder builder, int numElems) { builder.StartVector(4, numElems, 4); }
  public static Offset<server.PackagesBag> EndPackagesBag(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<server.PackagesBag>(o);
  }
  public static void FinishPackagesBagBuffer(FlatBufferBuilder builder, Offset<server.PackagesBag> offset) { builder.Finish(offset.Value); }
  public static void FinishSizePrefixedPackagesBagBuffer(FlatBufferBuilder builder, Offset<server.PackagesBag> offset) { builder.FinishSizePrefixed(offset.Value); }
};


}

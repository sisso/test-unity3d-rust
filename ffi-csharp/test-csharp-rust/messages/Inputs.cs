// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace messages
{

using global::System;
using global::FlatBuffers;

public struct Inputs : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_1_11_1(); }
  public static Inputs GetRootAsInputs(ByteBuffer _bb) { return GetRootAsInputs(_bb, new Inputs()); }
  public static Inputs GetRootAsInputs(ByteBuffer _bb, Inputs obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public Inputs __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public messages.InputEmpty? InputsEmpty(int j) { int o = __p.__offset(4); return o != 0 ? (messages.InputEmpty?)(new messages.InputEmpty()).__assign(__p.__indirect(__p.__vector(o) + j * 4), __p.bb) : null; }
  public int InputsEmptyLength { get { int o = __p.__offset(4); return o != 0 ? __p.__vector_len(o) : 0; } }

  public static Offset<messages.Inputs> CreateInputs(FlatBufferBuilder builder,
      VectorOffset inputs_emptyOffset = default(VectorOffset)) {
    builder.StartTable(1);
    Inputs.AddInputsEmpty(builder, inputs_emptyOffset);
    return Inputs.EndInputs(builder);
  }

  public static void StartInputs(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddInputsEmpty(FlatBufferBuilder builder, VectorOffset inputsEmptyOffset) { builder.AddOffset(0, inputsEmptyOffset.Value, 0); }
  public static VectorOffset CreateInputsEmptyVector(FlatBufferBuilder builder, Offset<messages.InputEmpty>[] data) { builder.StartVector(4, data.Length, 4); for (int i = data.Length - 1; i >= 0; i--) builder.AddOffset(data[i].Value); return builder.EndVector(); }
  public static VectorOffset CreateInputsEmptyVectorBlock(FlatBufferBuilder builder, Offset<messages.InputEmpty>[] data) { builder.StartVector(4, data.Length, 4); builder.Add(data); return builder.EndVector(); }
  public static void StartInputsEmptyVector(FlatBufferBuilder builder, int numElems) { builder.StartVector(4, numElems, 4); }
  public static Offset<messages.Inputs> EndInputs(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<messages.Inputs>(o);
  }
};


}

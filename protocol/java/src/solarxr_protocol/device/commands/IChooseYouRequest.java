// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.device.commands;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class IChooseYouRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static IChooseYouRequest getRootAsIChooseYouRequest(ByteBuffer _bb) { return getRootAsIChooseYouRequest(_bb, new IChooseYouRequest()); }
  public static IChooseYouRequest getRootAsIChooseYouRequest(ByteBuffer _bb, IChooseYouRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public IChooseYouRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startIChooseYouRequest(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endIChooseYouRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public IChooseYouRequest get(int j) { return get(new IChooseYouRequest(), j); }
    public IChooseYouRequest get(IChooseYouRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public IChooseYouRequestT unpack() {
    IChooseYouRequestT _o = new IChooseYouRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(IChooseYouRequestT _o) {
  }
  public static int pack(FlatBufferBuilder builder, IChooseYouRequestT _o) {
    if (_o == null) return 0;
    startIChooseYouRequest(builder);
    return endIChooseYouRequest(builder);
  }
}


// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class SettingsResetRequest implements flatbuffers.IUnpackableObject<SettingsResetRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):SettingsResetRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsSettingsResetRequest(bb:flatbuffers.ByteBuffer, obj?:SettingsResetRequest):SettingsResetRequest {
  return (obj || new SettingsResetRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsSettingsResetRequest(bb:flatbuffers.ByteBuffer, obj?:SettingsResetRequest):SettingsResetRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new SettingsResetRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startSettingsResetRequest(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endSettingsResetRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createSettingsResetRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  SettingsResetRequest.startSettingsResetRequest(builder);
  return SettingsResetRequest.endSettingsResetRequest(builder);
}

unpack(): SettingsResetRequestT {
  return new SettingsResetRequestT();
}


unpackTo(_o: SettingsResetRequestT): void {}
}

export class SettingsResetRequestT implements flatbuffers.IGeneratedObject {
constructor(){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return SettingsResetRequest.createSettingsResetRequest(builder);
}
}

// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class LetMeInRequest {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):LetMeInRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsLetMeInRequest(bb:flatbuffers.ByteBuffer, obj?:LetMeInRequest):LetMeInRequest {
  return (obj || new LetMeInRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsLetMeInRequest(bb:flatbuffers.ByteBuffer, obj?:LetMeInRequest):LetMeInRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new LetMeInRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startLetMeInRequest(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endLetMeInRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createLetMeInRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  LetMeInRequest.startLetMeInRequest(builder);
  return LetMeInRequest.endLetMeInRequest(builder);
}

unpack(): LetMeInRequestT {
  return new LetMeInRequestT();
}


unpackTo(_o: LetMeInRequestT): void {}
}

export class LetMeInRequestT {
constructor(){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return LetMeInRequest.createLetMeInRequest(builder);
}
}

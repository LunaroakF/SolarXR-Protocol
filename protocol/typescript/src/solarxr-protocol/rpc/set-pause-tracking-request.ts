// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class SetPauseTrackingRequest implements flatbuffers.IUnpackableObject<SetPauseTrackingRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):SetPauseTrackingRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsSetPauseTrackingRequest(bb:flatbuffers.ByteBuffer, obj?:SetPauseTrackingRequest):SetPauseTrackingRequest {
  return (obj || new SetPauseTrackingRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsSetPauseTrackingRequest(bb:flatbuffers.ByteBuffer, obj?:SetPauseTrackingRequest):SetPauseTrackingRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new SetPauseTrackingRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * Pauses skeleton tracking if true, resumes skeleton tracking if false.
 */
pauseTracking():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

static startSetPauseTrackingRequest(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addPauseTracking(builder:flatbuffers.Builder, pauseTracking:boolean) {
  builder.addFieldInt8(0, +pauseTracking, +false);
}

static endSetPauseTrackingRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createSetPauseTrackingRequest(builder:flatbuffers.Builder, pauseTracking:boolean):flatbuffers.Offset {
  SetPauseTrackingRequest.startSetPauseTrackingRequest(builder);
  SetPauseTrackingRequest.addPauseTracking(builder, pauseTracking);
  return SetPauseTrackingRequest.endSetPauseTrackingRequest(builder);
}

unpack(): SetPauseTrackingRequestT {
  return new SetPauseTrackingRequestT(
    this.pauseTracking()
  );
}


unpackTo(_o: SetPauseTrackingRequestT): void {
  _o.pauseTracking = this.pauseTracking();
}
}

export class SetPauseTrackingRequestT implements flatbuffers.IGeneratedObject {
constructor(
  public pauseTracking: boolean = false
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return SetPauseTrackingRequest.createSetPauseTrackingRequest(builder,
    this.pauseTracking
  );
}
}

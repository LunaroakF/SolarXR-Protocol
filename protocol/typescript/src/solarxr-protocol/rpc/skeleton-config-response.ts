// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { SkeletonPart, SkeletonPartT } from '../../solarxr-protocol/rpc/skeleton-part.js';


export class SkeletonConfigResponse implements flatbuffers.IUnpackableObject<SkeletonConfigResponseT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):SkeletonConfigResponse {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsSkeletonConfigResponse(bb:flatbuffers.ByteBuffer, obj?:SkeletonConfigResponse):SkeletonConfigResponse {
  return (obj || new SkeletonConfigResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsSkeletonConfigResponse(bb:flatbuffers.ByteBuffer, obj?:SkeletonConfigResponse):SkeletonConfigResponse {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new SkeletonConfigResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

skeletonParts(index: number, obj?:SkeletonPart):SkeletonPart|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new SkeletonPart()).__init(this.bb!.__indirect(this.bb!.__vector(this.bb_pos + offset) + index * 4), this.bb!) : null;
}

skeletonPartsLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

userHeight():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

static startSkeletonConfigResponse(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addSkeletonParts(builder:flatbuffers.Builder, skeletonPartsOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, skeletonPartsOffset, 0);
}

static createSkeletonPartsVector(builder:flatbuffers.Builder, data:flatbuffers.Offset[]):flatbuffers.Offset {
  builder.startVector(4, data.length, 4);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addOffset(data[i]!);
  }
  return builder.endVector();
}

static startSkeletonPartsVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(4, numElems, 4);
}

static addUserHeight(builder:flatbuffers.Builder, userHeight:number) {
  builder.addFieldFloat32(1, userHeight, 0.0);
}

static endSkeletonConfigResponse(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createSkeletonConfigResponse(builder:flatbuffers.Builder, skeletonPartsOffset:flatbuffers.Offset, userHeight:number):flatbuffers.Offset {
  SkeletonConfigResponse.startSkeletonConfigResponse(builder);
  SkeletonConfigResponse.addSkeletonParts(builder, skeletonPartsOffset);
  SkeletonConfigResponse.addUserHeight(builder, userHeight);
  return SkeletonConfigResponse.endSkeletonConfigResponse(builder);
}

unpack(): SkeletonConfigResponseT {
  return new SkeletonConfigResponseT(
    this.bb!.createObjList<SkeletonPart, SkeletonPartT>(this.skeletonParts.bind(this), this.skeletonPartsLength()),
    this.userHeight()
  );
}


unpackTo(_o: SkeletonConfigResponseT): void {
  _o.skeletonParts = this.bb!.createObjList<SkeletonPart, SkeletonPartT>(this.skeletonParts.bind(this), this.skeletonPartsLength());
  _o.userHeight = this.userHeight();
}
}

export class SkeletonConfigResponseT implements flatbuffers.IGeneratedObject {
constructor(
  public skeletonParts: (SkeletonPartT)[] = [],
  public userHeight: number = 0.0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const skeletonParts = SkeletonConfigResponse.createSkeletonPartsVector(builder, builder.createObjectOffsetList(this.skeletonParts));

  return SkeletonConfigResponse.createSkeletonConfigResponse(builder,
    skeletonParts,
    this.userHeight
  );
}
}

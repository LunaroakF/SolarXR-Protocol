// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



/**
 * Holds the Server information, this is a basic table holding various information about the currently running server
 * like its local ip address (useful for standalone users so they can specify the ip of the server more easily) and any more
 * infos we might want to add in the future. (like java version, working dir, server version ....)
 * This only holds the local ip for now. But there will be other information added as we chose to display them on the gui for instance
 */
export class ServerInfosResponse implements flatbuffers.IUnpackableObject<ServerInfosResponseT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ServerInfosResponse {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsServerInfosResponse(bb:flatbuffers.ByteBuffer, obj?:ServerInfosResponse):ServerInfosResponse {
  return (obj || new ServerInfosResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsServerInfosResponse(bb:flatbuffers.ByteBuffer, obj?:ServerInfosResponse):ServerInfosResponse {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ServerInfosResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

localIp():string|null
localIp(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
localIp(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startServerInfosResponse(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addLocalIp(builder:flatbuffers.Builder, localIpOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, localIpOffset, 0);
}

static endServerInfosResponse(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createServerInfosResponse(builder:flatbuffers.Builder, localIpOffset:flatbuffers.Offset):flatbuffers.Offset {
  ServerInfosResponse.startServerInfosResponse(builder);
  ServerInfosResponse.addLocalIp(builder, localIpOffset);
  return ServerInfosResponse.endServerInfosResponse(builder);
}

unpack(): ServerInfosResponseT {
  return new ServerInfosResponseT(
    this.localIp()
  );
}


unpackTo(_o: ServerInfosResponseT): void {
  _o.localIp = this.localIp();
}
}

export class ServerInfosResponseT implements flatbuffers.IGeneratedObject {
constructor(
  public localIp: string|Uint8Array|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const localIp = (this.localIp !== null ? builder.createString(this.localIp!) : 0);

  return ServerInfosResponse.createServerInfosResponse(builder,
    localIp
  );
}
}

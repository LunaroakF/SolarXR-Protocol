// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc.settings;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Settings for the skeletal model that are toggles.
 */
@SuppressWarnings("unused")
public final class ModelToggles extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static ModelToggles getRootAsModelToggles(ByteBuffer _bb) { return getRootAsModelToggles(_bb, new ModelToggles()); }
  public static ModelToggles getRootAsModelToggles(ByteBuffer _bb, ModelToggles obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ModelToggles __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public boolean hasExtendedSpine() { return 0 != __offset(4); }
  public boolean extendedSpine() { int o = __offset(4); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasExtendedPelvis() { return 0 != __offset(6); }
  public boolean extendedPelvis() { int o = __offset(6); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasExtendedKnee() { return 0 != __offset(8); }
  public boolean extendedKnee() { int o = __offset(8); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasForceArmsFromHmd() { return 0 != __offset(10); }
  public boolean forceArmsFromHmd() { int o = __offset(10); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasFloorClip() { return 0 != __offset(12); }
  public boolean floorClip() { int o = __offset(12); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasSkatingCorrection() { return 0 != __offset(14); }
  public boolean skatingCorrection() { int o = __offset(14); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasViveEmulation() { return 0 != __offset(16); }
  public boolean viveEmulation() { int o = __offset(16); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasToeSnap() { return 0 != __offset(18); }
  public boolean toeSnap() { int o = __offset(18); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasFootPlant() { return 0 != __offset(20); }
  public boolean footPlant() { int o = __offset(20); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasSelfLocalization() { return 0 != __offset(22); }
  public boolean selfLocalization() { int o = __offset(22); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasUsePosition() { return 0 != __offset(24); }
  public boolean usePosition() { int o = __offset(24); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasEnforceConstraints() { return 0 != __offset(26); }
  public boolean enforceConstraints() { int o = __offset(26); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean hasCorrectConstraints() { return 0 != __offset(28); }
  public boolean correctConstraints() { int o = __offset(28); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }

  public static int createModelToggles(FlatBufferBuilder builder,
      boolean extendedSpine,
      boolean extendedPelvis,
      boolean extendedKnee,
      boolean forceArmsFromHmd,
      boolean floorClip,
      boolean skatingCorrection,
      boolean viveEmulation,
      boolean toeSnap,
      boolean footPlant,
      boolean selfLocalization,
      boolean usePosition,
      boolean enforceConstraints,
      boolean correctConstraints) {
    builder.startTable(13);
    ModelToggles.addCorrectConstraints(builder, correctConstraints);
    ModelToggles.addEnforceConstraints(builder, enforceConstraints);
    ModelToggles.addUsePosition(builder, usePosition);
    ModelToggles.addSelfLocalization(builder, selfLocalization);
    ModelToggles.addFootPlant(builder, footPlant);
    ModelToggles.addToeSnap(builder, toeSnap);
    ModelToggles.addViveEmulation(builder, viveEmulation);
    ModelToggles.addSkatingCorrection(builder, skatingCorrection);
    ModelToggles.addFloorClip(builder, floorClip);
    ModelToggles.addForceArmsFromHmd(builder, forceArmsFromHmd);
    ModelToggles.addExtendedKnee(builder, extendedKnee);
    ModelToggles.addExtendedPelvis(builder, extendedPelvis);
    ModelToggles.addExtendedSpine(builder, extendedSpine);
    return ModelToggles.endModelToggles(builder);
  }

  public static void startModelToggles(FlatBufferBuilder builder) { builder.startTable(13); }
  public static void addExtendedSpine(FlatBufferBuilder builder, boolean extendedSpine) { builder.addBoolean(0, extendedSpine, false); }
  public static void addExtendedPelvis(FlatBufferBuilder builder, boolean extendedPelvis) { builder.addBoolean(1, extendedPelvis, false); }
  public static void addExtendedKnee(FlatBufferBuilder builder, boolean extendedKnee) { builder.addBoolean(2, extendedKnee, false); }
  public static void addForceArmsFromHmd(FlatBufferBuilder builder, boolean forceArmsFromHmd) { builder.addBoolean(3, forceArmsFromHmd, false); }
  public static void addFloorClip(FlatBufferBuilder builder, boolean floorClip) { builder.addBoolean(4, floorClip, false); }
  public static void addSkatingCorrection(FlatBufferBuilder builder, boolean skatingCorrection) { builder.addBoolean(5, skatingCorrection, false); }
  public static void addViveEmulation(FlatBufferBuilder builder, boolean viveEmulation) { builder.addBoolean(6, viveEmulation, false); }
  public static void addToeSnap(FlatBufferBuilder builder, boolean toeSnap) { builder.addBoolean(7, toeSnap, false); }
  public static void addFootPlant(FlatBufferBuilder builder, boolean footPlant) { builder.addBoolean(8, footPlant, false); }
  public static void addSelfLocalization(FlatBufferBuilder builder, boolean selfLocalization) { builder.addBoolean(9, selfLocalization, false); }
  public static void addUsePosition(FlatBufferBuilder builder, boolean usePosition) { builder.addBoolean(10, usePosition, false); }
  public static void addEnforceConstraints(FlatBufferBuilder builder, boolean enforceConstraints) { builder.addBoolean(11, enforceConstraints, false); }
  public static void addCorrectConstraints(FlatBufferBuilder builder, boolean correctConstraints) { builder.addBoolean(12, correctConstraints, false); }
  public static int endModelToggles(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ModelToggles get(int j) { return get(new ModelToggles(), j); }
    public ModelToggles get(ModelToggles obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ModelTogglesT unpack() {
    ModelTogglesT _o = new ModelTogglesT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ModelTogglesT _o) {
    Boolean _oExtendedSpine = hasExtendedSpine() ? extendedSpine() : null;
    _o.setExtendedSpine(_oExtendedSpine);
    Boolean _oExtendedPelvis = hasExtendedPelvis() ? extendedPelvis() : null;
    _o.setExtendedPelvis(_oExtendedPelvis);
    Boolean _oExtendedKnee = hasExtendedKnee() ? extendedKnee() : null;
    _o.setExtendedKnee(_oExtendedKnee);
    Boolean _oForceArmsFromHmd = hasForceArmsFromHmd() ? forceArmsFromHmd() : null;
    _o.setForceArmsFromHmd(_oForceArmsFromHmd);
    Boolean _oFloorClip = hasFloorClip() ? floorClip() : null;
    _o.setFloorClip(_oFloorClip);
    Boolean _oSkatingCorrection = hasSkatingCorrection() ? skatingCorrection() : null;
    _o.setSkatingCorrection(_oSkatingCorrection);
    Boolean _oViveEmulation = hasViveEmulation() ? viveEmulation() : null;
    _o.setViveEmulation(_oViveEmulation);
    Boolean _oToeSnap = hasToeSnap() ? toeSnap() : null;
    _o.setToeSnap(_oToeSnap);
    Boolean _oFootPlant = hasFootPlant() ? footPlant() : null;
    _o.setFootPlant(_oFootPlant);
    Boolean _oSelfLocalization = hasSelfLocalization() ? selfLocalization() : null;
    _o.setSelfLocalization(_oSelfLocalization);
    Boolean _oUsePosition = hasUsePosition() ? usePosition() : null;
    _o.setUsePosition(_oUsePosition);
    Boolean _oEnforceConstraints = hasEnforceConstraints() ? enforceConstraints() : null;
    _o.setEnforceConstraints(_oEnforceConstraints);
    Boolean _oCorrectConstraints = hasCorrectConstraints() ? correctConstraints() : null;
    _o.setCorrectConstraints(_oCorrectConstraints);
  }
  public static int pack(FlatBufferBuilder builder, ModelTogglesT _o) {
    if (_o == null) return 0;
    return createModelToggles(
      builder,
      _o.getExtendedSpine(),
      _o.getExtendedPelvis(),
      _o.getExtendedKnee(),
      _o.getForceArmsFromHmd(),
      _o.getFloorClip(),
      _o.getSkatingCorrection(),
      _o.getViveEmulation(),
      _o.getToeSnap(),
      _o.getFootPlant(),
      _o.getSelfLocalization(),
      _o.getUsePosition(),
      _o.getEnforceConstraints(),
      _o.getCorrectConstraints());
  }
}


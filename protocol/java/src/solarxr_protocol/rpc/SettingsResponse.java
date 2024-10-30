// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class SettingsResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static SettingsResponse getRootAsSettingsResponse(ByteBuffer _bb) { return getRootAsSettingsResponse(_bb, new SettingsResponse()); }
  public static SettingsResponse getRootAsSettingsResponse(ByteBuffer _bb, SettingsResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SettingsResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers() { return steamVrTrackers(new solarxr_protocol.rpc.SteamVRTrackersSetting()); }
  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers(solarxr_protocol.rpc.SteamVRTrackersSetting obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.FilteringSettings filtering() { return filtering(new solarxr_protocol.rpc.FilteringSettings()); }
  public solarxr_protocol.rpc.FilteringSettings filtering(solarxr_protocol.rpc.FilteringSettings obj) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.DriftCompensationSettings driftCompensation() { return driftCompensation(new solarxr_protocol.rpc.DriftCompensationSettings()); }
  public solarxr_protocol.rpc.DriftCompensationSettings driftCompensation(solarxr_protocol.rpc.DriftCompensationSettings obj) { int o = __offset(8); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.OSCRouterSettings oscRouter() { return oscRouter(new solarxr_protocol.rpc.OSCRouterSettings()); }
  public solarxr_protocol.rpc.OSCRouterSettings oscRouter(solarxr_protocol.rpc.OSCRouterSettings obj) { int o = __offset(10); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.VRCOSCSettings vrcOsc() { return vrcOsc(new solarxr_protocol.rpc.VRCOSCSettings()); }
  public solarxr_protocol.rpc.VRCOSCSettings vrcOsc(solarxr_protocol.rpc.VRCOSCSettings obj) { int o = __offset(12); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.VMCOSCSettings vmcOsc() { return vmcOsc(new solarxr_protocol.rpc.VMCOSCSettings()); }
  public solarxr_protocol.rpc.VMCOSCSettings vmcOsc(solarxr_protocol.rpc.VMCOSCSettings obj) { int o = __offset(14); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.settings.ModelSettings modelSettings() { return modelSettings(new solarxr_protocol.rpc.settings.ModelSettings()); }
  public solarxr_protocol.rpc.settings.ModelSettings modelSettings(solarxr_protocol.rpc.settings.ModelSettings obj) { int o = __offset(16); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.TapDetectionSettings tapDetectionSettings() { return tapDetectionSettings(new solarxr_protocol.rpc.TapDetectionSettings()); }
  public solarxr_protocol.rpc.TapDetectionSettings tapDetectionSettings(solarxr_protocol.rpc.TapDetectionSettings obj) { int o = __offset(18); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.AutoBoneSettings autoBoneSettings() { return autoBoneSettings(new solarxr_protocol.rpc.AutoBoneSettings()); }
  public solarxr_protocol.rpc.AutoBoneSettings autoBoneSettings(solarxr_protocol.rpc.AutoBoneSettings obj) { int o = __offset(20); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.ResetsSettings resetsSettings() { return resetsSettings(new solarxr_protocol.rpc.ResetsSettings()); }
  public solarxr_protocol.rpc.ResetsSettings resetsSettings(solarxr_protocol.rpc.ResetsSettings obj) { int o = __offset(22); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }

  public static int createSettingsResponse(FlatBufferBuilder builder,
      int steamVrTrackersOffset,
      int filteringOffset,
      int driftCompensationOffset,
      int oscRouterOffset,
      int vrcOscOffset,
      int vmcOscOffset,
      int modelSettingsOffset,
      int tapDetectionSettingsOffset,
      int autoBoneSettingsOffset,
      int resetsSettingsOffset) {
    builder.startTable(10);
    SettingsResponse.addResetsSettings(builder, resetsSettingsOffset);
    SettingsResponse.addAutoBoneSettings(builder, autoBoneSettingsOffset);
    SettingsResponse.addTapDetectionSettings(builder, tapDetectionSettingsOffset);
    SettingsResponse.addModelSettings(builder, modelSettingsOffset);
    SettingsResponse.addVmcOsc(builder, vmcOscOffset);
    SettingsResponse.addVrcOsc(builder, vrcOscOffset);
    SettingsResponse.addOscRouter(builder, oscRouterOffset);
    SettingsResponse.addDriftCompensation(builder, driftCompensationOffset);
    SettingsResponse.addFiltering(builder, filteringOffset);
    SettingsResponse.addSteamVrTrackers(builder, steamVrTrackersOffset);
    return SettingsResponse.endSettingsResponse(builder);
  }

  public static void startSettingsResponse(FlatBufferBuilder builder) { builder.startTable(10); }
  public static void addSteamVrTrackers(FlatBufferBuilder builder, int steamVrTrackersOffset) { builder.addOffset(0, steamVrTrackersOffset, 0); }
  public static void addFiltering(FlatBufferBuilder builder, int filteringOffset) { builder.addOffset(1, filteringOffset, 0); }
  public static void addDriftCompensation(FlatBufferBuilder builder, int driftCompensationOffset) { builder.addOffset(2, driftCompensationOffset, 0); }
  public static void addOscRouter(FlatBufferBuilder builder, int oscRouterOffset) { builder.addOffset(3, oscRouterOffset, 0); }
  public static void addVrcOsc(FlatBufferBuilder builder, int vrcOscOffset) { builder.addOffset(4, vrcOscOffset, 0); }
  public static void addVmcOsc(FlatBufferBuilder builder, int vmcOscOffset) { builder.addOffset(5, vmcOscOffset, 0); }
  public static void addModelSettings(FlatBufferBuilder builder, int modelSettingsOffset) { builder.addOffset(6, modelSettingsOffset, 0); }
  public static void addTapDetectionSettings(FlatBufferBuilder builder, int tapDetectionSettingsOffset) { builder.addOffset(7, tapDetectionSettingsOffset, 0); }
  public static void addAutoBoneSettings(FlatBufferBuilder builder, int autoBoneSettingsOffset) { builder.addOffset(8, autoBoneSettingsOffset, 0); }
  public static void addResetsSettings(FlatBufferBuilder builder, int resetsSettingsOffset) { builder.addOffset(9, resetsSettingsOffset, 0); }
  public static int endSettingsResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SettingsResponse get(int j) { return get(new SettingsResponse(), j); }
    public SettingsResponse get(SettingsResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SettingsResponseT unpack() {
    SettingsResponseT _o = new SettingsResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SettingsResponseT _o) {
    if (steamVrTrackers() != null) _o.setSteamVrTrackers(steamVrTrackers().unpack());
    else _o.setSteamVrTrackers(null);
    if (filtering() != null) _o.setFiltering(filtering().unpack());
    else _o.setFiltering(null);
    if (driftCompensation() != null) _o.setDriftCompensation(driftCompensation().unpack());
    else _o.setDriftCompensation(null);
    if (oscRouter() != null) _o.setOscRouter(oscRouter().unpack());
    else _o.setOscRouter(null);
    if (vrcOsc() != null) _o.setVrcOsc(vrcOsc().unpack());
    else _o.setVrcOsc(null);
    if (vmcOsc() != null) _o.setVmcOsc(vmcOsc().unpack());
    else _o.setVmcOsc(null);
    if (modelSettings() != null) _o.setModelSettings(modelSettings().unpack());
    else _o.setModelSettings(null);
    if (tapDetectionSettings() != null) _o.setTapDetectionSettings(tapDetectionSettings().unpack());
    else _o.setTapDetectionSettings(null);
    if (autoBoneSettings() != null) _o.setAutoBoneSettings(autoBoneSettings().unpack());
    else _o.setAutoBoneSettings(null);
    if (resetsSettings() != null) _o.setResetsSettings(resetsSettings().unpack());
    else _o.setResetsSettings(null);
  }
  public static int pack(FlatBufferBuilder builder, SettingsResponseT _o) {
    if (_o == null) return 0;
    int _steamVrTrackers = _o.getSteamVrTrackers() == null ? 0 : solarxr_protocol.rpc.SteamVRTrackersSetting.pack(builder, _o.getSteamVrTrackers());
    int _filtering = _o.getFiltering() == null ? 0 : solarxr_protocol.rpc.FilteringSettings.pack(builder, _o.getFiltering());
    int _driftCompensation = _o.getDriftCompensation() == null ? 0 : solarxr_protocol.rpc.DriftCompensationSettings.pack(builder, _o.getDriftCompensation());
    int _oscRouter = _o.getOscRouter() == null ? 0 : solarxr_protocol.rpc.OSCRouterSettings.pack(builder, _o.getOscRouter());
    int _vrcOsc = _o.getVrcOsc() == null ? 0 : solarxr_protocol.rpc.VRCOSCSettings.pack(builder, _o.getVrcOsc());
    int _vmcOsc = _o.getVmcOsc() == null ? 0 : solarxr_protocol.rpc.VMCOSCSettings.pack(builder, _o.getVmcOsc());
    int _modelSettings = _o.getModelSettings() == null ? 0 : solarxr_protocol.rpc.settings.ModelSettings.pack(builder, _o.getModelSettings());
    int _tapDetectionSettings = _o.getTapDetectionSettings() == null ? 0 : solarxr_protocol.rpc.TapDetectionSettings.pack(builder, _o.getTapDetectionSettings());
    int _autoBoneSettings = _o.getAutoBoneSettings() == null ? 0 : solarxr_protocol.rpc.AutoBoneSettings.pack(builder, _o.getAutoBoneSettings());
    int _resetsSettings = _o.getResetsSettings() == null ? 0 : solarxr_protocol.rpc.ResetsSettings.pack(builder, _o.getResetsSettings());
    return createSettingsResponse(
      builder,
      _steamVrTrackers,
      _filtering,
      _driftCompensation,
      _oscRouter,
      _vrcOsc,
      _vmcOsc,
      _modelSettings,
      _tapDetectionSettings,
      _autoBoneSettings,
      _resetsSettings);
  }
}


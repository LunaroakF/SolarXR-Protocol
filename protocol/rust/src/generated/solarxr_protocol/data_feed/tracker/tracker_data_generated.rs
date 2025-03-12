// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum TrackerDataOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Describes all possible information about a tracker. A tracker is anything that
/// provides kinematic data about a particular body part.
///
/// Trackers may be synthetic/computed or instead part of an actual hardware device.
/// There can be multiple trackers per hardware device.
pub struct TrackerData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrackerData<'a> {
  type Inner = TrackerData<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TrackerData<'a> {
  pub const VT_TRACKER_ID: flatbuffers::VOffsetT = 4;
  pub const VT_INFO: flatbuffers::VOffsetT = 6;
  pub const VT_STATUS: flatbuffers::VOffsetT = 8;
  pub const VT_PACKETERRORCODE: flatbuffers::VOffsetT = 10;
  pub const VT_ROTATION: flatbuffers::VOffsetT = 12;
  pub const VT_POSITION: flatbuffers::VOffsetT = 14;
  pub const VT_RAW_ANGULAR_VELOCITY: flatbuffers::VOffsetT = 16;
  pub const VT_RAW_ACCELERATION: flatbuffers::VOffsetT = 18;
  pub const VT_TEMP: flatbuffers::VOffsetT = 20;
  pub const VT_LINEAR_ACCELERATION: flatbuffers::VOffsetT = 22;
  pub const VT_ROTATION_REFERENCE_ADJUSTED: flatbuffers::VOffsetT = 24;
  pub const VT_ROTATION_IDENTITY_ADJUSTED: flatbuffers::VOffsetT = 26;
  pub const VT_TPS: flatbuffers::VOffsetT = 28;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TrackerData { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TrackerDataArgs<'args>
  ) -> flatbuffers::WIPOffset<TrackerData<'bldr>> {
    let mut builder = TrackerDataBuilder::new(_fbb);
    if let Some(x) = args.rotation_identity_adjusted { builder.add_rotation_identity_adjusted(x); }
    if let Some(x) = args.rotation_reference_adjusted { builder.add_rotation_reference_adjusted(x); }
    if let Some(x) = args.linear_acceleration { builder.add_linear_acceleration(x); }
    if let Some(x) = args.temp { builder.add_temp(x); }
    if let Some(x) = args.raw_acceleration { builder.add_raw_acceleration(x); }
    if let Some(x) = args.raw_angular_velocity { builder.add_raw_angular_velocity(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.rotation { builder.add_rotation(x); }
    if let Some(x) = args.info { builder.add_info(x); }
    if let Some(x) = args.tracker_id { builder.add_tracker_id(x); }
    if let Some(x) = args.tps { builder.add_tps(x); }
    builder.add_packetErrorCode(args.packetErrorCode);
    builder.add_status(args.status);
    builder.finish()
  }


  #[inline]
  pub fn tracker_id(&self) -> Option<super::super::datatypes::TrackerId<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<super::super::datatypes::TrackerId>>(TrackerData::VT_TRACKER_ID, None)}
  }
  #[inline]
  pub fn info(&self) -> Option<TrackerInfo<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<TrackerInfo>>(TrackerData::VT_INFO, None)}
  }
  #[inline]
  pub fn status(&self) -> super::super::datatypes::TrackerStatus {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::TrackerStatus>(TrackerData::VT_STATUS, Some(super::super::datatypes::TrackerStatus::NONE)).unwrap()}
  }
  #[inline]
  pub fn packetErrorCode(&self) -> super::super::datatypes::PacketErrorCode {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::PacketErrorCode>(TrackerData::VT_PACKETERRORCODE, Some(super::super::datatypes::PacketErrorCode::NOT_APPLICABLE)).unwrap()}
  }
  /// Sensor rotation after fusion
  #[inline]
  pub fn rotation(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION, None)}
  }
  /// Position, in meters
  #[inline]
  pub fn position(&self) -> Option<&'a super::super::datatypes::math::Vec3f> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Vec3f>(TrackerData::VT_POSITION, None)}
  }
  /// Raw angular velocity, in euler angles, rad/s
  #[inline]
  pub fn raw_angular_velocity(&self) -> Option<&'a super::super::datatypes::math::Vec3f> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Vec3f>(TrackerData::VT_RAW_ANGULAR_VELOCITY, None)}
  }
  /// Raw acceleration, in m/s^2
  #[inline]
  pub fn raw_acceleration(&self) -> Option<&'a super::super::datatypes::math::Vec3f> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Vec3f>(TrackerData::VT_RAW_ACCELERATION, None)}
  }
  /// Temperature, in degrees celsius
  #[inline]
  pub fn temp(&self) -> Option<&'a super::super::datatypes::Temperature> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::Temperature>(TrackerData::VT_TEMP, None)}
  }
  /// Acceleration without gravity, in m/s^2
  #[inline]
  pub fn linear_acceleration(&self) -> Option<&'a super::super::datatypes::math::Vec3f> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Vec3f>(TrackerData::VT_LINEAR_ACCELERATION, None)}
  }
  /// Reference-adjusted rotation for IMU-only trackers (VR HMD yaw is used as a reset reference).
  /// In other words, a rotation that is aligned to a reliable source of rotation ((0, VR HMD YAW, 0)),
  /// triggered after user input (using reset buttons).
  /// This is a SlimeVR-specific field and computed exclusively by SlimeVR server.
  /// Includes: mounting orientation, full, quick and mounting reset adjustments.
  /// This rotation can be used to reconstruct a skeleton pose using forward kinematics.
  #[inline]
  pub fn rotation_reference_adjusted(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION_REFERENCE_ADJUSTED, None)}
  }
  /// Zero-reference-adjusted rotation for IMU-only trackers (identity quaternion is used as a reset reference).
  /// In other words, a rotation that is aligned to a zero vector ((0, 0, 0)) by
  /// inverting the current rotation, triggered after user input (using reset buttons).
  /// This is a SlimeVR-specific field and computed exclusively by SlimeVR server.
  /// Includes: only full and quick reset adjustments.
  /// This rotation can be used in visualizations for IMU debugging.
  #[inline]
  pub fn rotation_identity_adjusted(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION_IDENTITY_ADJUSTED, None)}
  }
  /// Data ticks per second, processed by SlimeVR server
  #[inline]
  pub fn tps(&self) -> Option<u16> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(TrackerData::VT_TPS, None)}
  }
}

impl flatbuffers::Verifiable for TrackerData<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::super::datatypes::TrackerId>>("tracker_id", Self::VT_TRACKER_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<TrackerInfo>>("info", Self::VT_INFO, false)?
     .visit_field::<super::super::datatypes::TrackerStatus>("status", Self::VT_STATUS, false)?
     .visit_field::<super::super::datatypes::PacketErrorCode>("packetErrorCode", Self::VT_PACKETERRORCODE, false)?
     .visit_field::<super::super::datatypes::math::Quat>("rotation", Self::VT_ROTATION, false)?
     .visit_field::<super::super::datatypes::math::Vec3f>("position", Self::VT_POSITION, false)?
     .visit_field::<super::super::datatypes::math::Vec3f>("raw_angular_velocity", Self::VT_RAW_ANGULAR_VELOCITY, false)?
     .visit_field::<super::super::datatypes::math::Vec3f>("raw_acceleration", Self::VT_RAW_ACCELERATION, false)?
     .visit_field::<super::super::datatypes::Temperature>("temp", Self::VT_TEMP, false)?
     .visit_field::<super::super::datatypes::math::Vec3f>("linear_acceleration", Self::VT_LINEAR_ACCELERATION, false)?
     .visit_field::<super::super::datatypes::math::Quat>("rotation_reference_adjusted", Self::VT_ROTATION_REFERENCE_ADJUSTED, false)?
     .visit_field::<super::super::datatypes::math::Quat>("rotation_identity_adjusted", Self::VT_ROTATION_IDENTITY_ADJUSTED, false)?
     .visit_field::<u16>("tps", Self::VT_TPS, false)?
     .finish();
    Ok(())
  }
}
pub struct TrackerDataArgs<'a> {
    pub tracker_id: Option<flatbuffers::WIPOffset<super::super::datatypes::TrackerId<'a>>>,
    pub info: Option<flatbuffers::WIPOffset<TrackerInfo<'a>>>,
    pub status: super::super::datatypes::TrackerStatus,
    pub packetErrorCode: super::super::datatypes::PacketErrorCode,
    pub rotation: Option<&'a super::super::datatypes::math::Quat>,
    pub position: Option<&'a super::super::datatypes::math::Vec3f>,
    pub raw_angular_velocity: Option<&'a super::super::datatypes::math::Vec3f>,
    pub raw_acceleration: Option<&'a super::super::datatypes::math::Vec3f>,
    pub temp: Option<&'a super::super::datatypes::Temperature>,
    pub linear_acceleration: Option<&'a super::super::datatypes::math::Vec3f>,
    pub rotation_reference_adjusted: Option<&'a super::super::datatypes::math::Quat>,
    pub rotation_identity_adjusted: Option<&'a super::super::datatypes::math::Quat>,
    pub tps: Option<u16>,
}
impl<'a> Default for TrackerDataArgs<'a> {
  #[inline]
  fn default() -> Self {
    TrackerDataArgs {
      tracker_id: None,
      info: None,
      status: super::super::datatypes::TrackerStatus::NONE,
      packetErrorCode: super::super::datatypes::PacketErrorCode::NOT_APPLICABLE,
      rotation: None,
      position: None,
      raw_angular_velocity: None,
      raw_acceleration: None,
      temp: None,
      linear_acceleration: None,
      rotation_reference_adjusted: None,
      rotation_identity_adjusted: None,
      tps: None,
    }
  }
}

pub struct TrackerDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TrackerDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_tracker_id(&mut self, tracker_id: flatbuffers::WIPOffset<super::super::datatypes::TrackerId<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::super::datatypes::TrackerId>>(TrackerData::VT_TRACKER_ID, tracker_id);
  }
  #[inline]
  pub fn add_info(&mut self, info: flatbuffers::WIPOffset<TrackerInfo<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<TrackerInfo>>(TrackerData::VT_INFO, info);
  }
  #[inline]
  pub fn add_status(&mut self, status: super::super::datatypes::TrackerStatus) {
    self.fbb_.push_slot::<super::super::datatypes::TrackerStatus>(TrackerData::VT_STATUS, status, super::super::datatypes::TrackerStatus::NONE);
  }
  #[inline]
  pub fn add_packetErrorCode(&mut self, packetErrorCode: super::super::datatypes::PacketErrorCode) {
    self.fbb_.push_slot::<super::super::datatypes::PacketErrorCode>(TrackerData::VT_PACKETERRORCODE, packetErrorCode, super::super::datatypes::PacketErrorCode::NOT_APPLICABLE);
  }
  #[inline]
  pub fn add_rotation(&mut self, rotation: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION, rotation);
  }
  #[inline]
  pub fn add_position(&mut self, position: &super::super::datatypes::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Vec3f>(TrackerData::VT_POSITION, position);
  }
  #[inline]
  pub fn add_raw_angular_velocity(&mut self, raw_angular_velocity: &super::super::datatypes::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Vec3f>(TrackerData::VT_RAW_ANGULAR_VELOCITY, raw_angular_velocity);
  }
  #[inline]
  pub fn add_raw_acceleration(&mut self, raw_acceleration: &super::super::datatypes::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Vec3f>(TrackerData::VT_RAW_ACCELERATION, raw_acceleration);
  }
  #[inline]
  pub fn add_temp(&mut self, temp: &super::super::datatypes::Temperature) {
    self.fbb_.push_slot_always::<&super::super::datatypes::Temperature>(TrackerData::VT_TEMP, temp);
  }
  #[inline]
  pub fn add_linear_acceleration(&mut self, linear_acceleration: &super::super::datatypes::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Vec3f>(TrackerData::VT_LINEAR_ACCELERATION, linear_acceleration);
  }
  #[inline]
  pub fn add_rotation_reference_adjusted(&mut self, rotation_reference_adjusted: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION_REFERENCE_ADJUSTED, rotation_reference_adjusted);
  }
  #[inline]
  pub fn add_rotation_identity_adjusted(&mut self, rotation_identity_adjusted: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerData::VT_ROTATION_IDENTITY_ADJUSTED, rotation_identity_adjusted);
  }
  #[inline]
  pub fn add_tps(&mut self, tps: u16) {
    self.fbb_.push_slot_always::<u16>(TrackerData::VT_TPS, tps);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrackerDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TrackerDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TrackerData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TrackerData<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TrackerData");
      ds.field("tracker_id", &self.tracker_id());
      ds.field("info", &self.info());
      ds.field("status", &self.status());
      ds.field("packetErrorCode", &self.packetErrorCode());
      ds.field("rotation", &self.rotation());
      ds.field("position", &self.position());
      ds.field("raw_angular_velocity", &self.raw_angular_velocity());
      ds.field("raw_acceleration", &self.raw_acceleration());
      ds.field("temp", &self.temp());
      ds.field("linear_acceleration", &self.linear_acceleration());
      ds.field("rotation_reference_adjusted", &self.rotation_reference_adjusted());
      ds.field("rotation_identity_adjusted", &self.rotation_identity_adjusted());
      ds.field("tps", &self.tps());
      ds.finish()
  }
}

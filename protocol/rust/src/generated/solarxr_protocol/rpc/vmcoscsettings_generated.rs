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
pub enum VMCOSCSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

/// OSC Settings specific to VMC
pub struct VMCOSCSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for VMCOSCSettings<'a> {
  type Inner = VMCOSCSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> VMCOSCSettings<'a> {
  pub const VT_OSC_SETTINGS: flatbuffers::VOffsetT = 4;
  pub const VT_VRM_JSON: flatbuffers::VOffsetT = 6;
  pub const VT_ANCHOR_HIP: flatbuffers::VOffsetT = 8;
  pub const VT_MIRROR_TRACKING: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    VMCOSCSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args VMCOSCSettingsArgs<'args>
  ) -> flatbuffers::WIPOffset<VMCOSCSettings<'bldr>> {
    let mut builder = VMCOSCSettingsBuilder::new(_fbb);
    if let Some(x) = args.vrm_json { builder.add_vrm_json(x); }
    if let Some(x) = args.osc_settings { builder.add_osc_settings(x); }
    builder.add_mirror_tracking(args.mirror_tracking);
    builder.add_anchor_hip(args.anchor_hip);
    builder.finish()
  }


  #[inline]
  pub fn osc_settings(&self) -> Option<OSCSettings<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<OSCSettings>>(VMCOSCSettings::VT_OSC_SETTINGS, None)}
  }
  #[inline]
  pub fn vrm_json(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(VMCOSCSettings::VT_VRM_JSON, None)}
  }
  #[inline]
  pub fn anchor_hip(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VMCOSCSettings::VT_ANCHOR_HIP, Some(false)).unwrap()}
  }
  #[inline]
  pub fn mirror_tracking(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VMCOSCSettings::VT_MIRROR_TRACKING, Some(false)).unwrap()}
  }
}

impl flatbuffers::Verifiable for VMCOSCSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<OSCSettings>>("osc_settings", Self::VT_OSC_SETTINGS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("vrm_json", Self::VT_VRM_JSON, false)?
     .visit_field::<bool>("anchor_hip", Self::VT_ANCHOR_HIP, false)?
     .visit_field::<bool>("mirror_tracking", Self::VT_MIRROR_TRACKING, false)?
     .finish();
    Ok(())
  }
}
pub struct VMCOSCSettingsArgs<'a> {
    pub osc_settings: Option<flatbuffers::WIPOffset<OSCSettings<'a>>>,
    pub vrm_json: Option<flatbuffers::WIPOffset<&'a str>>,
    pub anchor_hip: bool,
    pub mirror_tracking: bool,
}
impl<'a> Default for VMCOSCSettingsArgs<'a> {
  #[inline]
  fn default() -> Self {
    VMCOSCSettingsArgs {
      osc_settings: None,
      vrm_json: None,
      anchor_hip: false,
      mirror_tracking: false,
    }
  }
}

pub struct VMCOSCSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> VMCOSCSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_osc_settings(&mut self, osc_settings: flatbuffers::WIPOffset<OSCSettings<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<OSCSettings>>(VMCOSCSettings::VT_OSC_SETTINGS, osc_settings);
  }
  #[inline]
  pub fn add_vrm_json(&mut self, vrm_json: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(VMCOSCSettings::VT_VRM_JSON, vrm_json);
  }
  #[inline]
  pub fn add_anchor_hip(&mut self, anchor_hip: bool) {
    self.fbb_.push_slot::<bool>(VMCOSCSettings::VT_ANCHOR_HIP, anchor_hip, false);
  }
  #[inline]
  pub fn add_mirror_tracking(&mut self, mirror_tracking: bool) {
    self.fbb_.push_slot::<bool>(VMCOSCSettings::VT_MIRROR_TRACKING, mirror_tracking, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> VMCOSCSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    VMCOSCSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<VMCOSCSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for VMCOSCSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("VMCOSCSettings");
      ds.field("osc_settings", &self.osc_settings());
      ds.field("vrm_json", &self.vrm_json());
      ds.field("anchor_hip", &self.anchor_hip());
      ds.field("mirror_tracking", &self.mirror_tracking());
      ds.finish()
  }
}

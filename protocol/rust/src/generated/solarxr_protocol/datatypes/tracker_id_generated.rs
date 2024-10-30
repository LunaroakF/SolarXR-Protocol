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
pub enum TrackerIdOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TrackerId<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrackerId<'a> {
  type Inner = TrackerId<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TrackerId<'a> {
  pub const VT_DEVICE_ID: flatbuffers::VOffsetT = 4;
  pub const VT_TRACKER_NUM: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TrackerId { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TrackerIdArgs<'args>
  ) -> flatbuffers::WIPOffset<TrackerId<'bldr>> {
    let mut builder = TrackerIdBuilder::new(_fbb);
    if let Some(x) = args.device_id { builder.add_device_id(x); }
    builder.add_tracker_num(args.tracker_num);
    builder.finish()
  }


  /// The device the tracker is associated with. If there is no hardware device it is
  /// associated with, this should be `null`.
  #[inline]
  pub fn device_id(&self) -> Option<&'a DeviceId> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<DeviceId>(TrackerId::VT_DEVICE_ID, None)}
  }
  /// There are possibly multiple trackers per device. This identifies which one.
  #[inline]
  pub fn tracker_num(&self) -> u8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u8>(TrackerId::VT_TRACKER_NUM, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TrackerId<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<DeviceId>("device_id", Self::VT_DEVICE_ID, false)?
     .visit_field::<u8>("tracker_num", Self::VT_TRACKER_NUM, false)?
     .finish();
    Ok(())
  }
}
pub struct TrackerIdArgs<'a> {
    pub device_id: Option<&'a DeviceId>,
    pub tracker_num: u8,
}
impl<'a> Default for TrackerIdArgs<'a> {
  #[inline]
  fn default() -> Self {
    TrackerIdArgs {
      device_id: None,
      tracker_num: 0,
    }
  }
}

pub struct TrackerIdBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TrackerIdBuilder<'a, 'b> {
  #[inline]
  pub fn add_device_id(&mut self, device_id: &DeviceId) {
    self.fbb_.push_slot_always::<&DeviceId>(TrackerId::VT_DEVICE_ID, device_id);
  }
  #[inline]
  pub fn add_tracker_num(&mut self, tracker_num: u8) {
    self.fbb_.push_slot::<u8>(TrackerId::VT_TRACKER_NUM, tracker_num, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrackerIdBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TrackerIdBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TrackerId<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TrackerId<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TrackerId");
      ds.field("device_id", &self.device_id());
      ds.field("tracker_num", &self.tracker_num());
      ds.finish()
  }
}

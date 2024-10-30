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
pub enum SkeletonPartOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SkeletonPart<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SkeletonPart<'a> {
  type Inner = SkeletonPart<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SkeletonPart<'a> {
  pub const VT_BONE: flatbuffers::VOffsetT = 4;
  pub const VT_VALUE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SkeletonPart { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SkeletonPartArgs
  ) -> flatbuffers::WIPOffset<SkeletonPart<'bldr>> {
    let mut builder = SkeletonPartBuilder::new(_fbb);
    builder.add_value(args.value);
    builder.add_bone(args.bone);
    builder.finish()
  }


  #[inline]
  pub fn bone(&self) -> SkeletonBone {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<SkeletonBone>(SkeletonPart::VT_BONE, Some(SkeletonBone::NONE)).unwrap()}
  }
  #[inline]
  pub fn value(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(SkeletonPart::VT_VALUE, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for SkeletonPart<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<SkeletonBone>("bone", Self::VT_BONE, false)?
     .visit_field::<f32>("value", Self::VT_VALUE, false)?
     .finish();
    Ok(())
  }
}
pub struct SkeletonPartArgs {
    pub bone: SkeletonBone,
    pub value: f32,
}
impl<'a> Default for SkeletonPartArgs {
  #[inline]
  fn default() -> Self {
    SkeletonPartArgs {
      bone: SkeletonBone::NONE,
      value: 0.0,
    }
  }
}

pub struct SkeletonPartBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SkeletonPartBuilder<'a, 'b> {
  #[inline]
  pub fn add_bone(&mut self, bone: SkeletonBone) {
    self.fbb_.push_slot::<SkeletonBone>(SkeletonPart::VT_BONE, bone, SkeletonBone::NONE);
  }
  #[inline]
  pub fn add_value(&mut self, value: f32) {
    self.fbb_.push_slot::<f32>(SkeletonPart::VT_VALUE, value, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SkeletonPartBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SkeletonPartBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SkeletonPart<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SkeletonPart<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SkeletonPart");
      ds.field("bone", &self.bone());
      ds.field("value", &self.value());
      ds.finish()
  }
}

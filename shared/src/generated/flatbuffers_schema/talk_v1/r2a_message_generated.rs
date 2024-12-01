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
pub enum R2aMessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct R2aMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for R2aMessage<'a> {
  type Inner = R2aMessage<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> R2aMessage<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_DATA: flatbuffers::VOffsetT = 6;
  pub const VT_SESSION_ID: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    R2aMessage { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args R2aMessageArgs<'args>
  ) -> flatbuffers::WIPOffset<R2aMessage<'bldr>> {
    let mut builder = R2aMessageBuilder::new(_fbb);
    builder.add_session_id(args.session_id);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.add_type_(args.type_);
    builder.finish()
  }


  #[inline]
  pub fn type_(&self) -> R2aMessageType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<R2aMessageType>(R2aMessage::VT_TYPE_, Some(R2aMessageType::FromFrontend)).unwrap()}
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(R2aMessage::VT_DATA, None)}
  }
  #[inline]
  pub fn session_id(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(R2aMessage::VT_SESSION_ID, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for R2aMessage<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<R2aMessageType>("type_", Self::VT_TYPE_, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .visit_field::<u64>("session_id", Self::VT_SESSION_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct R2aMessageArgs<'a> {
    pub type_: R2aMessageType,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub session_id: u64,
}
impl<'a> Default for R2aMessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    R2aMessageArgs {
      type_: R2aMessageType::FromFrontend,
      data: None,
      session_id: 0,
    }
  }
}

pub struct R2aMessageBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> R2aMessageBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_type_(&mut self, type_: R2aMessageType) {
    self.fbb_.push_slot::<R2aMessageType>(R2aMessage::VT_TYPE_, type_, R2aMessageType::FromFrontend);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(R2aMessage::VT_DATA, data);
  }
  #[inline]
  pub fn add_session_id(&mut self, session_id: u64) {
    self.fbb_.push_slot::<u64>(R2aMessage::VT_SESSION_ID, session_id, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> R2aMessageBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    R2aMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<R2aMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for R2aMessage<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("R2aMessage");
      ds.field("type_", &self.type_());
      ds.field("data", &self.data());
      ds.field("session_id", &self.session_id());
      ds.finish()
  }
}
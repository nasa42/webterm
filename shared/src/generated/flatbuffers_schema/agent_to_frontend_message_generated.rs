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
pub enum AgentToFrontendMessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AgentToFrontendMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AgentToFrontendMessage<'a> {
  type Inner = AgentToFrontendMessage<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AgentToFrontendMessage<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_DATA: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AgentToFrontendMessage { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args AgentToFrontendMessageArgs<'args>
  ) -> flatbuffers::WIPOffset<AgentToFrontendMessage<'bldr>> {
    let mut builder = AgentToFrontendMessageBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.add_type_(args.type_);
    builder.finish()
  }


  #[inline]
  pub fn type_(&self) -> AgentToFrontendMessageType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<AgentToFrontendMessageType>(AgentToFrontendMessage::VT_TYPE_, Some(AgentToFrontendMessageType::Data)).unwrap()}
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AgentToFrontendMessage::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for AgentToFrontendMessage<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<AgentToFrontendMessageType>("type_", Self::VT_TYPE_, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct AgentToFrontendMessageArgs<'a> {
    pub type_: AgentToFrontendMessageType,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for AgentToFrontendMessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    AgentToFrontendMessageArgs {
      type_: AgentToFrontendMessageType::Data,
      data: None,
    }
  }
}

pub struct AgentToFrontendMessageBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> AgentToFrontendMessageBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_type_(&mut self, type_: AgentToFrontendMessageType) {
    self.fbb_.push_slot::<AgentToFrontendMessageType>(AgentToFrontendMessage::VT_TYPE_, type_, AgentToFrontendMessageType::Data);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AgentToFrontendMessage::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> AgentToFrontendMessageBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    AgentToFrontendMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AgentToFrontendMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AgentToFrontendMessage<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AgentToFrontendMessage");
      ds.field("type_", &self.type_());
      ds.field("data", &self.data());
      ds.finish()
  }
}
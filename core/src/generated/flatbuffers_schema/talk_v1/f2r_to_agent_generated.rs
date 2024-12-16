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
pub enum F2rToAgentOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct F2rToAgent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for F2rToAgent<'a> {
  type Inner = F2rToAgent<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> F2rToAgent<'a> {
  pub const VT_PAYLOAD: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    F2rToAgent { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args F2rToAgentArgs<'args>
  ) -> flatbuffers::WIPOffset<F2rToAgent<'bldr>> {
    let mut builder = F2rToAgentBuilder::new(_fbb);
    if let Some(x) = args.payload { builder.add_payload(x); }
    builder.finish()
  }


  #[inline]
  pub fn payload(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(F2rToAgent::VT_PAYLOAD, None)}
  }
}

impl flatbuffers::Verifiable for F2rToAgent<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("payload", Self::VT_PAYLOAD, false)?
     .finish();
    Ok(())
  }
}
pub struct F2rToAgentArgs<'a> {
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for F2rToAgentArgs<'a> {
  #[inline]
  fn default() -> Self {
    F2rToAgentArgs {
      payload: None,
    }
  }
}

pub struct F2rToAgentBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> F2rToAgentBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(F2rToAgent::VT_PAYLOAD, payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> F2rToAgentBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    F2rToAgentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<F2rToAgent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for F2rToAgent<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("F2rToAgent");
      ds.field("payload", &self.payload());
      ds.finish()
  }
}
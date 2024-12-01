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
pub enum F2rHandshakeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct F2rHandshake<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for F2rHandshake<'a> {
  type Inner = F2rHandshake<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> F2rHandshake<'a> {
  pub const VT_FRONTEND_VERSION: flatbuffers::VOffsetT = 4;
  pub const VT_SERVER_ID: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    F2rHandshake { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args F2rHandshakeArgs<'args>
  ) -> flatbuffers::WIPOffset<F2rHandshake<'bldr>> {
    let mut builder = F2rHandshakeBuilder::new(_fbb);
    if let Some(x) = args.server_id { builder.add_server_id(x); }
    if let Some(x) = args.frontend_version { builder.add_frontend_version(x); }
    builder.finish()
  }


  #[inline]
  pub fn frontend_version(&self) -> Option<&'a Version> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Version>(F2rHandshake::VT_FRONTEND_VERSION, None)}
  }
  #[inline]
  pub fn server_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(F2rHandshake::VT_SERVER_ID, None)}
  }
}

impl flatbuffers::Verifiable for F2rHandshake<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Version>("frontend_version", Self::VT_FRONTEND_VERSION, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("server_id", Self::VT_SERVER_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct F2rHandshakeArgs<'a> {
    pub frontend_version: Option<&'a Version>,
    pub server_id: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for F2rHandshakeArgs<'a> {
  #[inline]
  fn default() -> Self {
    F2rHandshakeArgs {
      frontend_version: None,
      server_id: None,
    }
  }
}

pub struct F2rHandshakeBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> F2rHandshakeBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_frontend_version(&mut self, frontend_version: &Version) {
    self.fbb_.push_slot_always::<&Version>(F2rHandshake::VT_FRONTEND_VERSION, frontend_version);
  }
  #[inline]
  pub fn add_server_id(&mut self, server_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(F2rHandshake::VT_SERVER_ID, server_id);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> F2rHandshakeBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    F2rHandshakeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<F2rHandshake<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for F2rHandshake<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("F2rHandshake");
      ds.field("frontend_version", &self.frontend_version());
      ds.field("server_id", &self.server_id());
      ds.finish()
  }
}
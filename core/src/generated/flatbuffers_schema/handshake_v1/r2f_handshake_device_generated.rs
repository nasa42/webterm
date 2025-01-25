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
pub enum R2fHandshakeDeviceOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct R2fHandshakeDevice<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for R2fHandshakeDevice<'a> {
  type Inner = R2fHandshakeDevice<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> R2fHandshakeDevice<'a> {
  pub const VT_SUBNAME: flatbuffers::VOffsetT = 4;
  pub const VT_LAST_ONLINE_TIMESTAMP: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    R2fHandshakeDevice { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args R2fHandshakeDeviceArgs<'args>
  ) -> flatbuffers::WIPOffset<R2fHandshakeDevice<'bldr>> {
    let mut builder = R2fHandshakeDeviceBuilder::new(_fbb);
    builder.add_last_online_timestamp(args.last_online_timestamp);
    if let Some(x) = args.subname { builder.add_subname(x); }
    builder.finish()
  }


  #[inline]
  pub fn subname(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(R2fHandshakeDevice::VT_SUBNAME, None)}
  }
  #[inline]
  pub fn last_online_timestamp(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(R2fHandshakeDevice::VT_LAST_ONLINE_TIMESTAMP, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for R2fHandshakeDevice<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("subname", Self::VT_SUBNAME, false)?
     .visit_field::<u64>("last_online_timestamp", Self::VT_LAST_ONLINE_TIMESTAMP, false)?
     .finish();
    Ok(())
  }
}
pub struct R2fHandshakeDeviceArgs<'a> {
    pub subname: Option<flatbuffers::WIPOffset<&'a str>>,
    pub last_online_timestamp: u64,
}
impl<'a> Default for R2fHandshakeDeviceArgs<'a> {
  #[inline]
  fn default() -> Self {
    R2fHandshakeDeviceArgs {
      subname: None,
      last_online_timestamp: 0,
    }
  }
}

pub struct R2fHandshakeDeviceBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> R2fHandshakeDeviceBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_subname(&mut self, subname: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(R2fHandshakeDevice::VT_SUBNAME, subname);
  }
  #[inline]
  pub fn add_last_online_timestamp(&mut self, last_online_timestamp: u64) {
    self.fbb_.push_slot::<u64>(R2fHandshakeDevice::VT_LAST_ONLINE_TIMESTAMP, last_online_timestamp, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> R2fHandshakeDeviceBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    R2fHandshakeDeviceBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<R2fHandshakeDevice<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for R2fHandshakeDevice<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("R2fHandshakeDevice");
      ds.field("subname", &self.subname());
      ds.field("last_online_timestamp", &self.last_online_timestamp());
      ds.finish()
  }
}

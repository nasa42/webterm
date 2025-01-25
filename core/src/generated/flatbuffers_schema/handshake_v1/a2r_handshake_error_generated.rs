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
pub enum A2rHandshakeErrorOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct A2rHandshakeError<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for A2rHandshakeError<'a> {
  type Inner = A2rHandshakeError<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> A2rHandshakeError<'a> {
  pub const VT_ERROR_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_ERROR_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    A2rHandshakeError { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args A2rHandshakeErrorArgs<'args>
  ) -> flatbuffers::WIPOffset<A2rHandshakeError<'bldr>> {
    let mut builder = A2rHandshakeErrorBuilder::new(_fbb);
    if let Some(x) = args.error_message { builder.add_error_message(x); }
    builder.add_error_type(args.error_type);
    builder.finish()
  }


  #[inline]
  pub fn error_type(&self) -> A2rHandshakeErrorType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<A2rHandshakeErrorType>(A2rHandshakeError::VT_ERROR_TYPE, Some(A2rHandshakeErrorType::ErrorUnspecified)).unwrap()}
  }
  #[inline]
  pub fn error_message(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(A2rHandshakeError::VT_ERROR_MESSAGE, None)}
  }
}

impl flatbuffers::Verifiable for A2rHandshakeError<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<A2rHandshakeErrorType>("error_type", Self::VT_ERROR_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("error_message", Self::VT_ERROR_MESSAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct A2rHandshakeErrorArgs<'a> {
    pub error_type: A2rHandshakeErrorType,
    pub error_message: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for A2rHandshakeErrorArgs<'a> {
  #[inline]
  fn default() -> Self {
    A2rHandshakeErrorArgs {
      error_type: A2rHandshakeErrorType::ErrorUnspecified,
      error_message: None,
    }
  }
}

pub struct A2rHandshakeErrorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> A2rHandshakeErrorBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_error_type(&mut self, error_type: A2rHandshakeErrorType) {
    self.fbb_.push_slot::<A2rHandshakeErrorType>(A2rHandshakeError::VT_ERROR_TYPE, error_type, A2rHandshakeErrorType::ErrorUnspecified);
  }
  #[inline]
  pub fn add_error_message(&mut self, error_message: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(A2rHandshakeError::VT_ERROR_MESSAGE, error_message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> A2rHandshakeErrorBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    A2rHandshakeErrorBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<A2rHandshakeError<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for A2rHandshakeError<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("A2rHandshakeError");
      ds.field("error_type", &self.error_type());
      ds.field("error_message", &self.error_message());
      ds.finish()
  }
}

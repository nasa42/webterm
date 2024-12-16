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
pub enum R2aErrorOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct R2aError<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for R2aError<'a> {
  type Inner = R2aError<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> R2aError<'a> {
  pub const VT_ERROR_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_ERROR_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    R2aError { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args R2aErrorArgs<'args>
  ) -> flatbuffers::WIPOffset<R2aError<'bldr>> {
    let mut builder = R2aErrorBuilder::new(_fbb);
    if let Some(x) = args.error_message { builder.add_error_message(x); }
    builder.add_error_type(args.error_type);
    builder.finish()
  }


  #[inline]
  pub fn error_type(&self) -> R2aErrorType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<R2aErrorType>(R2aError::VT_ERROR_TYPE, Some(R2aErrorType::ErrorUnspecified)).unwrap()}
  }
  #[inline]
  pub fn error_message(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(R2aError::VT_ERROR_MESSAGE, None)}
  }
}

impl flatbuffers::Verifiable for R2aError<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<R2aErrorType>("error_type", Self::VT_ERROR_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("error_message", Self::VT_ERROR_MESSAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct R2aErrorArgs<'a> {
    pub error_type: R2aErrorType,
    pub error_message: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for R2aErrorArgs<'a> {
  #[inline]
  fn default() -> Self {
    R2aErrorArgs {
      error_type: R2aErrorType::ErrorUnspecified,
      error_message: None,
    }
  }
}

pub struct R2aErrorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> R2aErrorBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_error_type(&mut self, error_type: R2aErrorType) {
    self.fbb_.push_slot::<R2aErrorType>(R2aError::VT_ERROR_TYPE, error_type, R2aErrorType::ErrorUnspecified);
  }
  #[inline]
  pub fn add_error_message(&mut self, error_message: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(R2aError::VT_ERROR_MESSAGE, error_message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> R2aErrorBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    R2aErrorBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<R2aError<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for R2aError<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("R2aError");
      ds.field("error_type", &self.error_type());
      ds.field("error_message", &self.error_message());
      ds.finish()
  }
}
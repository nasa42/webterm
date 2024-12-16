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
pub enum F2aRootOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct F2aRoot<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for F2aRoot<'a> {
  type Inner = F2aRoot<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> F2aRoot<'a> {
  pub const VT_FORMAT: flatbuffers::VOffsetT = 4;
  pub const VT_IV: flatbuffers::VOffsetT = 6;
  pub const VT_PLAIN_MESSAGE_TYPE: flatbuffers::VOffsetT = 8;
  pub const VT_PLAIN_MESSAGE: flatbuffers::VOffsetT = 10;
  pub const VT_ENCRYPTED_PAYLOAD: flatbuffers::VOffsetT = 12;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    F2aRoot { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args F2aRootArgs<'args>
  ) -> flatbuffers::WIPOffset<F2aRoot<'bldr>> {
    let mut builder = F2aRootBuilder::new(_fbb);
    if let Some(x) = args.encrypted_payload { builder.add_encrypted_payload(x); }
    if let Some(x) = args.plain_message { builder.add_plain_message(x); }
    if let Some(x) = args.iv { builder.add_iv(x); }
    builder.add_plain_message_type(args.plain_message_type);
    builder.add_format(args.format);
    builder.finish()
  }


  #[inline]
  pub fn format(&self) -> F2aMessageFormat {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<F2aMessageFormat>(F2aRoot::VT_FORMAT, Some(F2aMessageFormat::Plain)).unwrap()}
  }
  #[inline]
  pub fn iv(&self) -> Option<&'a Bits96> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Bits96>(F2aRoot::VT_IV, None)}
  }
  #[inline]
  pub fn plain_message_type(&self) -> F2aPlainMessage {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<F2aPlainMessage>(F2aRoot::VT_PLAIN_MESSAGE_TYPE, Some(F2aPlainMessage::NONE)).unwrap()}
  }
  #[inline]
  pub fn plain_message(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(F2aRoot::VT_PLAIN_MESSAGE, None)}
  }
  #[inline]
  pub fn encrypted_payload(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(F2aRoot::VT_ENCRYPTED_PAYLOAD, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn plain_message_as_auth_request_preamble(&self) -> Option<F2aPlainAuthRequestPreamble<'a>> {
    if self.plain_message_type() == F2aPlainMessage::AuthRequestPreamble {
      self.plain_message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { F2aPlainAuthRequestPreamble::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn plain_message_as_auth_present_verification(&self) -> Option<F2aPlainAuthPresentVerification<'a>> {
    if self.plain_message_type() == F2aPlainMessage::AuthPresentVerification {
      self.plain_message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { F2aPlainAuthPresentVerification::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for F2aRoot<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<F2aMessageFormat>("format", Self::VT_FORMAT, false)?
     .visit_field::<Bits96>("iv", Self::VT_IV, false)?
     .visit_union::<F2aPlainMessage, _>("plain_message_type", Self::VT_PLAIN_MESSAGE_TYPE, "plain_message", Self::VT_PLAIN_MESSAGE, false, |key, v, pos| {
        match key {
          F2aPlainMessage::AuthRequestPreamble => v.verify_union_variant::<flatbuffers::ForwardsUOffset<F2aPlainAuthRequestPreamble>>("F2aPlainMessage::AuthRequestPreamble", pos),
          F2aPlainMessage::AuthPresentVerification => v.verify_union_variant::<flatbuffers::ForwardsUOffset<F2aPlainAuthPresentVerification>>("F2aPlainMessage::AuthPresentVerification", pos),
          _ => Ok(()),
        }
     })?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("encrypted_payload", Self::VT_ENCRYPTED_PAYLOAD, false)?
     .finish();
    Ok(())
  }
}
pub struct F2aRootArgs<'a> {
    pub format: F2aMessageFormat,
    pub iv: Option<&'a Bits96>,
    pub plain_message_type: F2aPlainMessage,
    pub plain_message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub encrypted_payload: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for F2aRootArgs<'a> {
  #[inline]
  fn default() -> Self {
    F2aRootArgs {
      format: F2aMessageFormat::Plain,
      iv: None,
      plain_message_type: F2aPlainMessage::NONE,
      plain_message: None,
      encrypted_payload: None,
    }
  }
}

pub struct F2aRootBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> F2aRootBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_format(&mut self, format: F2aMessageFormat) {
    self.fbb_.push_slot::<F2aMessageFormat>(F2aRoot::VT_FORMAT, format, F2aMessageFormat::Plain);
  }
  #[inline]
  pub fn add_iv(&mut self, iv: &Bits96) {
    self.fbb_.push_slot_always::<&Bits96>(F2aRoot::VT_IV, iv);
  }
  #[inline]
  pub fn add_plain_message_type(&mut self, plain_message_type: F2aPlainMessage) {
    self.fbb_.push_slot::<F2aPlainMessage>(F2aRoot::VT_PLAIN_MESSAGE_TYPE, plain_message_type, F2aPlainMessage::NONE);
  }
  #[inline]
  pub fn add_plain_message(&mut self, plain_message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(F2aRoot::VT_PLAIN_MESSAGE, plain_message);
  }
  #[inline]
  pub fn add_encrypted_payload(&mut self, encrypted_payload: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(F2aRoot::VT_ENCRYPTED_PAYLOAD, encrypted_payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> F2aRootBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    F2aRootBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<F2aRoot<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for F2aRoot<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("F2aRoot");
      ds.field("format", &self.format());
      ds.field("iv", &self.iv());
      ds.field("plain_message_type", &self.plain_message_type());
      match self.plain_message_type() {
        F2aPlainMessage::AuthRequestPreamble => {
          if let Some(x) = self.plain_message_as_auth_request_preamble() {
            ds.field("plain_message", &x)
          } else {
            ds.field("plain_message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        F2aPlainMessage::AuthPresentVerification => {
          if let Some(x) = self.plain_message_as_auth_present_verification() {
            ds.field("plain_message", &x)
          } else {
            ds.field("plain_message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("plain_message", &x)
        },
      };
      ds.field("encrypted_payload", &self.encrypted_payload());
      ds.finish()
  }
}
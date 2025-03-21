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
pub enum F2aEncryptedRootOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct F2aEncryptedRoot<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for F2aEncryptedRoot<'a> {
  type Inner = F2aEncryptedRoot<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> F2aEncryptedRoot<'a> {
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    F2aEncryptedRoot { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args F2aEncryptedRootArgs
  ) -> flatbuffers::WIPOffset<F2aEncryptedRoot<'bldr>> {
    let mut builder = F2aEncryptedRootBuilder::new(_fbb);
    if let Some(x) = args.message { builder.add_message(x); }
    builder.add_message_type(args.message_type);
    builder.finish()
  }


  #[inline]
  pub fn message_type(&self) -> F2aMessage {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<F2aMessage>(F2aEncryptedRoot::VT_MESSAGE_TYPE, Some(F2aMessage::NONE)).unwrap()}
  }
  #[inline]
  pub fn message(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(F2aEncryptedRoot::VT_MESSAGE, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_error(&self) -> Option<F2aError<'a>> {
    if self.message_type() == F2aMessage::Error {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { F2aError::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_activity_input(&self) -> Option<F2aActivityInput<'a>> {
    if self.message_type() == F2aMessage::ActivityInput {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { F2aActivityInput::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_activity_create_terminal(&self) -> Option<EmptyTable<'a>> {
    if self.message_type() == F2aMessage::ActivityCreateTerminal {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { EmptyTable::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_terminal_resize(&self) -> Option<EmptyTable<'a>> {
    if self.message_type() == F2aMessage::TerminalResize {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { EmptyTable::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for F2aEncryptedRoot<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<F2aMessage, _>("message_type", Self::VT_MESSAGE_TYPE, "message", Self::VT_MESSAGE, false, |key, v, pos| {
        match key {
          F2aMessage::Error => v.verify_union_variant::<flatbuffers::ForwardsUOffset<F2aError>>("F2aMessage::Error", pos),
          F2aMessage::ActivityInput => v.verify_union_variant::<flatbuffers::ForwardsUOffset<F2aActivityInput>>("F2aMessage::ActivityInput", pos),
          F2aMessage::ActivityCreateTerminal => v.verify_union_variant::<flatbuffers::ForwardsUOffset<EmptyTable>>("F2aMessage::ActivityCreateTerminal", pos),
          F2aMessage::TerminalResize => v.verify_union_variant::<flatbuffers::ForwardsUOffset<EmptyTable>>("F2aMessage::TerminalResize", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct F2aEncryptedRootArgs {
    pub message_type: F2aMessage,
    pub message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for F2aEncryptedRootArgs {
  #[inline]
  fn default() -> Self {
    F2aEncryptedRootArgs {
      message_type: F2aMessage::NONE,
      message: None,
    }
  }
}

pub struct F2aEncryptedRootBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> F2aEncryptedRootBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: F2aMessage) {
    self.fbb_.push_slot::<F2aMessage>(F2aEncryptedRoot::VT_MESSAGE_TYPE, message_type, F2aMessage::NONE);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(F2aEncryptedRoot::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> F2aEncryptedRootBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    F2aEncryptedRootBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<F2aEncryptedRoot<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for F2aEncryptedRoot<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("F2aEncryptedRoot");
      ds.field("message_type", &self.message_type());
      match self.message_type() {
        F2aMessage::Error => {
          if let Some(x) = self.message_as_error() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        F2aMessage::ActivityInput => {
          if let Some(x) = self.message_as_activity_input() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        F2aMessage::ActivityCreateTerminal => {
          if let Some(x) = self.message_as_activity_create_terminal() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        F2aMessage::TerminalResize => {
          if let Some(x) = self.message_as_terminal_resize() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("message", &x)
        },
      };
      ds.finish()
  }
}

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
pub enum R2fRootOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct R2fRoot<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for R2fRoot<'a> {
  type Inner = R2fRoot<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> R2fRoot<'a> {
  pub const VT_ROOT_PAYLOAD_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_ROOT_PAYLOAD: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    R2fRoot { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args R2fRootArgs
  ) -> flatbuffers::WIPOffset<R2fRoot<'bldr>> {
    let mut builder = R2fRootBuilder::new(_fbb);
    if let Some(x) = args.root_payload { builder.add_root_payload(x); }
    builder.add_root_payload_type(args.root_payload_type);
    builder.finish()
  }


  #[inline]
  pub fn root_payload_type(&self) -> R2fRootPayload {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<R2fRootPayload>(R2fRoot::VT_ROOT_PAYLOAD_TYPE, Some(R2fRootPayload::NONE)).unwrap()}
  }
  #[inline]
  pub fn root_payload(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(R2fRoot::VT_ROOT_PAYLOAD, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn root_payload_as_error(&self) -> Option<R2fError<'a>> {
    if self.root_payload_type() == R2fRootPayload::Error {
      self.root_payload().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { R2fError::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn root_payload_as_from_agent(&self) -> Option<R2fFromAgent<'a>> {
    if self.root_payload_type() == R2fRootPayload::FromAgent {
      self.root_payload().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { R2fFromAgent::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn root_payload_as_relay_shutting_down(&self) -> Option<EmptyTable<'a>> {
    if self.root_payload_type() == R2fRootPayload::RelayShuttingDown {
      self.root_payload().map(|t| {
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

impl flatbuffers::Verifiable for R2fRoot<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<R2fRootPayload, _>("root_payload_type", Self::VT_ROOT_PAYLOAD_TYPE, "root_payload", Self::VT_ROOT_PAYLOAD, false, |key, v, pos| {
        match key {
          R2fRootPayload::Error => v.verify_union_variant::<flatbuffers::ForwardsUOffset<R2fError>>("R2fRootPayload::Error", pos),
          R2fRootPayload::FromAgent => v.verify_union_variant::<flatbuffers::ForwardsUOffset<R2fFromAgent>>("R2fRootPayload::FromAgent", pos),
          R2fRootPayload::RelayShuttingDown => v.verify_union_variant::<flatbuffers::ForwardsUOffset<EmptyTable>>("R2fRootPayload::RelayShuttingDown", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct R2fRootArgs {
    pub root_payload_type: R2fRootPayload,
    pub root_payload: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for R2fRootArgs {
  #[inline]
  fn default() -> Self {
    R2fRootArgs {
      root_payload_type: R2fRootPayload::NONE,
      root_payload: None,
    }
  }
}

pub struct R2fRootBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> R2fRootBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_root_payload_type(&mut self, root_payload_type: R2fRootPayload) {
    self.fbb_.push_slot::<R2fRootPayload>(R2fRoot::VT_ROOT_PAYLOAD_TYPE, root_payload_type, R2fRootPayload::NONE);
  }
  #[inline]
  pub fn add_root_payload(&mut self, root_payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(R2fRoot::VT_ROOT_PAYLOAD, root_payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> R2fRootBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    R2fRootBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<R2fRoot<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for R2fRoot<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("R2fRoot");
      ds.field("root_payload_type", &self.root_payload_type());
      match self.root_payload_type() {
        R2fRootPayload::Error => {
          if let Some(x) = self.root_payload_as_error() {
            ds.field("root_payload", &x)
          } else {
            ds.field("root_payload", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        R2fRootPayload::FromAgent => {
          if let Some(x) = self.root_payload_as_from_agent() {
            ds.field("root_payload", &x)
          } else {
            ds.field("root_payload", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        R2fRootPayload::RelayShuttingDown => {
          if let Some(x) = self.root_payload_as_relay_shutting_down() {
            ds.field("root_payload", &x)
          } else {
            ds.field("root_payload", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("root_payload", &x)
        },
      };
      ds.finish()
  }
}

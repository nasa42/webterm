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
pub enum TerminalResizeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TerminalResize<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TerminalResize<'a> {
  type Inner = TerminalResize<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TerminalResize<'a> {
  pub const VT_COLS: flatbuffers::VOffsetT = 4;
  pub const VT_ROWS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TerminalResize { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args TerminalResizeArgs
  ) -> flatbuffers::WIPOffset<TerminalResize<'bldr>> {
    let mut builder = TerminalResizeBuilder::new(_fbb);
    builder.add_rows(args.rows);
    builder.add_cols(args.cols);
    builder.finish()
  }


  #[inline]
  pub fn cols(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(TerminalResize::VT_COLS, Some(0)).unwrap()}
  }
  #[inline]
  pub fn rows(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(TerminalResize::VT_ROWS, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TerminalResize<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u16>("cols", Self::VT_COLS, false)?
     .visit_field::<u16>("rows", Self::VT_ROWS, false)?
     .finish();
    Ok(())
  }
}
pub struct TerminalResizeArgs {
    pub cols: u16,
    pub rows: u16,
}
impl<'a> Default for TerminalResizeArgs {
  #[inline]
  fn default() -> Self {
    TerminalResizeArgs {
      cols: 0,
      rows: 0,
    }
  }
}

pub struct TerminalResizeBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> TerminalResizeBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_cols(&mut self, cols: u16) {
    self.fbb_.push_slot::<u16>(TerminalResize::VT_COLS, cols, 0);
  }
  #[inline]
  pub fn add_rows(&mut self, rows: u16) {
    self.fbb_.push_slot::<u16>(TerminalResize::VT_ROWS, rows, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TerminalResizeBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    TerminalResizeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TerminalResize<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TerminalResize<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TerminalResize");
      ds.field("cols", &self.cols());
      ds.field("rows", &self.rows());
      ds.finish()
  }
}
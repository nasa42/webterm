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
// struct Version, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Version(pub [u8; 3]);
impl Default for Version { 
  fn default() -> Self { 
    Self([0; 3])
  }
}
impl core::fmt::Debug for Version {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Version")
      .field("major", &self.major())
      .field("minor", &self.minor())
      .field("patch", &self.patch())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Version {}
impl<'a> flatbuffers::Follow<'a> for Version {
  type Inner = &'a Version;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Version>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Version {
  type Inner = &'a Version;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Version>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Version {
    type Output = Version;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Version as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(1)
    }
}

impl<'a> flatbuffers::Verifiable for Version {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Version {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    major: u8,
    minor: u8,
    patch: u8,
  ) -> Self {
    let mut s = Self([0; 3]);
    s.set_major(major);
    s.set_minor(minor);
    s.set_patch(patch);
    s
  }

  pub fn major(&self) -> u8 {
    let mut mem = core::mem::MaybeUninit::<<u8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_major(&mut self, x: u8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn minor(&self) -> u8 {
    let mut mem = core::mem::MaybeUninit::<<u8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[1..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_minor(&mut self, x: u8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[1..].as_mut_ptr(),
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn patch(&self) -> u8 {
    let mut mem = core::mem::MaybeUninit::<<u8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[2..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_patch(&mut self, x: u8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[2..].as_mut_ptr(),
        core::mem::size_of::<<u8 as EndianScalar>::Scalar>(),
      );
    }
  }

}


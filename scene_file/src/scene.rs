// automatically generated by the FlatBuffers compiler, do not modify



use crate::mesh_primitive::*;
use crate::bvh_accel::*;
use crate::common::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum SceneOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Scene<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Scene<'a> {
    type Inner = Scene<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Scene<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Scene { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SceneArgs<'args>) -> flatbuffers::WIPOffset<Scene<'bldr>> {
      let mut builder = SceneBuilder::new(_fbb);
      if let Some(x) = args.root { builder.add_root(x); }
      builder.finish()
    }

    pub const VT_ROOT: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn root(&self) -> Option<BVHNode<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<BVHNode>>(Scene::VT_ROOT, None)
  }
}

impl flatbuffers::Verifiable for Scene<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<BVHNode>>(&"root", Self::VT_ROOT, false)?
     .finish();
    Ok(())
  }
}
pub struct SceneArgs<'a> {
    pub root: Option<flatbuffers::WIPOffset<BVHNode<'a>>>,
}
impl<'a> Default for SceneArgs<'a> {
    #[inline]
    fn default() -> Self {
        SceneArgs {
            root: None,
        }
    }
}
pub struct SceneBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SceneBuilder<'a, 'b> {
  #[inline]
  pub fn add_root(&mut self, root: flatbuffers::WIPOffset<BVHNode<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<BVHNode>>(Scene::VT_ROOT, root);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SceneBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SceneBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Scene<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Scene<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Scene");
      ds.field("root", &self.root());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_scene<'a>(buf: &'a [u8]) -> Scene<'a> {
  unsafe { flatbuffers::root_unchecked::<Scene<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_scene<'a>(buf: &'a [u8]) -> Scene<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Scene<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Scene`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scene_unchecked`.
pub fn root_as_scene(buf: &[u8]) -> Result<Scene, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Scene>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Scene` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_scene_unchecked`.
pub fn size_prefixed_root_as_scene(buf: &[u8]) -> Result<Scene, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Scene>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Scene` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scene_unchecked`.
pub fn root_as_scene_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Scene<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Scene<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Scene` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_scene_unchecked`.
pub fn size_prefixed_root_as_scene_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Scene<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Scene<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Scene and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Scene`.
pub unsafe fn root_as_scene_unchecked(buf: &[u8]) -> Scene {
  flatbuffers::root_unchecked::<Scene>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Scene and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Scene`.
pub unsafe fn size_prefixed_root_as_scene_unchecked(buf: &[u8]) -> Scene {
  flatbuffers::size_prefixed_root_unchecked::<Scene>(buf)
}
#[inline]
pub fn finish_scene_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Scene<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_scene_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Scene<'a>>) {
  fbb.finish_size_prefixed(root, None);
}

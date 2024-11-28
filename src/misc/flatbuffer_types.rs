// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod recording {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

  #[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
  pub const ENUM_MIN_CONTENT: u8 = 0;
  #[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
  pub const ENUM_MAX_CONTENT: u8 = 2;
  #[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
  #[allow(non_camel_case_types)]
  pub const ENUM_VALUES_CONTENT: [Content; 3] = [
    Content::NONE,
    Content::RecordingPayload,
    Content::CommandPayload,
  ];

  #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
  #[repr(transparent)]
  pub struct Content(pub u8);
  #[allow(non_upper_case_globals)]
  impl Content {
    pub const NONE: Self = Self(0);
    pub const RecordingPayload: Self = Self(1);
    pub const CommandPayload: Self = Self(2);

    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_MAX: u8 = 2;
    pub const ENUM_VALUES: &'static [Self] = &[
      Self::NONE,
      Self::RecordingPayload,
      Self::CommandPayload,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
      match self {
        Self::NONE => Some("NONE"),
        Self::RecordingPayload => Some("RecordingPayload"),
        Self::CommandPayload => Some("CommandPayload"),
        _ => None,
      }
    }
  }
  impl core::fmt::Debug for Content {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
      if let Some(name) = self.variant_name() {
        f.write_str(name)
      } else {
        f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
      }
    }
  }
  impl<'a> flatbuffers::Follow<'a> for Content {
    type Inner = Self;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
      Self(b)
    }
  }

  impl flatbuffers::Push for Content {
    type Output = Content;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
      flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
  }

  impl flatbuffers::EndianScalar for Content {
    type Scalar = u8;
    #[inline]
    fn to_little_endian(self) -> u8 {
      self.0.to_le()
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(v: u8) -> Self {
      let b = u8::from_le(v);
      Self(b)
    }
  }

  impl<'a> flatbuffers::Verifiable for Content {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      u8::run_verifier(v, pos)
    }
  }

  impl flatbuffers::SimpleToVerifyInSlice for Content {}
  pub struct ContentUnionTableOffset {}

  pub enum MediaOffset {}
  #[derive(Copy, Clone, PartialEq)]

  pub struct Media<'a> {
    pub _tab: flatbuffers::Table<'a>,
  }

  impl<'a> flatbuffers::Follow<'a> for Media<'a> {
    type Inner = Media<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
  }

  impl<'a> Media<'a> {
    pub const VT_PORT: flatbuffers::VOffsetT = 4;
    pub const VT_CODEC: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
      Media { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
      _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
      args: &'args MediaArgs<'args>
    ) -> flatbuffers::WIPOffset<Media<'bldr>> {
      let mut builder = MediaBuilder::new(_fbb);
      if let Some(x) = args.codec { builder.add_codec(x); }
      builder.add_port(args.port);
      builder.finish()
    }


    #[inline]
    pub fn port(&self) -> i32 {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<i32>(Media::VT_PORT, Some(0)).unwrap()}
    }
    #[inline]
    pub fn codec(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Media::VT_CODEC, None).unwrap()}
    }
  }

  impl flatbuffers::Verifiable for Media<'_> {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      v.visit_table(pos)?
          .visit_field::<i32>("port", Self::VT_PORT, false)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("codec", Self::VT_CODEC, true)?
          .finish();
      Ok(())
    }
  }
  pub struct MediaArgs<'a> {
    pub port: i32,
    pub codec: Option<flatbuffers::WIPOffset<&'a str>>,
  }
  impl<'a> Default for MediaArgs<'a> {
    #[inline]
    fn default() -> Self {
      MediaArgs {
        port: 0,
        codec: None, // required field
      }
    }
  }

  pub struct MediaBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
  }
  impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MediaBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_port(&mut self, port: i32) {
      self.fbb_.push_slot::<i32>(Media::VT_PORT, port, 0);
    }
    #[inline]
    pub fn add_codec(&mut self, codec: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Media::VT_CODEC, codec);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MediaBuilder<'a, 'b, A> {
      let start = _fbb.start_table();
      MediaBuilder {
        fbb_: _fbb,
        start_: start,
      }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Media<'a>> {
      let o = self.fbb_.end_table(self.start_);
      self.fbb_.required(o, Media::VT_CODEC,"codec");
      flatbuffers::WIPOffset::new(o.value())
    }
  }

  impl core::fmt::Debug for Media<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      let mut ds = f.debug_struct("Media");
      ds.field("port", &self.port());
      ds.field("codec", &self.codec());
      ds.finish()
    }
  }
  pub enum RecordingPayloadOffset {}
  #[derive(Copy, Clone, PartialEq)]

  pub struct RecordingPayload<'a> {
    pub _tab: flatbuffers::Table<'a>,
  }

  impl<'a> flatbuffers::Follow<'a> for RecordingPayload<'a> {
    type Inner = RecordingPayload<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
  }

  impl<'a> RecordingPayload<'a> {
    pub const VT_CHANNEL_UUID: flatbuffers::VOffsetT = 4;
    pub const VT_ORIGIN: flatbuffers::VOffsetT = 6;
    pub const VT_VIDEO_MEDIAS: flatbuffers::VOffsetT = 8;
    pub const VT_AUDIO_MEDIAS: flatbuffers::VOffsetT = 10;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
      RecordingPayload { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
      _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
      args: &'args RecordingPayloadArgs<'args>
    ) -> flatbuffers::WIPOffset<RecordingPayload<'bldr>> {
      let mut builder = RecordingPayloadBuilder::new(_fbb);
      if let Some(x) = args.audio_medias { builder.add_audio_medias(x); }
      if let Some(x) = args.video_medias { builder.add_video_medias(x); }
      if let Some(x) = args.origin { builder.add_origin(x); }
      if let Some(x) = args.channel_uuid { builder.add_channel_uuid(x); }
      builder.finish()
    }


    #[inline]
    pub fn channel_uuid(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(RecordingPayload::VT_CHANNEL_UUID, None).unwrap()}
    }
    #[inline]
    pub fn origin(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(RecordingPayload::VT_ORIGIN, None).unwrap()}
    }
    #[inline]
    pub fn video_medias(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media<'a>>> {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media>>>>(RecordingPayload::VT_VIDEO_MEDIAS, None).unwrap()}
    }
    #[inline]
    pub fn audio_medias(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media<'a>>> {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media>>>>(RecordingPayload::VT_AUDIO_MEDIAS, None).unwrap()}
    }
  }

  impl flatbuffers::Verifiable for RecordingPayload<'_> {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      v.visit_table(pos)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("channel_uuid", Self::VT_CHANNEL_UUID, true)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("origin", Self::VT_ORIGIN, true)?
          .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Media>>>>("video_medias", Self::VT_VIDEO_MEDIAS, true)?
          .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Media>>>>("audio_medias", Self::VT_AUDIO_MEDIAS, true)?
          .finish();
      Ok(())
    }
  }
  pub struct RecordingPayloadArgs<'a> {
    pub channel_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub origin: Option<flatbuffers::WIPOffset<&'a str>>,
    pub video_medias: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media<'a>>>>>,
    pub audio_medias: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Media<'a>>>>>,
  }
  impl<'a> Default for RecordingPayloadArgs<'a> {
    #[inline]
    fn default() -> Self {
      RecordingPayloadArgs {
        channel_uuid: None, // required field
        origin: None, // required field
        video_medias: None, // required field
        audio_medias: None, // required field
      }
    }
  }

  pub struct RecordingPayloadBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
  }
  impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> RecordingPayloadBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_channel_uuid(&mut self, channel_uuid: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RecordingPayload::VT_CHANNEL_UUID, channel_uuid);
    }
    #[inline]
    pub fn add_origin(&mut self, origin: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RecordingPayload::VT_ORIGIN, origin);
    }
    #[inline]
    pub fn add_video_medias(&mut self, video_medias: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Media<'b >>>>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RecordingPayload::VT_VIDEO_MEDIAS, video_medias);
    }
    #[inline]
    pub fn add_audio_medias(&mut self, audio_medias: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Media<'b >>>>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RecordingPayload::VT_AUDIO_MEDIAS, audio_medias);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RecordingPayloadBuilder<'a, 'b, A> {
      let start = _fbb.start_table();
      RecordingPayloadBuilder {
        fbb_: _fbb,
        start_: start,
      }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RecordingPayload<'a>> {
      let o = self.fbb_.end_table(self.start_);
      self.fbb_.required(o, RecordingPayload::VT_CHANNEL_UUID,"channel_uuid");
      self.fbb_.required(o, RecordingPayload::VT_ORIGIN,"origin");
      self.fbb_.required(o, RecordingPayload::VT_VIDEO_MEDIAS,"video_medias");
      self.fbb_.required(o, RecordingPayload::VT_AUDIO_MEDIAS,"audio_medias");
      flatbuffers::WIPOffset::new(o.value())
    }
  }

  impl core::fmt::Debug for RecordingPayload<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      let mut ds = f.debug_struct("RecordingPayload");
      ds.field("channel_uuid", &self.channel_uuid());
      ds.field("origin", &self.origin());
      ds.field("video_medias", &self.video_medias());
      ds.field("audio_medias", &self.audio_medias());
      ds.finish()
    }
  }
  pub enum CommandPayloadOffset {}
  #[derive(Copy, Clone, PartialEq)]

  pub struct CommandPayload<'a> {
    pub _tab: flatbuffers::Table<'a>,
  }

  impl<'a> flatbuffers::Follow<'a> for CommandPayload<'a> {
    type Inner = CommandPayload<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
  }

  impl<'a> CommandPayload<'a> {
    pub const VT_CHANNEL_UUID: flatbuffers::VOffsetT = 4;
    pub const VT_NAME: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
      CommandPayload { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
      _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
      args: &'args CommandPayloadArgs<'args>
    ) -> flatbuffers::WIPOffset<CommandPayload<'bldr>> {
      let mut builder = CommandPayloadBuilder::new(_fbb);
      if let Some(x) = args.name { builder.add_name(x); }
      if let Some(x) = args.channel_uuid { builder.add_channel_uuid(x); }
      builder.finish()
    }


    #[inline]
    pub fn channel_uuid(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(CommandPayload::VT_CHANNEL_UUID, None).unwrap()}
    }
    #[inline]
    pub fn name(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(CommandPayload::VT_NAME, None).unwrap()}
    }
  }

  impl flatbuffers::Verifiable for CommandPayload<'_> {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      v.visit_table(pos)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("channel_uuid", Self::VT_CHANNEL_UUID, true)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, true)?
          .finish();
      Ok(())
    }
  }
  pub struct CommandPayloadArgs<'a> {
    pub channel_uuid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
  }
  impl<'a> Default for CommandPayloadArgs<'a> {
    #[inline]
    fn default() -> Self {
      CommandPayloadArgs {
        channel_uuid: None, // required field
        name: None, // required field
      }
    }
  }

  pub struct CommandPayloadBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
  }
  impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> CommandPayloadBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_channel_uuid(&mut self, channel_uuid: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(CommandPayload::VT_CHANNEL_UUID, channel_uuid);
    }
    #[inline]
    pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(CommandPayload::VT_NAME, name);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> CommandPayloadBuilder<'a, 'b, A> {
      let start = _fbb.start_table();
      CommandPayloadBuilder {
        fbb_: _fbb,
        start_: start,
      }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<CommandPayload<'a>> {
      let o = self.fbb_.end_table(self.start_);
      self.fbb_.required(o, CommandPayload::VT_CHANNEL_UUID,"channel_uuid");
      self.fbb_.required(o, CommandPayload::VT_NAME,"name");
      flatbuffers::WIPOffset::new(o.value())
    }
  }

  impl core::fmt::Debug for CommandPayload<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      let mut ds = f.debug_struct("CommandPayload");
      ds.field("channel_uuid", &self.channel_uuid());
      ds.field("name", &self.name());
      ds.finish()
    }
  }
  pub enum MessageOffset {}
  #[derive(Copy, Clone, PartialEq)]

  pub struct Message<'a> {
    pub _tab: flatbuffers::Table<'a>,
  }

  impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
  }

  impl<'a> Message<'a> {
    pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
    pub const VT_CONTENT_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_CONTENT: flatbuffers::VOffsetT = 8;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
      Message { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
      _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
      args: &'args MessageArgs<'args>
    ) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      if let Some(x) = args.content { builder.add_content(x); }
      if let Some(x) = args.type_ { builder.add_type_(x); }
      builder.add_content_type(args.content_type);
      builder.finish()
    }


    #[inline]
    pub fn type_(&self) -> &'a str {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_TYPE_, None).unwrap()}
    }
    #[inline]
    pub fn content_type(&self) -> Content {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<Content>(Message::VT_CONTENT_TYPE, Some(Content::NONE)).unwrap()}
    }
    #[inline]
    pub fn content(&self) -> flatbuffers::Table<'a> {
      // Safety:
      // Created from valid Table for this object
      // which contains a valid value in this slot
      unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Message::VT_CONTENT, None).unwrap()}
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn content_as_recording_payload(&self) -> Option<RecordingPayload<'a>> {
      if self.content_type() == Content::RecordingPayload {
        let u = self.content();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid union in this slot
        Some(unsafe { RecordingPayload::init_from_table(u) })
      } else {
        None
      }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn content_as_command_payload(&self) -> Option<CommandPayload<'a>> {
      if self.content_type() == Content::CommandPayload {
        let u = self.content();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid union in this slot
        Some(unsafe { CommandPayload::init_from_table(u) })
      } else {
        None
      }
    }

  }

  impl flatbuffers::Verifiable for Message<'_> {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      v.visit_table(pos)?
          .visit_field::<flatbuffers::ForwardsUOffset<&str>>("type_", Self::VT_TYPE_, true)?
          .visit_union::<Content, _>("content_type", Self::VT_CONTENT_TYPE, "content", Self::VT_CONTENT, true, |key, v, pos| {
            match key {
              Content::RecordingPayload => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RecordingPayload>>("Content::RecordingPayload", pos),
              Content::CommandPayload => v.verify_union_variant::<flatbuffers::ForwardsUOffset<CommandPayload>>("Content::CommandPayload", pos),
              _ => Ok(()),
            }
          })?
          .finish();
      Ok(())
    }
  }
  pub struct MessageArgs<'a> {
    pub type_: Option<flatbuffers::WIPOffset<&'a str>>,
    pub content_type: Content,
    pub content: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
  }
  impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
      MessageArgs {
        type_: None, // required field
        content_type: Content::NONE,
        content: None, // required field
      }
    }
  }

  pub struct MessageBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
  }
  impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MessageBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<&'b  str>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_TYPE_, type_);
    }
    #[inline]
    pub fn add_content_type(&mut self, content_type: Content) {
      self.fbb_.push_slot::<Content>(Message::VT_CONTENT_TYPE, content_type, Content::NONE);
    }
    #[inline]
    pub fn add_content(&mut self, content: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
      self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_CONTENT, content);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MessageBuilder<'a, 'b, A> {
      let start = _fbb.start_table();
      MessageBuilder {
        fbb_: _fbb,
        start_: start,
      }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
      let o = self.fbb_.end_table(self.start_);
      self.fbb_.required(o, Message::VT_TYPE_,"type_");
      self.fbb_.required(o, Message::VT_CONTENT,"content");
      flatbuffers::WIPOffset::new(o.value())
    }
  }

  impl core::fmt::Debug for Message<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      let mut ds = f.debug_struct("Message");
      ds.field("type_", &self.type_());
      ds.field("content_type", &self.content_type());
      match self.content_type() {
        Content::RecordingPayload => {
          if let Some(x) = self.content_as_recording_payload() {
            ds.field("content", &x)
          } else {
            ds.field("content", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        Content::CommandPayload => {
          if let Some(x) = self.content_as_command_payload() {
            ds.field("content", &x)
          } else {
            ds.field("content", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("content", &x)
        },
      };
      ds.finish()
    }
  }
  #[inline]
  /// Verifies that a buffer of bytes contains a `Message`
  /// and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_message_unchecked`.
  pub fn root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<Message>(buf)
  }
  #[inline]
  /// Verifies that a buffer of bytes contains a size prefixed
  /// `Message` and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `size_prefixed_root_as_message_unchecked`.
  pub fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<Message>(buf)
  }
  #[inline]
  /// Verifies, with the given options, that a buffer of bytes
  /// contains a `Message` and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_message_unchecked`.
  pub fn root_as_message_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
  ) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<Message<'b>>(opts, buf)
  }
  #[inline]
  /// Verifies, with the given verifier options, that a buffer of
  /// bytes contains a size prefixed `Message` and returns
  /// it. Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_message_unchecked`.
  pub fn size_prefixed_root_as_message_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
  ) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<Message<'b>>(opts, buf)
  }
  #[inline]
  /// Assumes, without verification, that a buffer of bytes contains a Message and returns it.
  /// # Safety
  /// Callers must trust the given bytes do indeed contain a valid `Message`.
  pub unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message {
    flatbuffers::root_unchecked::<Message>(buf)
  }
  #[inline]
  /// Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
  /// # Safety
  /// Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
  pub unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message {
    flatbuffers::size_prefixed_root_unchecked::<Message>(buf)
  }
  #[inline]
  pub fn finish_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
    fbb.finish(root, None);
  }

  #[inline]
  pub fn finish_size_prefixed_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Message<'a>>) {
    fbb.finish_size_prefixed(root, None);
  }
}  // pub mod recording


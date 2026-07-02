//! 视频笔记

pub mod action;
pub mod info;
pub mod list;
pub mod params;

pub use action::NoteAddParams;
pub use params::{
    NoteArchiveListParams, NoteIsForbidParams, NotePrivateInfoParams, NotePublicArchiveListParams,
    NotePublicInfoParams, NoteUserPrivateListParams, NoteUserPublicListParams,
};

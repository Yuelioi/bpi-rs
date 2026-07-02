//! 收藏夹

pub mod action;
pub mod info;
pub mod list;
pub mod params;

pub use list::FavListDetailParams;
pub use params::{
    FavCollectedListParams, FavCreatedListParams, FavFolderInfoParams, FavResourceIdsParams,
    FavResourceInfosParams,
};

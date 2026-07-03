//! 收藏夹

pub mod action;
mod client;
pub mod info;
pub mod list;
pub mod params;

pub use client::FavClient;
pub use list::FavListDetailParams;
pub use params::{
    FavCollectedListParams, FavCreatedListParams, FavFolderInfoParams, FavResourceIdsParams,
    FavResourceInfosParams,
};

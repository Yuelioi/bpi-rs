//! 搜索

pub mod hot;

mod result;
mod search_params;
pub mod suggest;

mod typed;

pub use search_params::{
    CategoryId, Duration, OrderSort, SearchOrder, SearchVideoParams, UserType,
};

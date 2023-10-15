use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub mod actions;
pub mod models;
pub mod schema;

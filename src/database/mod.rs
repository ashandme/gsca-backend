use diesel::{prelude::*, mysql::MysqlConnection, r2d2::{ConnectionManager, Pool}};
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub mod models;
pub mod schema;
pub mod actions;

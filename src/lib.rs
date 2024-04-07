pub mod console;
pub mod db;
pub mod file;
pub mod time_util;

pub enum Migrations {
    UP,
    DOWN,
}

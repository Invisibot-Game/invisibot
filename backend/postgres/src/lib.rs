#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Persistence handler for postgres, storing games in a postgresql database.

#[doc = "inline"]
pub mod db_connection;
#[doc = "inline"]
pub mod postgres_error;
#[doc = "inline"]
pub mod postgres_handler;

mod repositories;
mod services;
mod tables;

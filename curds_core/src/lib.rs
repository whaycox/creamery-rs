mod whey;
pub mod io;
pub mod logger;
pub mod cli;
pub mod time;
pub mod cron;

pub use curds_core_macro::whey_mock;

#[cfg(test)]
use std::{
    cell::Cell, 
    error::Error,
    fmt::Display,
    marker::PhantomData,
};
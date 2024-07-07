mod whey;
//mod message_dispatch;
pub mod cli;
pub mod time;
pub mod cron;

use curds_core_macro::*;

#[cfg(test)]
use std::{
    cell::Cell, 
    error::Error,
    fmt::Display,
    marker::PhantomData,
};
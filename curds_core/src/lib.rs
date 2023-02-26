mod dependency_injection;
mod whey;
//mod message_dispatch;
//mod cli;
//mod time;

use curds_core_abstraction::{dependency_injection::*, message_dispatch::*, whey::*};
use curds_core_macro::*;

use std::{
    rc::Rc,
    sync::RwLock,
};

#[cfg(test)]
use curds_core_abstraction::whey::*;

#[cfg(test)]
use std::{
    cell::Cell, 
    error::Error,
    fmt::Display,
    marker::PhantomData,
};
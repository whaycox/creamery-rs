mod dependency_injection;
mod message_dispatch;
mod cli;

use curds_core_abstraction::{dependency_injection::*, message_dispatch::*};
use curds_core_macro::*;

use std::{
    rc::Rc,
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
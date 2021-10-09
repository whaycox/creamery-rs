mod dependency_injection;
mod message_dispatch;

#[cfg(test)]
use curds_core_abstraction::{dependency_injection::*, message_dispatch::*};

#[cfg(test)]
use curds_core_macro::*;

#[cfg(test)]
use std::{
    rc::Rc, 
    cell::Cell, 
    error::Error,
    fmt::Display,
};
use super::*;

mod mock;
mod fields;
mod impls;

use fields::*;
use impls::*;

pub use mock::*;

use proc_macro2::TokenStream;

fn testing_struct_name(ident: &Ident) -> Ident { format_ident!("Testing{}", ident) }

fn expect_calls_method(ident: &Ident) -> Ident { format_ident!("expect_calls_{}", ident) }
fn store_expected_input_method(ident: &Ident) -> Ident { format_ident!("store_expected_input_{}", ident) }
fn default_return_method(ident: &Ident) -> Ident { format_ident!("default_return_{}", ident) }
fn store_return_method(ident: &Ident) -> Ident { format_ident!("store_return_{}", ident) }

fn expected_calls_field(ident: &Ident) -> Ident { format_ident!("expected_calls_{}", ident) }
fn recorded_calls_field(ident: &Ident) -> Ident { format_ident!("recorded_calls_{}", ident) }
fn default_generator_field(ident: &Ident) -> Ident { format_ident!("default_generator_{}", ident) }
fn expected_input_field(ident: &Ident) -> Ident { format_ident!("expected_input_{}", ident) }
fn expected_input_times_field(ident: &Ident) -> Ident { format_ident!("expected_input_times_{}", ident) }
fn returned_field(ident: &Ident) -> Ident { format_ident!("returned_{}", ident) }
fn returned_times_field(ident: &Ident) -> Ident { format_ident!("returned_times_{}", ident) }
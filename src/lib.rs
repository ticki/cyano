#![feature(question_mark, dotdot_in_tuple_patterns, box_patterns, rustc_private, str_escape)]

extern crate rustc;
extern crate rustc_data_structures;

pub mod codegen;
pub mod compiler;
pub mod cell;

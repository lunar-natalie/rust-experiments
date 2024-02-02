#![feature(trait_alias)]
pub trait Sortable = PartialOrd + Clone + Copy;

pub mod sort;

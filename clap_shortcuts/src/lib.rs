#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use anyhow::Result;
use clap::ValueEnum;
pub extern crate clap;

/// this trait allows to apply a function with params P on self depending of a variant clap::ValueEnum.
/// Three methods exist to take self only as needed.
/// They all return a Result so that the errors of the function can be propagated.
pub trait ShortCuts<Args> {
    /// this method only takes a immutable borrow.
    fn shortcut_ref(&self, shortcut: &impl ValueEnum, params: Args) -> Result<()>;
    /// this method takes a mutable borrow.
    fn shortcut_mut(&mut self, shortcut: &impl ValueEnum, params: Args) -> Result<()>;
    /// this method takes ownership.
    fn shortcut_owned(self, shortcut: &impl ValueEnum, params: Args) -> Result<()>;
}

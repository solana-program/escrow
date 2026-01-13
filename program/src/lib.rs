#![no_std]

extern crate alloc;

use pinocchio::address::declare_id;

pub mod errors;
pub mod traits;
pub mod utils;

pub mod events;
pub mod instructions;
pub mod processors;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

declare_id!("GokvZqD2yP696rzNBNbQvcZ4VsLW7jNvFXU1kW9m7k83");

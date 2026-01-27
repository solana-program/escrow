//! # Escrow Program
//!
//! A Solana program for managing token escrows with optional timelocks,
//! hooks, and mint allowlists.
//!
//! ## Features
//! - Mint allowlisting per escrow
//! - Optional withdrawal timelock
//! - Custom pre/post hooks for deposit/withdraw
//! - Token-2022 extension blocking
//!
//! ## Architecture
//! Built with Pinocchio (no_std). Clients auto-generated via Codama.

#![no_std]

extern crate alloc;

use pinocchio::address::declare_id;

pub mod errors;
pub mod traits;
pub mod utils;

pub mod events;
pub mod instructions;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

declare_id!("GokvZqD2yP696rzNBNbQvcZ4VsLW7jNvFXU1kW9m7k83");

// Copyright 2015-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! This crate serves as interface between native frontend and Rust code. Try to avoid placing any
//! logic here, just interfacing. When porting to new platform, all Rust changes will probably
//! happen here.

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::let_unit_value)]

use std::{fmt::Display, str::FromStr};

/// Container for severe error message
///
/// TODO: implement properly or remove completely
#[derive(Debug)]
pub enum ErrorDisplayed {
    /// String description of error
    Str {
        /// Error description
        s: String,
    },
}

impl From<anyhow::Error> for ErrorDisplayed {
    fn from(e: anyhow::Error) -> Self {
        Self::Str {
            s: format!("error on signer side: {}", e),
        }
    }
}

impl FromStr for ErrorDisplayed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ErrorDisplayed::Str { s: s.to_string() })
    }
}

impl From<String> for ErrorDisplayed {
    fn from(s: String) -> Self {
        ErrorDisplayed::Str { s }
    }
}

impl Display for ErrorDisplayed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

/// Determines estimated required number of multiframe QR that should be gathered before decoding
/// is attempted
pub fn qrparser_get_packets_total(data: &str, cleaned: bool) -> anyhow::Result<u32, ErrorDisplayed> {
    qr_reader_phone::get_length(data, cleaned).map_err(Into::into)
}

/// Attempts to convert QR data (transfered as json-like string) into decoded but not parsed UOS
/// payload
///
/// `cleaned` is platform-specific flag indicating whether QR payloads have QR prefix stripped by
/// QR parsing code
pub fn qrparser_try_decode_qr_sequence(
    data: &str,
    cleaned: bool,
) -> anyhow::Result<String, anyhow::Error> {
    qr_reader_phone::decode_sequence(data, cleaned)
}

/// Must be called once on normal first start of the app upon accepting conditions; relies on old
/// data being already removed
pub fn history_init_history_with_cert(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::cold_default::signer_init_with_cert(dbname).map_err(|e| format!("{}", e))
}

/// Must be called once upon jailbreak (removal of general verifier) after all old data was removed
pub fn history_init_history_no_cert(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::cold_default::signer_init_no_cert(dbname).map_err(|e| format!("{}", e))
}

/// Must be called every time network detector detects network. Sets alert flag in database that could
/// only be reset by full reset or calling [`history_acknowledge_warnings`]
///
/// This changes log, so it is expected to fail all operations that check that database remained
/// intact
fn history_device_was_online(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::device_was_online(dbname).map_err(|e| format!("{}", e))
}

/// Checks if network alert flag was set
fn history_get_warnings(dbname: &str) -> anyhow::Result<bool, String> {
    db_handling::helpers::get_danger_status(dbname).map_err(|e| format!("{}", e))
}

/// Resets network alert flag; makes record of reset in log
fn history_acknowledge_warnings(dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::reset_danger_status_to_safe(dbname).map_err(|e| format!("{}", e))
}

/// Must be called every time seed backup shows seed to user
///
/// Makes record in log
fn history_seed_name_was_shown(seed_name: &str, dbname: &str) -> anyhow::Result<(), String> {
    db_handling::manage_history::seed_name_was_shown(dbname, seed_name.to_string())
        .map_err(|e| format!("{}", e))
}

/// Must be called once to initialize logging from Rust in development mode.
///
/// Do not use in production.
#[cfg(target_os = "android")]
fn init_logging(tag: String) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace) // limit log level
            .with_tag(tag) // logs will show under mytag tag
            .with_filter(
                // configure messages for specific crate
                android_logger::FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );
}

/// Placeholder to init logging on non-android platforms
///
/// TODO: is this used?
#[cfg(not(target_os = "android"))]
fn init_logging(_tag: String) {
    env_logger::init();
}

#[cfg(test)]
mod tests {
    //use super::*;
}
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! All logs are passed to a `GlobalLog` object that then stores/outputs them.

use crate::limine;
use arrayvec::ArrayString;
use core::fmt;
use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;

/// The selected logger at compile time. It must implement a `new()` function that returns `Self` &
/// the `core::fmt::Write` trait.
type GlobalLog = StaticLog;

lazy_static! {
    /// Global object that stores the whole kernel log runtime
    static ref GLOBAL_LOG: Mutex<GlobalLog> = Mutex::new(GlobalLog::new());
}

/// Panic message when `GlobalLog.write_str()` fails. See more in the current `GlobalLog`
/// implementation.
const PRINT_PANIC: &'static str = "Could not write to GLOBAL_LOG!";

/// Used in the `log!()` macro as utility function to reach `GLOBAL_LOG`
pub fn print(msg: Arguments) {
    GLOBAL_LOG.lock().write_fmt(msg).expect(PRINT_PANIC)
}

/// Main macro used to log data, similar syntax to the standart `print!()`
///
/// WARNING: In newer rust version using padding -> blocks the main thread for an uknown reason
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::log::print(format_args!($($arg)*)));
}

// Static Log implementation

use crate::config::LOG_STATIC_CAPACITY;

/// Simple implementation of `GlobalLog` with a static size/limit.
///
/// This writes all info to `limine::print_bytes` and stores it in a static buffer.
/// One big issue with this is that if the buffer fills using `log!()` will cause a `PRINT_PANIC`.
/// The only possible fix is to increase `LOG_STATIC_CAPACITY`,
/// recompile and hope it does not fill again.
struct StaticLog {
    content: ArrayString<LOG_STATIC_CAPACITY>,
}

impl StaticLog {
    fn new() -> Self {
        Self {
            content: ArrayString::<LOG_STATIC_CAPACITY>::new(),
        }
    }
}

impl Write for StaticLog {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        crate::driver::serial::write(s);
        if s.len() > self.content.remaining_capacity() {
            return Err(fmt::Error);
        }
        self.content.push_str(s);
        Ok(())
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::limine;
use arrayvec::ArrayString;
use core::fmt;
use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;

type GlobalLog = StaticLog;

lazy_static! {
    static ref GLOBAL_LOG: Mutex<GlobalLog> = Mutex::new(GlobalLog::new());
}

pub fn print(msg: Arguments) {
    GLOBAL_LOG
        .lock()
        .write_fmt(msg)
        .expect("Could not write to GLOBAL_LOG!")
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::log::print(format_args!($($arg)*)));
}

// Static Log implementation

const STATIC_LOG_MAX_CHARACTERS: usize = 65535;

struct StaticLog {
    content: ArrayString<STATIC_LOG_MAX_CHARACTERS>,
}

impl StaticLog {
    fn new() -> Self {
        Self {
            content: ArrayString::<STATIC_LOG_MAX_CHARACTERS>::new(),
        }
    }
}

impl Write for StaticLog {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > self.content.remaining_capacity() {
            return Err(fmt::Error);
        }
        limine::print_bytes(s.as_bytes());
        self.content.push_str(s);
        Ok(())
    }
}

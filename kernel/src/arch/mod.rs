/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


pub enum ArchType {
    X86_64,
    AArch64,
}

#[cfg(target_arch = "x86_64")] 
mod amd64;
#[cfg(target_arch = "x86_64")] 
pub use amd64::*;

#[cfg(target_arch = "aarch64")]
mod arm64;
#[cfg(target_arch = "aarch64")]
pub use arm64::*;


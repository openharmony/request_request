// Copyright (c) 2023 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A simple fast pseudorandom implementation.
//! 
//! This module provides a fast thread-local pseudorandom number generator based
//! on the xorshift* algorithm. It produces random values in the range from 0 to
//! usize::MAX.
//! 
//! Reference: xorshift* <https://dl.acm.org/doi/10.1145/2845077>

use std::cell::Cell;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::num::Wrapping;

/// Generates a pseudorandom 64-bit unsigned integer.
///
/// Returns a random value in the range [0, u64::MAX] using the xorshift*
/// algorithm with a period of 2^64-1.
///
/// # Examples
///
/// ```rust
/// use request_utils::fastrand::fast_random;
///
/// let random_value = fast_random();
/// println!("Random value: {}", random_value);
/// 
/// // Generate a random value within a specific range
/// let bounded_value = random_value % 100; // Value in [0, 99]
/// ```
///
/// # Notes
///
/// This implementation uses a thread-local random number generator with
/// automatic seeding. The xorshift* algorithm is chosen for its excellent
/// speed-to-quality ratio for non-cryptographic purposes.
///
/// For cryptographic applications, consider using the standard library's
/// `rand::Rng` with a secure random number generator instead.
pub fn fast_random() -> u64 {
    thread_local! {
        static RNG: Cell<Wrapping<u64>> = Cell::new(Wrapping(seed()));
    }

    RNG.with(|rng| {
        let mut s = rng.get();
        // Xorshift* algorithm steps
        s ^= s >> 12; // Shift and XOR operations to generate non-linear behavior
        s ^= s << 25;
        s ^= s >> 27;
        rng.set(s);
        // Multiply by a large prime to improve distribution properties
        s.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    })
}

/// Generates a non-zero seed for the random number generator.
///
/// Uses the standard library's `RandomState` to generate a seed value that is
/// guaranteed to be non-zero to ensure proper operation of the xorshift*
/// algorithm.
///
/// # Notes
///
/// The seed generation is designed to continue hashing incrementing values
/// until a non-zero result is obtained.
fn seed() -> u64 {
    let seed = RandomState::new();

    let mut out = 0;
    let mut count = 0;
    // Continue hashing until a non-zero seed is obtained
    // Xorshift* requires a non-zero seed to generate a proper sequence
    while out == 0 {
        count += 1;
        let mut hasher = seed.build_hasher();
        hasher.write_usize(count);
        out = hasher.finish();
    }
    out
}

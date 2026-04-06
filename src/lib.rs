#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

#[cfg(test)]
mod tests;

/// MSEED is based on an approximation of the golden ratio (phi):
/// MSEED ≈ φ * 10^8
const MSEED: i32 = 161_803_398;

/// A deterministic random number generator.
///
/// This RNG algorithm is used by .NET and is
/// based on Knuth's subtractive method.
///
/// For more information, see the [module level documentation](self).
#[derive(Clone)]
pub struct DotnetRng {
    seed_array: [i32; 56],
    inext: u8,
    inextp: u8,
}

impl DotnetRng {
    /// Creates a new [`DotnetRng`] instance based on the given seed.
    ///
    /// All methods will return the same "random" numbers for an RNG
    /// instance with the same state. The state is determined by the seed
    /// and the number of imes [`DotnetRng::next`] has been called in total.
    #[must_use]
    pub const fn new(seed: i32) -> Self {
        let seed: i32 = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };
        let mut num1: i32 = MSEED.wrapping_sub(seed);
        let mut num2: i32 = 1;
        let mut index1: usize = 0;

        let mut seed_array = [0i32; 56];
        seed_array[55] = num1;

        let mut i: u8 = 1;
        while i < 55 {
            index1 += 21;
            if index1 >= 55 {
                index1 -= 55;
            }
            seed_array[index1] = num2;
            num2 = num1.wrapping_sub(num2);
            if num2 < 0 {
                num2 = num2.wrapping_add(i32::MAX);
            }
            num1 = seed_array[index1];
            i += 1;
        }

        i = 1;
        while i < 5 {
            let mut j: u8 = 1;
            while j < 56 {
                let mut num3: u8 = j + 30;
                if num3 >= 55 {
                    num3 -= 55;
                }

                let seed1: i32 = seed_array[j as usize];
                let seed2: i32 = seed_array[num3 as usize + 1];

                let mut num: i32 = seed1.wrapping_sub(seed2);
                if num < 0 {
                    num = num.wrapping_add(i32::MAX);
                }

                seed_array[j as usize] = num;
                j += 1;
            }
            i += 1;
        }

        Self { seed_array, inext: 0, inextp: 21 }
    }

    /// Generates a new number and throws it away.
    ///
    /// This is useful if you want to advance the RNG's internal
    /// state without actually using the generated number.
    #[inline]
    pub const fn skip(&mut self) {
        let _ = self.next();
    }

    /// Gets the next pseudorandom 32-bit signed integer based
    /// on the state of this [`DotnetRng`] instance.
    ///
    /// **Range**: [`i32::MIN`] .. [`i32::MAX`]
    ///
    /// For the same internal state, this will always return the same number.
    ///
    /// The state is simply determined by the given seed and how many times
    /// [`Self::next`] has been called before.
    #[doc(alias = "next_i32")]
    #[must_use = "if you intend to only advance the internal rng state, use `.skip()`"]
    pub const fn next(&mut self) -> i32 {
        let mut index1: u8 = self.inext + 1;
        if index1 >= 56 {
            index1 = 1;
        }

        let mut index2: u8 = self.inextp + 1;
        if index2 >= 56 {
            index2 = 1;
        }

        let seed1: i32 = self.seed_array[index1 as usize];
        let seed2: i32 = self.seed_array[index2 as usize];
        let mut num: i32 = seed1.wrapping_sub(seed2);
        if num == i32::MAX {
            num -= 1;
        }
        if num < 0 {
            num = num.wrapping_add(i32::MAX);
        }

        self.seed_array[index1 as usize] = num;
        self.inext = index1;
        self.inextp = index2;

        num
    }

    /// Gets the next signed 32-bit integer within the given range.
    ///
    /// **Range**: `min` .. `max`
    ///
    /// If the range is [`i32::MAX`] or larger, a slighty different algorithm
    /// is used which internally calls `.next()` twice instead of once.
    ///
    /// For more information on number generation, see [`DotnetRng::next`].
    ///
    /// # Panics
    /// This function will panic if `min > max`.
    #[must_use = "if you intend to only advance the internal rng state, use `.skip()`"]
    pub const fn next_ranged(&mut self, min: i32, max: i32) -> i32 {
        assert!(min <= max, "minimum is greater than maximum");

        if let Some(range) = max.checked_sub(min) {
            return (self.next_f64() * (range as f64)) as i32 + min;
        }

        // Large range; more steps needed.
        let mut sample: i32 = self.next();
        if self.next() % 2 == 0 {
            sample = -sample;
        }
        let mut num: f64 = sample as f64;
        num += (i32::MAX - 1) as f64;
        num /= (2 * (i32::MAX) as u32) as f64 - 1.0;

        let range: f64 = (max as f64) - (min as f64);
        (num * range) as i32 + min
    }

    /// Gets the next double-precision floating point number.
    ///
    /// **Range**: 0 .. 1
    ///
    /// For more information on number generation, see [`DotnetRng::next`].
    #[doc(alias = "next_double")]
    #[inline]
    #[must_use = "if you intend to only advance the internal rng state, use `.skip()`"]
    pub const fn next_f64(&mut self) -> f64 {
        self.next() as f64 * (1.0 / i32::MAX as f64)
    }

    /// Fills a given buffer with random bytes.
    ///
    /// For each byte, `.next()` is called and its return value is truncated to
    /// an unsigned 8-bit integer. The internal state is therefore advanced
    /// `buffer.len()` times.
    ///
    /// If you have a known array size at compile-time, consider using
    /// [`DotnetRng::next_bytes`] instead.
    ///
    /// For more information on number generation, see [`DotnetRng::next`].
    pub const fn fill_bytes(&mut self, buffer: &mut [u8]) {
        let mut i = 0;
        while i < buffer.len() {
            buffer[i] = self.next() as u8;
            i += 1;
        }
    }

    /// Creates and fills a buffer with random bytes.
    ///
    /// For each byte, `.next()` is called and its return value is truncated to
    /// an unsigned 8-bit integer. The internal state is therefore advanced
    /// `N` times.
    ///
    /// If you do not have a known array size at compile-time, consider using
    /// [`DotnetRng::fill_bytes`].
    ///
    /// For more information on number generation, see [`DotnetRng::next`].
    #[must_use = "if you intend to only advance the internal rng state, use `.skip()`"]
    pub const fn next_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut buffer = [0u8; N];
        self.fill_bytes(&mut buffer);
        buffer
    }
}

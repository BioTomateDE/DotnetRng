#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::cargo, missing_docs)]

/// MSEED is based on an approximation of the golden ratio (phi):
/// MSEED ≈ φ * 10^8
const MSEED: i32 = 161_803_398;

/// The random number generator used by **.NET** for seeded RNG.
///
/// All instances of this struct initialized with
/// the same seed will always produce the same values.
///
/// This struct's size is 228 bytes on all platforms and architectures.
///
/// For more information, see the [module level documentation](self).
#[derive(Clone)]
pub struct DotnetRng {
    seed_array: [i32; 56],
    inext: u8,
    inextp: u8,
}

impl DotnetRng {
    /// Creates a new .NET random number generator with the given seed.
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

        // TODO(const-hack): This can be replaced with a for-loop when it is const-stable.
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

        Self {
            seed_array,
            inext: 0,
            inextp: 21,
        }
    }

    /// Generates the next 32-bit signed integer.
    ///
    /// The generated number is never negative and never exactly [`i32::MAX`].
    ///
    /// # Related
    /// This function is equivalent to the C# method overload `Next()`.
    ///
    /// # Notes
    /// This function is the base for all other functions.
    /// Calling `.next()` one time means advancing the internal state of the RNG once.
    #[doc(alias = "next_i32")]
    #[doc(alias = "next_int")]
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

    /// Generates the next signed 32-bit integer within the given range.
    ///
    /// The generated number is greater or equal to `min` and less than `max`.
    /// Exception: If `min == max`, then `min` is returned.
    ///
    /// # Related
    /// This function is equivalent to the C# method overload `Next(int minValue, int maxValue)`.
    /// The overload `Next(int maxValue)` can be simulated using `next_range(0, max)`.
    ///
    /// # Notes
    /// If the range is larger than [`i32::MAX`], a slightly different algorithm
    /// is used which internally calls `.next()` twice instead of once.
    ///
    /// # Panics
    /// This function will panic if `min > max`.
    #[doc(alias = "next_ranged")]
    #[doc(alias = "next_between")]
    #[must_use]
    pub const fn next_range(&mut self, min: i32, max: i32) -> i32 {
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
        num /= (2 * (i32::MAX) as u64) as f64 - 1.0;

        let range: f64 = (max as f64) - (min as f64);
        ((num * range) as i64 + min as i64) as i32
    }

    const fn next_u64(&mut self) -> u64 {
        let low = self.next_range(0, 1 << 22) as u64;
        let mid = self.next_range(0, 1 << 22) as u64;
        let high = self.next_range(0, 1 << 20) as u64;
        low | (mid << 22) | (high << 44)
    }

    /// Generates the next 64-bit signed integer.
    ///
    /// The generated number is never negative and never exactly [`i64::MAX`].
    ///
    /// # Related
    /// This function is equivalent to the C# method overload `NextInt64()`.
    ///
    /// # Notes
    /// In most cases, this will call `self.next()` thrice.
    /// However, it is possible that the generated number is exactly
    /// [`i64::MAX`], which is rejected and causes another iteration.
    #[doc(alias = "next_int64")]
    #[doc(alias = "next_int_64")]
    #[must_use]
    pub const fn next_i64(&mut self) -> i64 {
        loop {
            let num = (self.next_u64() >> 1) as i64;
            if num != i64::MAX {
                return num;
            }

            // The chances of this happening is 1 in 9223372036854775807.
            cold();
        }
    }

    /// Generates the next 64-bit signed integer within the given range.
    ///
    /// The generated number is greater or equal to `min` and less than `max`.
    /// Exception: If `min == max`, then `min` is returned.
    ///
    /// # Related
    /// This function is equivalent to the C# method overload `NextInt64(long minValue, long maxValue)`.
    /// The overload `NextInt64(int maxValue)` can be simulated using `next_i64_range(0, max)`.
    ///
    /// # Panics
    /// This function will panic if `min > max`.
    #[doc(alias = "next_range_i64")]
    #[must_use]
    pub const fn next_i64_range(&mut self, min: i64, max: i64) -> i64 {
        assert!(min <= max, "minimum is greater than maximum");
        let range = max.wrapping_sub(min) as u64;
        if range <= 1 {
            return min;
        }

        let bits = range.ilog2() + !range.is_power_of_two() as u32;
        loop {
            let num = self.next_u64();
            let shift = u64::BITS - bits;
            let result = num >> shift;
            if result < range {
                return (result as i64).wrapping_add(min);
            }
        }
    }

    /// Generates the next double-precision floating point number.
    ///
    /// The generated number is greater or equal to 0 and less than 1.
    ///
    /// # Related
    /// This function is equivalent to the C# method `NextDouble()`.
    #[doc(alias = "next_double")]
    #[inline]
    #[must_use = "if you intend to only advance the internal rng state, use `.skip()`"]
    pub const fn next_f64(&mut self) -> f64 {
        self.next() as f64 * (1.0 / i32::MAX as f64)
    }

    /// Generates the next single-precision floating point number.
    ///
    /// The generated number is greater or equal to 0 and less than 1.
    ///
    /// # Related
    /// This function is equivalent to the C# method `NextSingle()`.
    ///
    /// # Notes
    /// In most cases, this will only call `self.next()` once.
    /// However, it is possible that the generated double rounds to `1.0`
    /// when rounded to a single-precision float, which is rejected and causes another iteration.
    #[doc(alias = "next_single")]
    #[doc(alias = "next_float")]
    #[must_use]
    pub const fn next_f32(&mut self) -> f32 {
        loop {
            let num = self.next_f64() as f32;
            if num <= 1.0 {
                return num;
            }

            // Very low chance for the number to round to 1.0
            cold();
        }
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
    /// # Related
    /// This function is equivalent to the C# method `NextBytes(Byte[])`.
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
    #[inline]
    #[must_use = "if you intend to only advance the internal rng state, use `.skip_n()`"]
    pub const fn next_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut buffer = [0u8; N];
        self.fill_bytes(&mut buffer);
        buffer
    }

    /// Generates a new number and throws it away.
    ///
    /// This is useful if you want to advance the RNG's internal
    /// state without actually using the generated number.
    ///
    /// This function is equivalent to:
    /// ```
    /// # let mut rng = dotnet_rng::DotnetRng::new(0);
    /// let _ = rng.next();
    /// ```
    #[inline]
    pub const fn skip(&mut self) {
        let _ = self.next();
    }

    /// Generates `count` numbers and throws them away.
    ///
    /// This is useful if you want to advance the RNG's internal
    /// state without actually using the generated numbers.
    ///
    /// This function is equivalent to:
    /// ```
    /// # let mut rng = dotnet_rng::DotnetRng::new(0);
    /// # let count = 8;
    /// for i in 0..count {
    ///     let _ = rng.next();
    /// }
    /// ```
    #[inline]
    pub const fn skip_n(&mut self, count: u32) {
        let mut i = 0;
        while i < count {
            self.skip();
            i += 1;
        }
    }
}

// Compile-time tests
const fn _assert() {
    use core::panic::{RefUnwindSafe, UnwindSafe};
    const fn require<T: Clone + Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>() {}
    require::<DotnetRng>();
}

const _: () = assert!(size_of::<DotnetRng>() == 228);

// This can be replaced by `core::hint::cold_path` when MSRV >= 1.95.0
#[inline(always)]
#[cold]
const fn cold() {}

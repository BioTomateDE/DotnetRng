# dotnet-rng

A Rust implementation of .NET's Random algorithm based on Knuth's subtractive method.

This crate is:

* tested to match exactly with .NET generated values
* extremely lightweight (no dependencies, low amount of code)
* entirely usable in constant evaluation (all functions are marked as `const`)
* compatible with embedded systems (`no_std` and `no_alloc`)
* platform-independent (no usage of `usize` or pointers in `struct`s)

## Usage

```rust
use dotnet_rng::DotnetRng;

// Create a new RNG instance with a given seed
let mut rng = DotnetRng::new(1337);

// Generate integer between [0, 2147483647)
let int: i32 = rng.next();

// Generate integer between [100, 200)
let int: i32 = rng.next_range(100, 200);

// Generate long between [0, 9223372036854775807)
let long: i64 = rng.next_i64();

// Generate long between [-69, 420)
let long: i64 = rng.next_i64_range(-69, 420);

// Generate double-precision float between [0, 1)
let double: f64 = rng.next_f64();

// Generate 64 random bytes
let bytes: [u8; 64] = rng.next_bytes();

// Fill existing byte buffer
let mut buffer = [0u8; 187];
rng.fill_bytes(&mut buffer);
println!("Bytes: {buffer:?}");

// Advance internal state
rng.skip();  // Same as let _ = rng.next();

// RNG is deterministic
let mut new_rng = rng.clone();
assert_eq!(rng.next(), new_rng.next());
assert_eq!(rng.next_f64(), new_rng.next_f64());
```

## Purpose

This crate is not intended to be the best or most efficient random number generator.
Its purpose is to mimic .NET's RNG algorithm exactly.

If you just need random numbers, you should use the `rand` crate.
If you need to get the exact pseudorandom values that you
would get for the same seed in .NET, then this crate is for you.

## Reference

The RNG algorithm is taken from the C# System library:
[Archive file](https://github.com/microsoft/referencesource/blob/main/mscorlib/system/random.cs)
|
[Current file](https://github.com/dotnet/dotnet/blob/main/src/runtime/src/libraries/System.Private.CoreLib/src/System/Random.CompatImpl.cs).

## License

The original algorithm was made by Microsoft.
This Rust port was made by BioTomateDE.

This crate is re-licensed under the [MIT license](https://opensource.org/license/mit).
See the attached LICENSE file for more information.

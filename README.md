[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/osrandom/badge.svg)](https://docs.rs/osrandom)
[![crates.io](https://img.shields.io/crates/v/osrandom.svg)](https://crates.io/crates/crypto_api_osrandom)
[![Download numbers](https://img.shields.io/crates/d/osrandom.svg)](https://crates.io/crates/osrandom)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/osrandom?svg=true)](https://ci.appveyor.com/project/KizzyCode/osrandom)
[![dependency status](https://deps.rs/crate/osrandom/0.2.0/status.svg)](https://deps.rs/crate/osrandom/0.2.0)


# osrandom
Welcome to `osrandom` 🎉

This crate provides access to your operating system's cryptographically secure random number generator.


## APIs used
The following native APIs are used:
 - macOS/iOS: `SecRandomCopyBytes` from the security framework
 - FreeBSD/OpenBSD/NetBSD: `arc4random_buf` (which does not use ARC4 anymore but a secure PRF like
   [ChaCha20](https://cr.yp.to/chacha.html))
 - Windows: `CryptGenRandom` with `PROV_RSA_FULL` as provider
 - Linux-GNU: `getrandom` for glibc versions >= 2.25 or `/dev/urandom` for ancient distributions
 - Linux-MUSL: `/dev/urandom` for ancient distributions

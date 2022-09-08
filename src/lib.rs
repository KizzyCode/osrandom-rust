#![doc = include_str!("../README.md")]

use std::{
    ffi::CStr,
    io::{self, ErrorKind::Other},
    os::raw::c_char,
};

/// Returns the name of the underlying provider
pub fn provider() -> &'static str {
    // The API of the bridge library
    extern "C" {
        // const char* osrandom_provider();
        fn osrandom_provider() -> *const c_char;
    }

    // Get the provider
    let ptr = unsafe { osrandom_provider() };
    let cstr = unsafe { CStr::from_ptr(ptr) };
    match cstr.to_str() {
        Ok(provider) => provider,
        Err(e) => unreachable!("Invalid provider: {e}"),
    }
}

/// Fills the given slice with crpytographically secure random bytes
pub fn to_slice(slice: &mut [u8]) -> io::Result<()> {
    // The API of the bridge library
    extern "C" {
        // uint8_t crypto_api_osrandom(uint8_t* buf, size_t len);
        fn osrandom_native(buf: *mut u8, len: usize) -> u8;
    }

    // Call the native implementation
    let result = unsafe { osrandom_native(slice.as_mut_ptr(), slice.len()) };
    match result {
        0 => Ok(()),
        _ => Err(Other.into()),
    }
}

/// Creates a `LEN`-sized array filled with crpytographically secure random bytes
pub fn to_array<const LEN: usize>() -> io::Result<[u8; LEN]> {
    let mut array = [0; LEN];
    crate::to_slice(&mut array)?;
    Ok(array)
}

/// Creates a `len`-sized vec filled with crpytographically secure random bytes
pub fn to_vec(len: usize) -> io::Result<Vec<u8>> {
    let mut vec = vec![0; len];
    crate::to_slice(&mut vec)?;
    Ok(vec)
}

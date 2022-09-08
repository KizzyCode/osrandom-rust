use cc::Build;
use std::{
    env,
    error::Error,
    io::{self, ErrorKind::InvalidData},
    process::Command,
    str,
};

/// Checks if the current glibc version is 2.25 or newer and thus supports `getrandom`
fn linux_has_getrandom() -> bool {
    // Get the glibc version
    let glibc_version = || -> Result<(u8, u8), Box<dyn Error>> {
        // Execute "lld --version" to parse the glibc version from it
        let lld_result = Command::new("ldd").arg("--version").output()?;
        let output = String::from_utf8(lld_result.stdout)?;

        // Find the line starting with "ldd (GNU libc) "
        let (major, minor) = output
            .lines()
            .find_map(|line| line.strip_prefix("ldd (GNU libc) "))
            .and_then(|version| version.split_once('.'))
            .ok_or_else(|| io::Error::from(InvalidData))?;

        // Parse the tuple
        let major_minor = (major.parse()?, minor.parse()?);
        Ok(major_minor)
    };

    // Get and check version or display error
    glibc_version()
        .map(|major_minor| matches!(major_minor, (2..=255, 25..=255)))
        .map_err(|e| eprintln!("Failed to get glibc version: {e}"))
        .unwrap_or(false)
}

/// Select the random number generator
fn select_random() -> &'static str {
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Cannot determine target architecture");
    let flavour = env::var("CARGO_CFG_TARGET_ENV").expect("Cannot determine target environment");
    match (os.as_str(), flavour.as_str()) {
        ("macos" | "ios", _) => {
            println!("cargo:rustc-link-lib=framework=Security");
            "librandom/secrandomcopybytes.c"
        }
        ("freebsd" | "openbsd" | "netbsd", _) => "librandom/arc4random.c",
        ("linux", "gnu") if linux_has_getrandom() => "librandom/getrandom.c",
        ("linux", _) => "librandom/urandom.c",
        ("windows", _) => "librandom/cryptgenrandom.c",
        (os, flavour) => panic!("Unsupported target OS: {os}-{flavour}"),
    }
}

fn main() {
    // Compile and link the library
    let provider = select_random();
    eprintln!("Selected provider: {provider}");
    Build::new().file(provider).warnings_into_errors(true).compile("helpers");
}

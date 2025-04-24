//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // For tests7: Set up TEST_FOO environment variable with current timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8: Enable "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

//! The binary's exit status, which scripts read to tell a failed run from a clean one.

use std::env;
use std::process::Command;

/// A filter outside the published events fails before any network call, so
/// it errors without a cookie or a connection.
#[test]
fn a_failed_run_exits_nonzero_with_the_reason() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustmas"))
        .args(["solve", "-y", "1999"])
        .current_dir(env::temp_dir())
        .output()
        .expect("the binary runs");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "stderr: {stderr}");
    assert!(stderr.starts_with("Error: "), "stderr: {stderr}");
    assert!(stderr.contains("year 1999"), "stderr: {stderr}");
}

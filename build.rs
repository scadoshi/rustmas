use std::{fs::read_dir, path::Path};

/// Warns when `inputs/` is missing or empty.
///
/// Solutions embed their input with `include_str!`, and `inputs/` is gitignored,
/// so a fresh clone fails with a path error that doesn't say what to do about
/// it. Only the days registered in `solutions!` are actually embedded, so this
/// checks that inputs exist at all rather than looking for a particular file.
///
/// This warns rather than panicking on purpose. A build script gates the whole
/// package, so failing here would also block `init`, which is the binary you
/// need in order to fix the problem.
fn main() {
    println!("cargo:rerun-if-changed=inputs");

    let inputs = Path::new("inputs");
    let populated = read_dir(inputs).is_ok_and(|mut entries| entries.next().is_some());

    if !populated {
        println!(
            "cargo:warning=inputs/ is missing or empty; run `cargo run --bin init` to download them"
        );
    }
}

#![feature(test)]

extern crate test;
extern crate pythonc;

use pythonc::Stage;

use test::Bencher;

#[bench]
fn intense_branching(b: &mut Bencher) {
    let pythonc = pythonc::Pythonc::new();

    let manifest_dir = ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let programs_dir = manifest_dir.join("tests/programs");

    b.iter(|| {
        pythonc.emit(
            &programs_dir.join("intense_branching.py"),
            Stage::bin,
            None,
            None,
            false,
            false).unwrap();
    });
}

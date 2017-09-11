extern crate error_chain;
extern crate pythonc;

use error_chain::ChainedError;
use pythonc::{Compiler, ErrorKind, Result, ResultExt};
use std::path::Path;
use std::path::PathBuf;
use std::fs;

#[test]
fn pyyc_tests_contrib() {
    if let Err(e) = run() {
        panic!("{}", e.display_chain());
    }
}

fn run() -> Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let runtime = manifest_dir.join("runtime/libpyyruntime.a");
    let pyyc_tests_contrib = manifest_dir.join("tests/pyyc-tests-contrib");
    let subdirs = subdirs(pyyc_tests_contrib)
        .chain_err(|| "getting subdirs of pyyc-tests-contrib")?;
    for subdir in subdirs {
        test_dir(&subdir, &runtime).chain_err(|| format!("test dir {:?}", subdir.display()))?;
    }

    Err("yello".into())
}

fn subdirs<P>(dir: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>
{
    let rd = fs::read_dir(dir).chain_err(|| "reading dir")?;
    let mut subdirs = vec![];
    for entry in rd {
        let entry = entry.chain_err(|| "reading dir entries")?;
        let file_type = entry.file_type().chain_err(|| "getting dir entry type")?;
        if file_type.is_dir() {
            subdirs.push(entry.path());
        }
    }
    return Ok(subdirs)
}

fn test_dir<P1, P2>(dir: P1, runtime: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let dir = dir.as_ref();
    let runtime = runtime.as_ref();
    let rd = fs::read_dir(dir).chain_err(|| "reading dir")?;
    for entry in rd {
        let entry = entry.chain_err(|| "reading dir entries")?;
        let path = entry.path();
        let ext = path.extension().ok_or(format!("{:?} doesn't have an extension", path.display()))?;
        if ext != "py" {
            continue
        }
        let source = &path;
        let in_file = source.with_extension("in");
        let mut compiler = Compiler::new(source);
        compiler.create_new(false);
        compiler.runtime(runtime);
        compiler.run().chain_err(|| format!("compiling {:?}", source.display()))?;
    }
    Ok(())
}

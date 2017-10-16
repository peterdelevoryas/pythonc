extern crate error_chain;
extern crate python;

use error_chain::ChainedError;
use python::{Compiler, Result, ResultExt};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::fs::File;
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
    let subdirs = subdirs(pyyc_tests_contrib).chain_err(
        || "getting subdirs of pyyc-tests-contrib",
    )?;
    for subdir in subdirs {
        test_dir(&subdir, &runtime).chain_err(|| {
            format!("test dir {:?}", subdir.display())
        })?;
    }

    Ok(())
}

fn subdirs<P>(dir: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
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
    return Ok(subdirs);
}

fn read_file(path: &Path) -> Result<String> {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open(path).chain_err(|| {
        format!("Could not open file {:?}", path.display())
    })?;
    let size = f.metadata()
        .chain_err(|| "Could not query file size")?
        .len() as usize;
    let mut s = String::with_capacity(size);
    f.read_to_string(&mut s).chain_err(
        || "Could not read file data",
    )?;
    Ok(s)
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
        let ext = path.extension();
        if ext.is_none() || ext.unwrap() != "py" {
            continue;
        }
        let source = &path;
        let compiler = Compiler::new();
        compiler
            .emit(source, python::CompilerStage::Bin, None, Some(runtime.into()))
            .chain_err(|| format!("Unable to compile {:?}", source.display()))?;

        let compiled = source.with_extension("bin");
        let input = source.with_extension("in");
        let output = source.with_extension("out");
        let expected = source.with_extension("expected");

        let input_file: Stdio = File::open(&input).map(Stdio::from).unwrap_or(Stdio::null());
        let compiled = Command::new(&compiled)
            .stdin(input_file)
            .stdout(File::create(&output).chain_err(|| "creating output file")?)
            .spawn()
            .chain_err(|| "spawning child process")?;

        let input_file: Stdio = File::open(&input).map(Stdio::from).unwrap_or(Stdio::null());
        let reference = Command::new("python")
            .arg(&source)
            .stdin(input_file)
            .stdout(File::create(&expected).chain_err(
                || "creating expected file",
            )?)
            .spawn()
            .chain_err(|| "spawning child process")?;

        compiled
            .wait_with_output()
            .chain_err(|| "waiting on compiled")
            .and_then(|output| if output.stderr.len() > 0 {
                Err(String::from_utf8(output.stderr).unwrap().into())
            } else {
                Ok(())
            })?;

        reference
            .wait_with_output()
            .chain_err(|| "waiting on reference")
            .and_then(|output| if output.stderr.len() > 0 {
                Err(String::from_utf8(output.stderr).unwrap().into())
            } else {
                Ok(())
            })?;

        diff(&output, &expected)?;
    }
    Ok(())
}

fn diff<P1, P2>(left: P1, right: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    use std::io::Read;

    let lpath = left.as_ref();
    let rpath = right.as_ref();
    let mut left = File::open(lpath).chain_err(|| "opening left")?;
    let mut right = File::open(rpath).chain_err(|| "opening right")?;
    let mut left_buf = String::new();
    let mut right_buf = String::new();
    left.read_to_string(&mut left_buf)?;
    right.read_to_string(&mut right_buf)?;

    for ((left_i, left_c), right_c) in left_buf.char_indices().zip(right_buf.chars()) {
        if left_c != right_c {
            let mismatch = format!(
                "diff {:?} {:?} at index {:?}",
                lpath.display(),
                rpath.display(),
                left_i
            );
            return Err(mismatch.into());
        }
    }
    return Ok(());
}

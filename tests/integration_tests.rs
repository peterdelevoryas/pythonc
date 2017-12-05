#[macro_use]
extern crate error_chain;
extern crate pythonc;

use error_chain::ChainedError;
use pythonc::{Pythonc, Stage, Result, ResultExt};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::fs::File;
use std::fs;

#[test]
fn test_programs() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let programs_dir = manifest_dir.join("tests/programs");

    use std::io::Write;
    if let Err(e) = run(&programs_dir, Stage::bin) {
        panic!("\n{}", e.display_chain());
    }
}

fn run(dir: &Path, stage: Stage) -> Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let runtime = manifest_dir.join("runtime/libpyyruntime.a");
    run_tests(dir, &runtime, stage).chain_err(|| format!("Test failure"))
}

fn subdirs<P>(dir: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let rd = fs::read_dir(dir).chain_err(|| {
        format!("Error reading {}", dir.display())
    })?;
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

fn run_tests<P1, P2>(dir: P1, runtime: P2, stage: Stage) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let dir = dir.as_ref();
    let runtime = runtime.as_ref();
    let rd = fs::read_dir(dir).chain_err(|| {
        format!("Error reading dir {}", dir.display())
    })?;
    for entry in rd {
        let entry = entry.chain_err(|| "reading dir entries")?;
        let path = entry.path();
        let ext = path.extension();
        if ext.is_none() || ext.unwrap() != "py" {
            continue;
        }
        let source = &path;
        let pythonc = Pythonc::new();

        let source_file_name = match source.file_name() {
            Some(file_name) => {
                match file_name.to_str() {
                    Some(s) => s,
                    None => bail!("Test source file name {:?} is not utf-8", source.display()),
                }
            }
            None => {
                bail!(
                    "Test source path {:?} doesn't have file name",
                    source.display()
                )
            }
        };

        let result = ::std::panic::catch_unwind(|| {
            pythonc
                .emit(source, stage, None, Some(runtime.into()), true, true)
                .chain_err(|| format!("Unable to compile {:?}", source_file_name))
        });

        match result {
            Ok(result) => result?,
            Err(e) => bail!("panicked while compiling {:?}", source_file_name),
        }

        if stage == Stage::bin {
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

            diff(&output, &expected).chain_err(|| {
                format!("Diff on test {}", source.display())
            })?;
        }
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

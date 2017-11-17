#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate pythonc;

use pythonc::error::*;
use std::path::PathBuf;
use clap::Arg;

quick_main!(run);

fn run() -> pythonc::Result<()> {
    env_logger::init()?;
    let m = clap::App::new("pythonc")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!("\n"))
        .about("A Python (subset) compiler written in Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Input path")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("STAGE")
                .help("Output stage")
                .takes_value(true)
                .long("emit")
                .required(false)
                .multiple(false)
                .possible_values(&pythonc::Stage::variants()),
        )
        .arg(
            Arg::with_name("FILENAME")
                .help("Out file")
                .takes_value(true)
                .short("o")
                .required(false),
        )
        .arg(
            Arg::with_name("stdout")
                .help("Write output to stdout")
                .takes_value(false)
                .long("stdout")
                .required(false)
                .conflicts_with("FILENAME"),
        )
        .arg(
            Arg::with_name("LIB")
                .help("Runtime library path (for linking)")
                .takes_value(true)
                .long("runtime")
                .required(false),
        )
        .arg(
            Arg::with_name("show_casts")
                .help("Show casts (inject and project) in explicated stages")
                .takes_value(false)
                .long("show-casts")
                .required(false),
        )
        .arg(
            Arg::with_name("show_nums")
                .help("Show global var numbering when formatting vars")
                .takes_value(false)
                .long("show-nums")
                .required(false),
        )
        .get_matches();

    let emit: pythonc::Stage = match m.value_of("STAGE") {
        Some(stage) => stage.parse()?,
        None => pythonc::Stage::Asm,
    };

    let pythonc = pythonc::Pythonc::new();

    let in_path: PathBuf = match m.value_of("INPUT") {
        Some(s) => s.into(),
        None => bail!("Missing input argument"),
    };
    let out_path = if m.is_present("stdout") {
        Some(PathBuf::from("/dev/stdout"))
    } else {
        m.value_of("FILENAME").map(PathBuf::from)
    };
    let runtime = m.value_of("LIB").map(PathBuf::from);

    if emit == pythonc::Stage::Bin {
        if runtime.is_none() {
            bail!("Cannot emit binary without runtime")
        }
    }

    let show_casts = m.is_present("show_casts");
    let show_nums = m.is_present("show_nums");

    pythonc
        .emit(&in_path, emit, out_path, runtime, show_casts, show_nums)
        .chain_err(|| format!("Could not compile {:?}", in_path.display()))
}

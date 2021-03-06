extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Rust's package manager

Usage:
    cargo <command> [<args>...]
    cargo [options]

Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output

Some common cargo commands are:
    build       Compile the current project
    clean       Remove the target directory
    doc         Build this project's and its dependencies' documentation
    new         Create a new cargo project
    run         Build and execute src/main.rs
    test        Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock

See 'cargo help <command>' for more information on a specific command.
";

#[derive(RustcDecodable, Show)]
struct Args {
    arg_command: Command,
    arg_args: Vec<String>,
    flag_list: bool,
    flag_verbose: bool,
}

#[derive(RustcDecodable, Show)]
enum Command {
    Build, Clean, Doc, New, Run, Test, Bench, Update,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}

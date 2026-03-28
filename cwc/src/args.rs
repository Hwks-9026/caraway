use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about = "A repl and interpreter for expressions and closures.", 
    long_about = None, 
    )]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub input: Option<PathBuf>,
    #[arg(short = 'j', long = "jobs", value_name = "Number of Threads", default_value_t = 1)]
    pub threads: usize
}

pub fn parse_args() -> Args {
    Args::parse()
}



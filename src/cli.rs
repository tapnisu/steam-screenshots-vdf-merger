use clap::Parser;

#[derive(Parser, Debug, Default, Clone, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input files to join
    #[clap(short, long, num_args = 1.., required=true)]
    pub input: Vec<String>,
}

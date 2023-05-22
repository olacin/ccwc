use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Print the byte counts
    #[arg(short = 'c', long = "bytes")]
    pub bytes: bool,

    /// Print the newline counts
    #[arg(short = 'l', long = "lines")]
    pub lines: bool,

    /// Print the word counts
    #[arg(short = 'w', long = "words")]
    pub words: bool,

    /// Print the character counts
    #[arg(short = 'm', long = "chars")]
    pub chars: bool,

    pub file: Option<String>,
}

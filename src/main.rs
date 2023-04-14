use akkadia::AkkadiaError;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Get information on dependencies of your projects")]
struct Args {
    #[arg(
        short,
        long,
        help = "Path to project directory or file.",
        long_help = "Specify the path to a dependency file or to a directory containing the file file. Check out the documentation for more information."
    )]
    path: String,

    #[arg(short, long, default_value = "false")]
    graph: bool,
}

fn main() -> Result<(), AkkadiaError> {
    env_logger::try_init()?;

    let args = Args::try_parse()?;

    Ok(())
}

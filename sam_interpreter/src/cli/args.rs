use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct SamArgs {
    #[clap(long, short)]
    file: Option<String>,
}
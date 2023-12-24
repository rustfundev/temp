use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Latitute of the geographic coordinate
    #[arg(short, long)]
    pub latitude: String,

    /// Longitude of the geographic coordinate
    #[arg(short = 'L', long)]
    pub longitude: String,
}

use clap::Parser;

/// A web server for the Alinventory application.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The port to run the web server on
    #[clap(long, default_value_t = 9090)]
    pub port: u16,
}

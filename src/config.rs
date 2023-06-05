#[derive(clap::Parser)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,
}

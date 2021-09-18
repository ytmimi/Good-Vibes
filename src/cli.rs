use rand::{rngs::ThreadRng, Rng};

use std::fs::File;
use std::io::BufReader;
use std::net::Ipv4Addr;
use structopt::StructOpt;

pub struct Vibes(Vec<String>);

impl Vibes {
    pub fn random_vibe(&self, rng: &mut ThreadRng) -> &String {
        let rand_index = rng.gen_range(0..self.0.len());
        &self.0[rand_index]
    }
}

fn try_from_file_path(file_path: &str) -> std::io::Result<Vibes> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let good_vibes: Vec<String> = serde_json::from_reader(reader)?;
    Ok(Vibes(good_vibes))
}

fn try_log_level(level: &str) -> Result<String, String> {
    match level {
        "trace" | "debug" | "info" | "warn" | "error" => Ok(level.to_owned()),
        _ => Err(
            format!(
                "{} is not a valid log level. Select one of 'trace', 'debug', 'info', 'warn', or 'error'.",
                level
            )
        )
    }
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(
        long = "--host",
        default_value = "127.0.0.1",
        help = "Defaults to 127.0.0.1"
    )]
    pub host: Ipv4Addr,
    #[structopt(
        long = "--port",
        default_value = "8080",
        help = "The port the server runs on. By default 8080."
    )]
    pub port: u16,
    #[structopt(
        long = "--vibes",
        parse(try_from_str = try_from_file_path),
        help = "Path a JSON File with a list of positive messages!"
    )]
    pub vibes: Vibes,
    #[structopt(
        long = "--log-level",
        parse(try_from_str = try_log_level),
        help = "Log Level. Select one of 'trace', 'debug', 'info', 'warn', or 'error'."
    )]
    pub log_level: Option<String>,
}

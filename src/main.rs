use rand::prelude::*;
use std::fs::File;
use std::io::{BufReader, Result};
use std::net::Ipv4Addr;
use std::sync::Arc;
use structopt::StructOpt;
use warp::Filter;

struct Vibes(Vec<String>);

impl Vibes {
    fn random_vibe(&self, rng: &mut ThreadRng) -> &String {
        let rand_index = rng.gen_range(0..self.0.len());
        &self.0[rand_index]
    }
}

fn try_from_file_path(file_path: &str) -> Result<Vibes> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let good_vibes: Vec<String> = serde_json::from_reader(reader)?;
    Ok(Vibes(good_vibes))
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        long = "--host",
        default_value = "127.0.0.1",
        help = "Defaults to 127.0.0.1"
    )]
    host: Ipv4Addr,
    #[structopt(
        long = "--port",
        default_value = "8080",
        help = "The port the server runs on. By default 8080."
    )]
    port: u16,
    #[structopt(
        long = "--vibes",
        parse(try_from_str = try_from_file_path),
        help = "Path a JSON File with a list of positive messages!"
    )]
    vibes: Vibes,
}

#[tokio::main]
async fn main() {
    let Cli { host, port, vibes } = Cli::from_args();

    let vibes = Arc::new(vibes);

    let hello: _ = warp::path!("good-vibes").map(move || {
        let mut rng = rand::thread_rng();
        warp::reply::json(&*vibes.random_vibe(&mut rng))
    });

    warp::serve(hello).run((host, port)).await;
}

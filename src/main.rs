mod cli;

use flexi_logger::Logger;
use std::sync::Arc;
use structopt::StructOpt;
use warp::Filter;

use cli::Cli;

#[tokio::main]
async fn main() {
    let Cli {
        host,
        port,
        vibes,
        log_level,
    } = Cli::from_args();

    if let Some(level) = log_level {
        // based on try_log_level we know that we'll only get a valid log level
        Logger::try_with_str(level)
            .unwrap()
            .start()
            .expect("Setting up logger failed");
    }

    let vibes = Arc::new(vibes);

    let hello: _ = warp::path!("good-vibes").map(move || {
        let mut rng = rand::thread_rng();
        warp::reply::json(&*vibes.random_vibe(&mut rng))
    });

    warp::serve(hello).run((host, port)).await;
}

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use structopt::StructOpt;

mod detect;
mod handlers;
mod logos;

const DEFAULT_HTTP_ADDR: &str = "0.0.0.0";
const DEFAULT_HTTP_PORT: &str = "9876";

#[derive(Debug, StructOpt)]
#[structopt(name = "WUW")]
struct Opt {
    #[structopt(default_value = DEFAULT_HTTP_ADDR, short, long)]
    bind_addr: String,

    #[structopt(default_value = DEFAULT_HTTP_PORT, short, long)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let opt = Opt::from_args();

    log::info!(
        "starting HTTP server at http://{:}:{:}",
        opt.bind_addr,
        opt.port,
    );

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/{bs58_pubkey}", web::get().to(handlers::get_pubkey_chain))
            .wrap(Logger::default())
    })
    .bind((opt.bind_addr, opt.port))?
    .run()
    .await
}

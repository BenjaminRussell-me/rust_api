use actix_web::{
    HttpServer,
    HttpResponse,
    get,
    App,
    web::Path,
    Responder,
};
use rhai::Engine;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

mod user;

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    // get the numbers from the url path
    let (num1, num2) = path.into_inner();

    // create an instance of the rhai engine
    let mut engine = Engine::new();

    // register an API that exposes the numbers to Rhai
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    // run the script
    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    // return the result
    HttpResponse::Ok().body(result.to_string())
}

// When Rust executes the Rhai script, Rhai returns the result of the last expression

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let mut listenfd = ListenFd::from_env();

    let mut server =HttpServer::new(|| {
        App::new()
            .configure(user::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
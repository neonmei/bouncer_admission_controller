use bouncer::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().unwrap().await
}

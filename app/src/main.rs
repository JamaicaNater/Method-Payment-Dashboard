use method_assesment::serve;

#[tokio::main]
async fn main() {
    env_logger::init();
    serve().await;
}

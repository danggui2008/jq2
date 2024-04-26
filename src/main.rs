use jq2::Server;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let s = Server::new();
    s.start().await
}
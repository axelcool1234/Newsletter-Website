use newsletter_website::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port"); // Port 0 triggers an OS scan for a random
                                                                                          // available port which will then be bound to
                                                                                          // this TcpListener
    run(listener)?.await
}

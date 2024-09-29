use std::net::TcpListener;
use test_case::test_case;

/// Launches the application for testing purposes
/// and returns its address (i.e. http://localhost:port)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter_website::run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=John%20Doe&email=John_Doe%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 200)
}

#[test_case("name=John%20Doe"            ; "missing the email")]
#[test_case("email=John_Doe%40gmail.com" ; "missing the name")]
#[test_case(""                           ; "missing both name and email")]
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(invalid_body: &'static str) {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 400)
}

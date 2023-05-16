use actix_web::rt::spawn;
use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("failed to get /health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("failed to bind address");

    let _ = spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let addr = spawn_app();

    let config = get_configuration().expect("failed to read configuration file");

    let connect_string = config.database.connection_string();

    let mut connection = PgConnection::connect(&connect_string)
        .await
        .expect("failed to connect to postgres");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to post to /subscriptions");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved record");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to post to /subscriptions");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

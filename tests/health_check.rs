// use newsletter;
use newsletter::config::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::net::TcpListener;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    println!("{}", &port);
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}/api/v1/newsletter", port);
    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    println!("{}", &format!("{}/health", app.address));
    let response = client
        .get(&format!("{}/health", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    //assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
#[warn(unused_variables)]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!(
        "SELECT email, name FROM subscriptions where email = 'ursula_le_guin@gmail.com'",
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");
    sqlx::query!(
        r#"
        DELETE FROM subscriptions where email=$1 and name=$2
        "#,
        &saved.email,
        &saved.name,
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to delete the test email.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

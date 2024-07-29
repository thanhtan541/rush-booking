use argon2::PasswordHasher;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, Version};
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use once_cell::sync::Lazy;
use rush_booking::startup::get_connection_pool;
use rush_booking::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
    utils::ResponseData,
};
use sqlx::PgPool;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub api_client: reqwest::Client,
    pub test_user: TestUser,
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: SafeEmail().fake(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
        // `dbg!` is a macro that prints and returns the value // of an expression for quick and dirty debugging.
        // dbg!(&password_hash);
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash) VALUES ($1, $2, $3)",
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to create test users.");
    }
}

impl TestApp {
    pub async fn get_healthcheck(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/health_check", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_hosts(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/admin/hosts", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_rooms(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/admin/rooms", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    // Singleton Pattern
    Lazy::force(&TRACING);

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        // Wildcard port, the system will find available port
        c.application.port = 0;
        c
    };
    let app = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let port = app.port();
    let address = format!("http://127.0.0.1:{}", port);

    // Run the application
    let _ = tokio::spawn(app.run_until_stopped());
    let test_app = TestApp {
        db_pool: get_connection_pool(&configuration.database),
        address,
        port,
        api_client,
        test_user: TestUser::generate(),
    };
    // Add test user
    test_app.test_user.store(&test_app.db_pool).await;
    test_app
}

pub async fn get_response_data_from_json<T>(res: reqwest::Response) -> ResponseData<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    res.json::<ResponseData<T>>()
        .await
        .expect("Unexpected response format")
}

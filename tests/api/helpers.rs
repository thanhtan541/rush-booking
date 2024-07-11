use rush_booking::{configuration::get_configuration, startup::Application};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn get_healthcheck(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/health_check", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
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
    TestApp {
        address,
        port,
        api_client,
    }
}

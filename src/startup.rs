use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use std::{io::Error, net::TcpListener};

use crate::{
    configuration::Settings,
    routes::{health_check, list_rooms},
};

pub struct ApplicationBaseUrl(pub String);
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).expect(&format!(
            "Failed to bind port {}",
            configuration.application.port
        ));
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, configuration.application.base_url).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }
}

// pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
//     PgPoolOptions::new().connect_lazy_with(configuration.with_db())
// }

async fn run(listener: TcpListener, base_url: String) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(web::scope("/admin").service(list_rooms))
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

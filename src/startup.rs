use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{io::Error, net::TcpListener};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{DatabaseSettings, Settings},
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
        let connection_pool = get_connection_pool(&configuration.database);

        let server = run(
            listener,
            configuration.application.base_url,
            connection_pool,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    base_url: String,
    db_pool: PgPool,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Logger middleware
            // Sent active-web log to log subscriber
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(web::scope("/admin").service(list_rooms))
            .app_data(base_url.clone())
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

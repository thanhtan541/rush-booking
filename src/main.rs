use rush_booking::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let app = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(app.run_until_stopped());

    tokio::select! {
        _ = application_task => {}
    };

    Ok(())
}

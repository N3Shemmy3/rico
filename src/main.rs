use axum::Router;
use axum_folder_router::folder_router;
use log::info;

#[derive(Clone)]
struct AppState;

// Imports route.rs files & generates an ::into_router() fn
#[folder_router("src/api/", AppState)]
struct AppRouter();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create app state
    let app_state = AppState;

    // Use the init fn generated above
    let app_router: Router<AppState> = AppRouter::into_router();

    // Build the router and provide the state
    let app: Router<()> = app_router.with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let local_address = listener.local_addr()?;
    info!("listening on : {local_address}");
    axum::serve(listener, app).await?;
    Ok(())
}

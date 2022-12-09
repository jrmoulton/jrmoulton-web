use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::{io, net::SocketAddr};
use tokio::sync::mpsc::{channel, Receiver};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(
        serve(using_serve_dir(), 80),
        async_watch("content/"),
        async_watch("crates/gen/src"),
        async_watch("crates/tree-painter/src"),
        async_watch("styles/"),
        async_watch("themes/"),
        async_watch("templates/"),
    );
}

fn using_serve_dir() -> Router {
    // `SpaRouter` is just a convenient wrapper around `ServeDir`
    //
    // You can use `ServeDir` directly to further customize your setup
    let serve_dir = get_service(ServeDir::new("build")).handle_error(handle_error);

    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/build", serve_dir.clone())
        .fallback_service(serve_dir)
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) {
    let (mut watcher, mut rx) = async_watcher().unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    tokio::process::Command::new("cargo")
                        .arg("r")
                        .arg("-r")
                        .arg("--bin")
                        .arg("gen")
                        .stderr(std::process::Stdio::null())
                        .spawn()
                        .unwrap()
                        .wait()
                        .await
                        .unwrap();
                    println!("Generated code");
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

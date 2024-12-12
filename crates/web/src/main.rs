#![allow(unused_imports)]
use std::{
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
    routing::{get, get_service},
    BoxError, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
    spawn,
    sync::mpsc::{channel, Receiver},
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Ports {
    http: u16,
    https: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[allow(unused_variables)]
    let ports = Ports { http: 80, https: 443 };

    // #[cfg(not(debug_assertions))]
    // spawn(redirect_http_to_https(ports));

    if cfg!(debug_assertions) {
        tokio::join!(
            serve(using_serve_dir(), ports.https),
            async_watch("content/"),
            async_watch("crates/gen/src"),
            async_watch("crates/tree-painter/src"),
            async_watch("styles/"),
            async_watch("js/"),
            async_watch("themes/"),
            async_watch("templates/"),
        );
    } else {
        tokio::join!(
            serve(using_serve_dir(), ports.https),
            async_watch("content/"),
        );
    }
}

fn using_serve_dir() -> Router {
    // `SpaRouter` is just a convenient wrapper around `ServeDir`
    //
    // You can use `ServeDir` directly to further customize your setup
    let serve_dir = get_service(ServeDir::new("build")).handle_error(handle_error);

    Router::new().nest_service("/", serve_dir.clone())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    if cfg!(not(debug_assertions)) {
        let config = RustlsConfig::from_pem_file(
            PathBuf::from("/etc/letsencrypt/live/jrmoulton.com/fullchain.pem"),
            PathBuf::from("/etc/letsencrypt/live/jrmoulton.com/privkey.pem"),
        )
        .await
        .unwrap();
        axum_server::bind_rustls(addr, config)
            .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
            .await
            .unwrap();
    } else {
        axum::Server::bind(&addr)
            .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
            .await
            .unwrap();
    }
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
                    let res = tokio::process::Command::new("cargo")
                        .arg("r")
                        .arg(RELEASE)
                        .arg("--bin")
                        .arg("gen")
                        .stderr(std::process::Stdio::null())
                        .spawn()
                        .unwrap()
                        .wait()
                        .await
                        .unwrap();
                    if res.success() {
                        println!("Generated code");
                    } else {
                        println!("Failed to generate codd");
                    };
                },
                _ => {},
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

#[cfg(not(debug_assertions))]
const RELEASE: &'static str = "-r";
#[cfg(debug_assertions)]
const RELEASE: &'static str = "-r";

async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, axum::BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(axum::response::Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            },
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    tracing::debug!("http redirect listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}

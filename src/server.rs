use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::header::HeaderValue;
use hyper::server::conn::http1::Builder;
use hyper::{Method, Request, Response, StatusCode, Version};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::database::DatabaseProvider;
use crate::Error;

pub(crate) async fn serve(
    host: IpAddr,
    port: u16,
    db: Arc<RwLock<impl DatabaseProvider + Send + Sync + 'static>>,
) -> Result<(), Error> {
    log::info!("starting the server at: http://{}:{}/", host, port);

    let listener = match TcpListener::bind(SocketAddr::from((host, port))).await {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("an error occurred while creating a new tcp listener: {}", e);
            return Err(e.into());
        }
    };

    loop {
        let db = Arc::clone(&db);

        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    let service = hyper::service::service_fn(move |req: Request<Incoming>| {
                        let db = Arc::clone(&db);

                        async {
                            Ok::<_, Infallible>(match service_fn(req, db).await {
                                Ok(res) => res,
                                Err(_) => internal_server_error(),
                            })
                        }
                    });

                    if let Err(e) = Builder::new()
                        .title_case_headers(true)
                        .keep_alive(true)
                        .serve_connection(TokioIo::new(stream), service)
                        .await
                    {
                        log::error!("an error occurred while serving the connection: {}", e);
                    };
                });
            }
            Err(e) => {
                log::error!("an error occurred while accepting a new connection: {}", e);
            }
        }
    }
}

async fn service_fn(
    req: Request<Incoming>,
    db: Arc<RwLock<impl DatabaseProvider>>,
) -> Result<Response<Full<Bytes>>, Error> {
    if req.version() > Version::HTTP_11 {
        return http_version_not_supported();
    }

    let normalized_path = normalize_path(req.uri().path());
    let segments = normalized_path.split("/").collect::<Vec<&str>>();

    match &segments.as_slice() {
        ["api", s @ ..] => match s {
            ["crypto-sanctions", address] => {
                if !matches!(req.method(), &Method::GET) {
                    return method_not_allowed("GET");
                }

                if !address.chars().all(char::is_alphanumeric) {
                    return bad_request();
                }

                let s = db.read().await.search(address.to_string()).await?;
                let body = format!("{{\"address\": \"{}\", \"sanctioned\": {}}}", address, s);

                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Full::new(Bytes::copy_from_slice(body.as_bytes())))
                    .map_err(|e| e.into())
            }
            _ => not_found(),
        },
        _ => not_found(),
    }
}

fn normalize_path(path: &str) -> String {
    let mut segments = vec![];

    path.split("/").for_each(|s| match s {
        "" | "." => {}
        ".." => {
            segments.pop();
        }
        s => segments.push(s),
    });

    segments.join("/")
}

fn bad_request() -> Result<Response<Full<Bytes>>, Error> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("content-type", "application/json")
        .body(format!("{{\"error\": \"bad request\"}}").into())
        .map_err(|e| e.into())
}

fn not_found() -> Result<Response<Full<Bytes>>, Error> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(format!("{{\"error\": \"not found\"}}").into())
        .map_err(|e| e.into())
}

fn method_not_allowed(allowed: &str) -> Result<Response<Full<Bytes>>, Error> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header("allow", allowed)
        .header("content-type", "application/json")
        .body(format!("{{\"error\": \"method not allowed\"}}").into())
        .map_err(|e| e.into())
}

fn http_version_not_supported() -> Result<Response<Full<Bytes>>, Error> {
    Response::builder()
        .status(StatusCode::HTTP_VERSION_NOT_SUPPORTED)
        .header("content-type", "application/json")
        .body(format!("{{\"error\": \"http version not supported\"}}").into())
        .map_err(|e| e.into())
}

fn internal_server_error() -> Response<Full<Bytes>> {
    let mut res = Response::new(format!("{{\"error\": \"internal server error\"}}").into());
    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    res.headers_mut()
        .insert("content-type", HeaderValue::from_static("application/json"));

    res
}

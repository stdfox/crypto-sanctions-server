use std::sync::Arc;

use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use itertools::Itertools;
use regex::Regex;
use tokio::sync::RwLock;
use tokio::time::Duration;

use crate::database::DatabaseProvider;
use crate::Error;

const DEFAULT_URL: &str = "https://www.treasury.gov/ofac/downloads/sdnlist.txt";
const DEFAULT_TIMEOUT: u64 = 60;

pub(crate) async fn try_update(db: Arc<RwLock<impl DatabaseProvider>>) -> Result<(), Error> {
    let payload = try_fetch().await?;
    let records = try_parse(payload).await?;

    db.write().await.save_records(records).await
}

async fn try_fetch() -> Result<Bytes, Error> {
    let timeout = Duration::from_secs(DEFAULT_TIMEOUT);
    let https = HttpsConnector::new();
    let io = TokioExecutor::new();

    let client = Client::builder(io)
        .pool_idle_timeout(timeout)
        .build::<_, Empty<Bytes>>(https);

    let res = client.get(Uri::from_static(DEFAULT_URL)).await?;
    let res = res.collect().await?;

    Ok(res.to_bytes())
}

async fn try_parse(payload: Bytes) -> Result<Vec<String>, Error> {
    const REGEX: &str =
        r"(Digital[\s]{0,}Currency[\s]{0,}Address)[\s]{0,}-[\s]{0,}([\w]{3,4})[\s]{0,}([\w-]+)";

    let re = match Regex::new(REGEX) {
        Ok(res) => res,
        Err(e) => return Err(e.into()),
    };

    let payload = match String::from_utf8(payload.to_vec()) {
        Ok(res) => res,
        Err(e) => return Err(e.into()),
    };

    let res = re
        .captures_iter(&payload)
        .map(|cap| {
            match cap.get(3) {
                Some(address) => address.as_str(),
                None => "",
            }
            .to_string()
        })
        .filter(|s| !s.is_empty())
        .unique_by(|s| s.to_owned())
        .sorted()
        .collect();

    Ok(res)
}

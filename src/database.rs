use std::future::Future;

use crate::Error;

const DEFAULT_CAPACITY: usize = 1000;

pub(crate) trait DatabaseProvider {
    fn save_records(&mut self, records: Vec<String>) -> impl Future<Output = Result<(), Error>>;
    fn records(&self) -> impl Future<Output = Result<Vec<String>, Error>>;
    fn search(&self, address: String) -> impl Future<Output = Result<bool, Error>> + Send + Sync;
}

pub(crate) struct InMemoryDatabase {
    inner: Vec<String>,
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self {
            inner: Vec::with_capacity(DEFAULT_CAPACITY),
        }
    }
}

impl DatabaseProvider for InMemoryDatabase {
    async fn save_records(&mut self, records: Vec<String>) -> Result<(), Error> {
        log::trace!("saving {} sanction records", records.len());

        self.inner.clone_from(&records);
        self.inner.sort();

        Ok(())
    }

    async fn records(&self) -> Result<Vec<String>, Error> {
        Ok(self.inner.clone())
    }

    async fn search(&self, address: String) -> Result<bool, Error> {
        Ok(self.inner.binary_search(&address).is_ok())
    }
}

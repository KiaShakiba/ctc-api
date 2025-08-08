use std::{
	env,
	sync::MutexGuard,
};

use diesel_async::{
	AsyncPgConnection,
	pooled_connection::{
		AsyncDieselConnectionManager,
		deadpool::{Pool, Object},
	},
};

use paper_client::{PaperClient, PaperPool};
use crate::error::Error;

type DbConnection = Object<AsyncPgConnection>;

#[derive(Clone)]
pub struct AppState {
	db: Pool<AsyncPgConnection>,
	cache: PaperPool,
}

impl AppState {
	pub async fn init() -> anyhow::Result<Self> {
		let db_addr = env::var("DATABASE_URL").map_err(|_| Error::default())?;
		let db_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_addr);

		let db = Pool::builder(db_manager)
			.build()
			.map_err(|_| Error::default())?;

		let cache_addr = env::var("CACHE_URL").map_err(|_| Error::default())?;
		let cache = PaperPool::new(cache_addr, 4)?;

		let state = AppState {
			db,
			cache,
		};

		Ok(state)
	}

	pub async fn db(&self) -> Result<DbConnection, Error> {
		self.db
			.get().await
			.map_err(|_| Error::default())
	}

	pub fn cache(&self) -> MutexGuard<'_, PaperClient> {
		self.cache.client()
	}
}

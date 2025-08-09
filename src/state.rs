use std::{
	env,
	sync::MutexGuard,
};

use serde::{
	Serialize,
	de::DeserializeOwned,
};

use diesel_async::{
	AsyncPgConnection,
	pooled_connection::{
		AsyncDieselConnectionManager,
		deadpool::{Pool, Object},
	},
};

use paper_client::{
	PaperClient,
	PaperPool,
	error::{PaperClientError, PaperCacheError},
};

use postcard::{to_allocvec, from_bytes};
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

pub trait Cachable {
	type Id;

	fn cache_key(id: Self::Id) -> String;

	fn to_cached(
		&self,
		mut cache: MutexGuard<'_, PaperClient>,
		id: Self::Id,
	) -> Result<(), Error>
	where
		Self: Serialize,
	{
		let bytes = to_allocvec(self)?;
		cache.set(Self::cache_key(id), bytes, Some(3_600))?;

		Ok(())
	}

	fn from_cached(
		mut cache: MutexGuard<'_, PaperClient>,
		id: Self::Id,
	) -> Result<Option<Self>, Error>
	where
		Self: DeserializeOwned,
	{
		let value = match cache.get(Self::cache_key(id)) {
			Ok(value) => value,
			Err(PaperClientError::CacheError(PaperCacheError::KeyNotFound)) => return Ok(None),
			Err(err) => return Err(err.into()),
		};

		let bytes: Box<[u8]> = value.into();
		Ok(Some(from_bytes::<Self>(&bytes)?))
	}

	fn purge_cache(
		mut cache: MutexGuard<'_, PaperClient>,
		id: Self::Id,
	) -> Result<(), Error> {
		cache.del(Self::cache_key(id))?;
		Ok(())
	}
}

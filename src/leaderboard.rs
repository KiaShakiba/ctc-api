use std::{
	time::Duration,
	collections::hash_map::{HashMap, Entry},
};

use serde::{Serialize, Serializer};

#[derive(Default)]
pub struct Leaderboard {
	user_map: HashMap<i32, LeaderboardResult>,
}

#[derive(Default, Serialize)]
pub struct LeaderboardResult {
	pub username: String,

	#[serde(serialize_with = "serialize_duration")]
	pub duration: Duration,
}

impl Leaderboard {
	pub fn is_faster_result(&self, user_id: i32, duration: Duration) -> bool {
		self.user_map
			.get(&user_id)
			.is_none_or(|result| result.duration > duration)
	}

	pub fn insert(&mut self, user_id: i32, result: LeaderboardResult) {
		match self.user_map.entry(user_id) {
			Entry::Occupied(mut entry) if result.duration < entry.get().duration => {
				entry.insert(result);
			},

			Entry::Vacant(entry) => {
				entry.insert(result);
			},

			_ => {},
		}
	}

	pub fn into_results(self) -> Vec<LeaderboardResult> {
		let mut results = self.user_map.into_values().collect::<Vec<_>>();
		results.sort_by_key(|result| result.duration);
		results
	}
}

fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer
{
	let value = format!("{duration:?}");
	serializer.serialize_str(&value)
}

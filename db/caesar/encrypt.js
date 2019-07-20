'use strict';

let Query = require('../query');

const saveMessage = async (
	username,
	key,
	message
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from caesar
				where username=$1 and
					cipher is null and
					type='encrypt'
		)
		insert into caesar
			(username, key, message, type)
				values
			($1, $2, $3, 'encrypt')
	`;

	query.values = [
		username,
		key,
		message
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getUnencrypted = async (username) => {
	let query = new Query();

	query.command = `
		select key, message
			from caesar
			where username=$1 and
				cipher is null and
				type='encrypt'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const saveEncrypted = async (
	username,
	key,
	message,
	cipher
) => {
	let query = new Query();

	query.command = `
		update caesar
			set
				datetime_submitted=now(),
				cipher=$4
			where username=$1 and
				key=$2 and
				message=$3 and
				type='encrypt'
	`;

	query.values = [username, key, message, cipher];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getTime = async (
	username,
	key,
	message,
	cipher
) => {
	let query = new Query();

	query.command = `
		select extract(
				epoch from (datetime_submitted - datetime_created)
			) as time
			from caesar
			where username=$1 and
				key=$2 and
				message=$3 and
				cipher=$4 and
				type='encrypt'
	`;

	query.values = [username, key, message, cipher];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0].time;
};

const getResults = async () => {
	let query = new Query();

	query.command = `
		select username, min(extract(
				epoch from (datetime_submitted - datetime_created)
			)) as time
			from caesar
			where cipher is not null and type='encrypt'
			group by username
			order by time asc
	`;

	let result = await query.execute();

	result.rows = result.rows.map((row) => {
		row.time = Math.round(row.time * 10000) / 10000;
		return row;
	});

	return result.rows;
};

module.exports.saveMessage = saveMessage;
module.exports.getUnencrypted = getUnencrypted;
module.exports.saveEncrypted = saveEncrypted;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

'use strict';

let Query = require('../query');

const saveMessageCipher = async (
	username,
	message,
	cipher
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from caesar
				where username=$1 and
					key is null and
					type='attack'
		)
		insert into caesar
			(username, message, cipher, type)
				values
			($1, $2, $3, 'attack')
	`;

	query.values = [
		username,
		message,
		cipher
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getMessageCipher = async (username) => {
	let query = new Query();

	query.command = `
		select message, cipher
			from caesar
			where username=$1 and
				key is null and
				type='attack'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const saveKey = async (
	username,
	message,
	cipher,
	key
) => {
	let query = new Query();

	query.command = `
		update caesar
			set
				datetime_submitted=now(),
				key=$4
			where username=$1 and
				message=$2 and
				cipher=$3 and
				type='attack'
	`;

	query.values = [username, message, cipher, key];

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
				type='attack'
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
			where key is not null and type='attack'
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

module.exports.saveMessageCipher = saveMessageCipher;
module.exports.getMessageCipher = getMessageCipher;
module.exports.saveKey = saveKey;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

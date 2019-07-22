'use strict';

let Query = require('../query');

const saveCipher = async (
	username,
	key,
	cipher
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from caesar
				where username=$1 and
					message is null and
					type='decrypt'
		)
		insert into caesar
			(username, key, cipher, type)
				values
			($1, $2, $3, 'decrypt')
	`;

	query.values = [
		username,
		key,
		cipher
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getEncrypted = async (username) => {
	let query = new Query();

	query.command = `
		select key, cipher
			from caesar
			where username=$1 and
				message is null and
				type='decrypt'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const saveDecrypted = async (
	username,
	key,
	cipher,
	message
) => {
	let query = new Query();

	query.command = `
		update caesar
			set
				datetime_submitted=now(),
				message=$4
			where username=$1 and
				key=$2 and
				cipher=$3 and
				type='decrypt'

	`;

	query.values = [username, key, cipher, message];

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
				type='decrypt'
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
			where message is not null and type='decrypt'
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

module.exports.saveCipher = saveCipher;
module.exports.getEncrypted = getEncrypted;
module.exports.saveDecrypted = saveDecrypted;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

'use strict';

let Query = require('../query');

const saveServerSk = async (
	username,
	g,
	n,
	sk
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from diffie_hellman
				where username=$1 and
					k is null and
					type='exchange'
		)
		insert into diffie_hellman
			(username, g, n, sk_server, type)
				values
			($1, $2, $3, $4, 'exchange')
	`;

	query.values = [
		username,
		g,
		n,
		sk
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getServerSk = async (username) => {
	let query = new Query();

	query.command = `
		select g, n, sk_server
			from diffie_hellman
			where username=$1 and
				k is null and
				type='exchange'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	result.rows[0].skServer = result.rows[0].sk_server;

	delete result.rows[0].sk_server;

	return result.rows[0];
};

const saveKey = async (
	username,
	g,
	n,
	skServer,
	pkUser,
	k
) => {
	let query = new Query();

	query.command = `
		update diffie_hellman
			set
				datetime_submitted=now(),
				pk_user=$5,
				k=$6
			where username=$1 and
				g=$2 and
				n=$3 and
				sk_server=$4 and
				type='exchange'
	`;

	query.values = [
		username,
		g,
		n,
		skServer,
		pkUser,
		k
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getTime = async (
	username,
	g,
	n,
	skServer,
	pkUser,
	k
) => {
	let query = new Query();

	query.command = `
		select extract(
				epoch from (datetime_submitted - datetime_created)
			) as time
			from diffie_hellman
			where username=$1 and
				g=$2 and
				n=$3 and
				sk_server=$4 and
				pk_user=$5 and
				k=$6 and
				type='exchange'
	`;

	query.values = [
		username,
		g,
		n,
		skServer,
		pkUser,
		k
	];

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
			from diffie_hellman
			where k is not null and type='exchange'
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

module.exports.saveServerSk = saveServerSk;
module.exports.getServerSk = getServerSk;
module.exports.saveKey = saveKey;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

'use strict';

let Query = require('../query');

const saveCipher = async (
	username,
	p,
	q,
	e,
	d,
	c
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from rsa
				where username=$1 and
					m is null and
					type='decrypt'
		)
		insert into rsa
			(username, p, q, e, d, c, type)
				values
			($1, $2, $3, $4, $5, $6, 'decrypt')
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		c
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getEncrypted = async (username) => {
	let query = new Query();

	query.command = `
		select p, q, e, d, c
			from rsa
			where username=$1 and
				m is null and
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
	p,
	q,
	e,
	d,
	c,
	m
) => {
	let query = new Query();

	query.command = `
		update rsa
			set
				datetime_submitted=now(),
				m=$7
			where username=$1 and
				p=$2 and
				q=$3 and
				e=$4 and
				d=$5 and
				c=$6 and
				type='decrypt'
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		c,
		m
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getTime = async (
	username,
	p,
	q,
	e,
	d,
	c,
	m
) => {
	let query = new Query();

	query.command = `
		select extract(
				epoch from (datetime_submitted - datetime_created)
			) as time
			from rsa
			where username=$1 and
				p=$2 and
				q=$3 and
				e=$4 and
				d=$5 and
				c=$6 and
				m=$7 and
				type='decrypt'
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		c,
		m
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
			from rsa
			where m is not null and type='decrypt'
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

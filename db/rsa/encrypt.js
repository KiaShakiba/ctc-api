'use strict';

let Query = require('../query');

const saveMessage = async (
	username,
	p,
	q,
	e,
	d,
	m
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from rsa
				where username=$1 and
					c is null and
					type='encrypt'
		)
		insert into rsa
			(username, p, q, e, d, m, type)
				values
			($1, $2, $3, $4, $5, $6, 'encrypt')
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		m
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getUnencrypted = async (username) => {
	let query = new Query();

	query.command = `
		select p, q, e, d, m
			from rsa
			where username=$1 and
				c is null and
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
	p,
	q,
	e,
	d,
	m,
	c
) => {
	let query = new Query();

	query.command = `
		update rsa
			set
				datetime_submitted=now(),
				c=$7
			where username=$1 and
				p=$2 and
				q=$3 and
				e=$4 and
				d=$5 and
				m=$6 and
				type='encrypt'
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		m,
		c
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
	m,
	c
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
				m=$6 and
				c=$7 and
				type='encrypt'
	`;

	query.values = [
		username,
		p,
		q,
		e,
		d,
		m,
		c
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
			where c is not null and type='encrypt'
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

'use strict';

let Query = require('../query');

const saveMessage = async (
	username,
	p,
	q,
	g,
	hString,
	sk,
	pk,
	k,
	r,
	s,
	m
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from dss
				where username=$1 and
					w is null and
					type='verify'
		)
		insert into dss
			(username, p, q, g, h, sk, pk, k, r, s, m, type)
				values
			($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'verify')
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		sk,
		pk,
		k,
		r,
		s,
		m
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getUnverified = async (username) => {
	let query = new Query();

	query.command = `
		select p, q, g, h, sk, pk, k, r, s, m
			from dss
			where username=$1 and
				w is null and
				type='verify'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const saveVerified = async (
	username,
	p,
	q,
	g,
	hString,
	sk,
	pk,
	k,
	r,
	s,
	m,
	u,
	v,
	w
) => {
	let query = new Query();

	query.command = `
		update dss
			set
				datetime_submitted=now(),
				u=$12,
				v=$13,
				w=$14
			where username=$1 and
				p=$2 and
				q=$3 and
				g=$4 and
				h=$5 and
				sk=$6 and
				pk=$7 and
				k=$8 and
				r=$9 and
				s=$10 and
				m=$11 and
				type='verify'
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		sk,
		pk,
		k,
		r,
		s,
		m,
		u,
		v,
		w
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getTime = async (
	username,
	p,
	q,
	g,
	hString,
	sk,
	pk,
	k,
	r,
	s,
	m,
	u,
	v,
	w
) => {
	let query = new Query();

	query.command = `
		select extract(
				epoch from (datetime_submitted - datetime_created)
			) as time
			from dss
			where username=$1 and
				p=$2 and
				q=$3 and
				g=$4 and
				h=$5 and
				sk=$6 and
				pk=$7 and
				k=$8 and
				r=$9 and
				s=$10 and
				m=$11 and
				u=$12 and
				v=$13 and
				w=$14 and
				type='verify'
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		sk,
		pk,
		k,
		r,
		s,
		m,
		u,
		v,
		w
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
			from dss
			where w is not null and type='verify'
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
module.exports.getUnverified = getUnverified;
module.exports.saveVerified = saveVerified;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

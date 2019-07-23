'use strict';

let Query = require('../query');

const saveMessage = async (
	username,
	p,
	q,
	g,
	hString,
	m
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from dss
				where username=$1 and
					r is null and
					s is null and
					type='sign'
		)
		insert into dss
			(username, p, q, g, h, m, type)
				values
			($1, $2, $3, $4, $5, $6, 'sign')
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		m
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getUnsigned = async (username) => {
	let query = new Query();

	query.command = `
		select p, q, g, h, m
			from dss
			where username=$1 and
				r is null and
				s is null and
				type='sign'
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const saveSigned = async (
	username,
	p,
	q,
	g,
	hString,
	m,
	pk,
	r,
	s,
	u,
	v,
	w
) => {
	let query = new Query();

	query.command = `
		update dss
			set
				datetime_submitted=now(),
				pk=$7,
				r=$8,
				s=$9,
				u=$10,
				v=$11,
				w=$12
			where username=$1 and
				p=$2 and
				q=$3 and
				g=$4 and
				h=$5 and
				m=$6 and
				type='sign'
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		m,
		pk,
		r,
		s,
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
	m,
	pk,
	r,
	s,
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
				m=$6 and
				pk=$7 and
				r=$8 and
				s=$9 and
				u=$10 and
				v=$11 and
				w=$12 and
				type='sign'
	`;

	query.values = [
		username,
		p,
		q,
		g,
		hString,
		m,
		pk,
		r,
		s,
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
			where r is not null and
				s is not null and
				type='sign'
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
module.exports.getUnsigned = getUnsigned;
module.exports.saveSigned = saveSigned;
module.exports.getTime = getTime;
module.exports.getResults = getResults;

'use strict';

let Query = require('../query');

const save = async (
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
		insert into rsa
			(username, p, q, e, d, m, c, type)
				values
			($1, $2, $3, $4, $5, $6, $7, 'verify')
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

const getResults = async () => {
	let query = new Query();

	query.command = `
		select username, avg(p * q) as average_n,
				avg(e) as average_e, avg(d) as average_d,
				avg(m) as average_m, avg(c) as average_c
			from rsa
			where type='verify'
			group by username
	`;

	let result = await query.execute();

	return result.rows.map((row) => {
		row.average = {
			n: row.average_n,
			e: row.average_e,
			d: row.average_d,
			m: row.average_m,
			c: row.average_c
		};

		delete row.average_n;
		delete row.average_e;
		delete row.average_d;
		delete row.average_m;
		delete row.average_c;

		return row;
	});
};

module.exports.save = save;
module.exports.getResults = getResults;

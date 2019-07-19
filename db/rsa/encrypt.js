'use strict';

let Query = require('../query');

const saveUnencrypted = async (
	username,
	p,
	q,
	e,
	m
) => {
	let query = new Query();

	query.command = `
		with del as (
			delete from rsa
				where username=$1 and
					m is null and
					type='encrypt'
		)
		insert into rsa
			(username, p, q, e, m, type)
				values
			($1, $2, $3, $4, $5, 'encrypt')
	`;

	query.values = [
		username,
		p,
		q,
		e,
		m
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

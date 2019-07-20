'use strict';

const ClientError = require('../client-error');
const rsaVerifyDb = require('../../db/rsa/verify');

require('../number');

const encrypt = (m, e, n) => {
	return m.powerMod(e, n);
};

const submit = async (
	username,
	p,
	q,
	e,
	d,
	m,
	c
) => {
	if (isNaN(p) || p < 0 || !p.isPrime()) {
		throw new ClientError(400, 'Invalid p.');
	}

	if (isNaN(q) || q < 0 || !q.isPrime()) {
		throw new ClientError(400, 'Invalid q.');
	}

	let n = p * q,
		totient = (p - 1) * (q - 1);

	if (isNaN(e) || e <= 1 || e >= totient ||
		e.gcd(totient) !== 1 || e >= n) {

		throw new ClientError(400, 'Invalid e.');
	}

	if (isNaN(d) || d <= 0 || d !== e.inverseMod(totient) ||
		d >= n) {

		throw new ClientError(400, 'Invalid d.');
	}

	if (isNaN(m) || m < 1 || m >= n) {
		throw new ClientError(400, 'Invalid message.');
	}

	if (isNaN(c) || c !== m.powerMod(e, n)) {
		throw new ClientError(400, 'Invalid cipher.');
	}

	let saved = await rsaVerifyDb.save(
		username,
		p,
		q,
		e,
		d,
		m,
		c
	);

	if (!saved) {
		throw new ClientError();
	}

	return 'Correct!';
};

const getResults = async () => {
	return await rsaVerifyDb.getResults();
};

module.exports.submit = submit;
module.exports.getResults = getResults;

'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const rsaEncryptDb = require('../../db/rsa/encrypt');

require('../number');

const encrypt = (m, e, n) => {
	return m.powerMod(e, n);
};

const get = async (username) => {
	let bits = 12,
		p = await secure.randomPrime(bits),
		q = await secure.randomPrime(bits);

	while (p === q) {
		q = await secure.randomPrime(bits);
	}

	let n = p * q,
		totient = (p - 1) * (q - 1),
		e = 0,
		d = 0,
		m = await secure.randomNumberInRange(
			Math.floor(n / 2),
			n - 1
		);

	do {
		e = await secure.randomNumberInRange(1, totient - 1);
		d = e.inverseMod(totient);
	} while (e.gcd(totient) !== 1 || d === null || e === d);

	let saved = await rsaEncryptDb.saveMessage(
		username,
		p,
		q,
		e,
		d,
		m
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		pk: {
			n: n,
			e: e,
		},
		message: m
	};
};

const submit = async (
	username,
	cipher
) => {
	if (isNaN(cipher) || cipher < 0) {
		throw new ClientError(400, 'Invalid cipher.');
	}

	let unencrypted = await rsaEncryptDb.getUnencrypted(username);

	if (!unencrypted) {
		throw new ClientError(400, 'User has not gotten a pk/message pair.');
	}

	let n = unencrypted.p * unencrypted.q;

	if (cipher !== encrypt(unencrypted.m, unencrypted.e, n)) {
		throw new ClientError(400, 'Incorrect cipher.');
	}

	let saved = await rsaEncryptDb.saveEncrypted(
		username,
		unencrypted.p,
		unencrypted.q,
		unencrypted.e,
		unencrypted.d,
		unencrypted.m,
		cipher
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await rsaEncryptDb.getTime(
		username,
		unencrypted.p,
		unencrypted.q,
		unencrypted.e,
		unencrypted.d,
		unencrypted.m,
		cipher
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await rsaEncryptDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

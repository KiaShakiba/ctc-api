'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const rsaDecryptDb = require('../../db/rsa/decrypt');

require('../number');

const encrypt = (m, e, n) => {
	return m.powerMod(e, n);
};

const decrypt = (c, d, n) => {
	return c.powerMod(d, n);
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

	let c = encrypt(m, e, n);

	let saved = await rsaDecryptDb.saveCipher(
		username,
		p,
		q,
		e,
		d,
		c
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		sk: {
			n: n,
			d: d,
		},
		cipher: c
	};
};

const submit = async (
	username,
	message
) => {
	if (isNaN(message) || message < 0) {
		throw new ClientError(400, 'Invalid cipher.');
	}

	let encrypted = await rsaDecryptDb.getEncrypted(username);

	if (!encrypted) {
		throw new ClientError(400, 'User has not gotten a sk/cipher pair.');
	}

	let n = encrypted.p * encrypted.q;

	if (message !== decrypt(encrypted.c, encrypted.d, n)) {
		throw new ClientError(400, 'Incorrect cipher.');
	}

	let saved = await rsaDecryptDb.saveDecrypted(
		username,
		encrypted.p,
		encrypted.q,
		encrypted.e,
		encrypted.d,
		encrypted.c,
		message
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await rsaDecryptDb.getTime(
		username,
		encrypted.p,
		encrypted.q,
		encrypted.e,
		encrypted.d,
		encrypted.c,
		message
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await rsaDecryptDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

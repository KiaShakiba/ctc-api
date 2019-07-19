'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const caesarEncryptDb = require('../../db/caesar/encrypt');

require('../mod');

const gcd = (a, b) => {
	if (b === 0) {
		return a;
	}

	return gcd(b, a % b);
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
		m = await secure.randomNumberInRange(
			Math.floor(n / 2),
			n - 1
		);

	do {
		e = await secure.randomNumberInRange(1, totient - 1);
	} while (gcd(e, totient) !== 1);

	return {
		pk: {
			n: n,
			e: e
		},
		message: m
	};
};

module.exports.get = get;

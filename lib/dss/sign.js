'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const dssSignDb = require('../../db/dss/sign');

require('../number');

const HASH_FUNCTIONS = {
	'x mod <q>': (x, q) => x.mod(q),
	'2x mod <q>': (x, q) => (2 * x).mod(q),
	'3x mod <q>': (x, q) => (3 * x).mod(q)
};

const getRandomHashFunction = async (q) => {
	let keys = Object.keys(HASH_FUNCTIONS),
		index = await secure.randomNumberInRange(0, keys.length - 1);

	let hashFunctionString = keys[index].replace(/\<q\>/g, q);

	return {
		string: keys[index],
		formattedString: hashFunctionString,
		func: HASH_FUNCTIONS[keys[index]]
	};
};

const get = async (username) => {
	let bits = 12,
		p = await secure.randomPrime(bits),
		primeFactors = (p - 1).primeFactors(),
		q = 0,
		g = 0;

	do {
		while (!primeFactors.length) {
			p = await secure.randomPrime(bits);
			primeFactors = (p - 1).primeFactors();
		}

		q = primeFactors[primeFactors.length - 1];
		g = 2;

		while (g.order(p) !== q && g <= p) {
			g++
		}
	} while (g.order(p) !== q);

	let h = await getRandomHashFunction(q),
		m = await secure.randomNumberInRange(
			Math.floor(p / 2),
			p - 1
		),
		digest = h.func(m, q);

	while (!digest) {
		m = await secure.randomNumberInRange(
			Math.floor(p / 2),
			p - 1
		);

		digest = h.func(m, q);
	}

	let saved = await dssSignDb.saveMessage(
		username,
		p,
		q,
		g,
		h.string,
		m
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		p: p,
		q: q,
		g: g,
		h: h.formattedString,
		message: m
	};
};

const submit = async (
	username,
	pk,
	r,
	s
) => {
	if (isNaN(pk) || pk <= 0) {
		throw new ClientError(400, 'Invalid public key.');
	}

	if (isNaN(r) || r <= 0) {
		throw new ClientError(400, 'Invalid r.');
	}

	if (isNaN(s) || s <= 0) {
		throw new ClientError(400, 'Invalid s.');
	}

	let unsigned = await dssSignDb.getUnsigned(username);

	if (!unsigned) {
		throw new ClientError(400, 'User has not gotten a message.');
	}

	if (pk >= unsigned.p) {
		throw new ClientError(400, 'Invalid public key.');
	}

	if (r >= unsigned.q) {
		throw new ClientError(400, 'Invalid r.');
	}

	if (s >= unsigned.q) {
		throw new ClientError(400, 'Invalid s.');
	}

	let hash = HASH_FUNCTIONS[unsigned.h],
		digest = hash(unsigned.m, unsigned.q),
		u = (digest * s.inverseMod(unsigned.q)).mod(unsigned.q),
		v = ((-r).mod(unsigned.q) * s.inverseMod(unsigned.q)).mod(unsigned.q),
		w = (unsigned.g.powerMod(u, unsigned.p) * pk.powerMod(v, unsigned.p)).mod(unsigned.p).mod(unsigned.q);

	if (w !== r) {
		throw new ClientError(400, 'Incorrect signature.');
	}

	let saved = await dssSignDb.saveSigned(
		username,
		unsigned.p,
		unsigned.q,
		unsigned.g,
		unsigned.h,
		unsigned.m,
		pk,
		r,
		s,
		u,
		v,
		w
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await dssSignDb.getTime(
		username,
		unsigned.p,
		unsigned.q,
		unsigned.g,
		unsigned.h,
		unsigned.m,
		pk,
		r,
		s,
		u,
		v,
		w
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await dssSignDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

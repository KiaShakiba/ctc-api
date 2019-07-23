'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const dssVerifyDb = require('../../db/dss/verify');

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
		k = await secure.randomNumberInRange(1, q - 1),
		r = g.powerMod(k, p).mod(q);

	while (!r) {
		k = await secure.randomNumberInRange(1, q - 1);
		r = g.powerMod(k, p).mod(q);
	}

	let sk = await secure.randomNumberInRange(1, q - 1),
		pk = g.powerMod(sk, p);

	let m = await secure.randomNumberInRange(
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

	let s = ((digest - sk * r) * k.inverseMod(q)).mod(q);

	let saved = await dssVerifyDb.saveMessage(
		username,
		p,
		q,
		g,
		h.string,
		sk,
		pk,
		k,
		r,
		s,
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
		pk: pk,
		message: m,
		sig: {
			r: r,
			s: s
		}
	};
};

const submit = async (
	username,
	u,
	v,
	w
) => {
	if (isNaN(u) || u < 0) {
		throw new ClientError(400, 'Invalid u.');
	}

	if (isNaN(v) || v < 0) {
		throw new ClientError(400, 'Invalid v.');
	}

	if (isNaN(w) || w < 0) {
		throw new ClientError(400, 'Invalid w.');
	}

	let unverified = await dssVerifyDb.getUnverified(username);

	if (!unverified) {
		throw new ClientError(400, 'User has not gotten a signature/message pair.');
	}

	let hash = HASH_FUNCTIONS[unverified.h],
		digest = hash(unverified.m, unverified.q);

	if (u !== (digest * unverified.s.inverseMod(unverified.q)).mod(unverified.q)) {
		throw new ClientError(400, 'Incorrect u.');
	}

	if (v !== ((-unverified.r).mod(unverified.q) * unverified.s.inverseMod(unverified.q)).mod(unverified.q)) {
		throw new ClientError(400, 'Incorrect v.');
	}

	if (w !== (unverified.g.powerMod(u, unverified.p) * unverified.pk.powerMod(v, unverified.p)).mod(unverified.p).mod(unverified.q)) {
		throw new ClientError(400, 'Incorrect w.');
	}

	let saved = await dssVerifyDb.saveVerified(
		username,
		unverified.p,
		unverified.q,
		unverified.g,
		unverified.h,
		unverified.sk,
		unverified.pk,
		unverified.k,
		unverified.r,
		unverified.s,
		unverified.m,
		u,
		v,
		w
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await dssVerifyDb.getTime(
		username,
		unverified.p,
		unverified.q,
		unverified.g,
		unverified.h,
		unverified.sk,
		unverified.pk,
		unverified.k,
		unverified.r,
		unverified.s,
		unverified.m,
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
	return await dssVerifyDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const diffieHellmanExchangeDb = require('../../db/diffie-hellman/exchange');

require('../number');

const get = async (username) => {
	let bits = 10,
		n = await secure.randomPrime(bits);

	let g = n.primitive(),
		sk = await secure.randomNumberInRange(
			Math.floor(n / 2),
			n - 1
		),
		pk = g.powerMod(sk, n);

	let saved = diffieHellmanExchangeDb.saveServerSk(
		username,
		g,
		n,
		sk
	);

	return {
		g: g,
		n: n,
		pk: pk
	};
};

const submit = async (
	username,
	pkUser,
	k
) => {
	if (isNaN(pkUser) || pkUser < 1) {
		throw new ClientError(400, 'Invalid pk.');
	}

	if (isNaN(k) || k < 1) {
		throw new ClientError(400, 'Invalid k.');
	}

	let getSaved = await diffieHellmanExchangeDb.getServerSk(username);

	if (!getSaved) {
		throw new ClientError(400, 'User has not gotten a public key.');
	}

	let g = getSaved.g,
		n = getSaved.n,
		skServer = getSaved.skServer;

	if (pkUser.powerMod(skServer, n) !== k) {
		throw new ClientError(400, 'Incorrect pk or k.');
	}

	let saved = await diffieHellmanExchangeDb.saveKey(
		username,
		g,
		n,
		skServer,
		pkUser,
		k
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await diffieHellmanExchangeDb.getTime(
		username,
		g,
		n,
		skServer,
		pkUser,
		k
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await diffieHellmanExchangeDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

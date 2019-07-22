'use strict';

const ClientError = require('./client-error');

require('./number');

const getCoprimes = (number) => {
	if (isNaN(number) || number < 2) {
		throw new ClientError(400, 'Invalid number.');
	}

	return number.coprimes();
};

const getPowerMod = (number, e, m) => {
	if (isNaN(number) || number < 2) {
		throw new ClientError(400, 'Invalid number.');
	}

	if (isNaN(e) || e < 0) {
		throw new ClientError(400, 'Invalid exponent.');
	}

	if (isNaN(m) || m < 1) {
		throw new ClientError(400, 'Invalid modulus.');
	}

	return number.powerMod(e, m);
};

const getInverseMod = (number, m) => {
	if (isNaN(number) || number < 2) {
		throw new ClientError(400, 'Invalid number.');
	}

	if (isNaN(m) || m < 1) {
		throw new ClientError(400, 'Invalid modulus.');
	}

	let result = number.inverseMod(m);

	if (result === null) {
		throw new ClientError(400, 'Invalid number.');
	}

	return result;
};

module.exports.getCoprimes = getCoprimes;
module.exports.getPowerMod = getPowerMod;
module.exports.getInverseMod = getInverseMod;

'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const caesarEncryptDb = require('../../db/caesar/encrypt');

require('../number');

const SIZE = 6;
const LETTERS = 'abcdefghijklmnopqrstuvwxyz'.split('').map(l => l.toUpperCase());

const encrypt = (message, key) => {
	return message
		.split('')
		.map((letter) => {
			let index = (LETTERS.indexOf(letter) + key).mod(LETTERS.length);
			return LETTERS[index];
		})
		.join('');
};

const get = async (username) => {
	let key = await secure.randomNumberInRange(8, 18),
		message = '';

	for (let i=0; i<SIZE; i++) {
		message += LETTERS[await secure.randomNumberInRange(0, LETTERS.length - 1)];
	}

	let saved = await caesarEncryptDb.saveMessage(
		username,
		key,
		message
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		key: key,
		message: message
	};
};

const submit = async (
	username,
	cipher
) => {
	if (!cipher ||
		!/^[A-Z]+$/.test(cipher)) {

		throw new ClientError(400, 'Invalid cipher.');
	}

	let unencrypted = await caesarEncryptDb.getUnencrypted(username);

	if (!unencrypted) {
		throw new ClientError(400, 'User has not gotten a key/message pair.');
	}

	if (cipher !== encrypt(unencrypted.message, unencrypted.key)) {
		throw new ClientError(400, 'Incorrect cipher.');
	}

	let saved = caesarEncryptDb.saveEncrypted(
		username,
		unencrypted.key,
		unencrypted.message,
		cipher
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await caesarEncryptDb.getTime(
		username,
		unencrypted.key,
		unencrypted.message,
		cipher
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await caesarEncryptDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

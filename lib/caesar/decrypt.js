'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const caesarDecryptDb = require('../../db/caesar/decrypt');

require('../mod');

const SIZE = 6;
const LETTERS = 'abcdefghijklmnopqrstuvwxyz'.split('').map(l => l.toUpperCase());

const decrypt = (cipher, key) => {
	return cipher
		.split('')
		.map((letter) => {
			let index = (LETTERS.indexOf(letter) - key).mod(LETTERS.length);
			return LETTERS[index];
		})
		.join('');
};

const get = async (username) => {
	let key = await secure.randomNumberInRange(8, 18),
		cipher = '';

	for (let i=0; i<SIZE; i++) {
		cipher += LETTERS[await secure.randomNumberInRange(0, LETTERS.length - 1)];
	}

	let saved = await caesarDecryptDb.saveCipher(
		username,
		key,
		cipher
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		key: key,
		cipher: cipher
	};
};

const submit = async (
	username,
	message
) => {
	if (!message ||
		!/^[A-Z]+$/.test(message)) {

		throw new ClientError(400, 'Invalid message.');
	}

	let encrypted = await caesarDecryptDb.getEncrypted(username);

	if (!encrypted) {
		throw new ClientError(400, 'User has not gotten a key/cipher pair.');
	}

	if (message !== decrypt(encrypted.cipher, encrypted.key)) {
		throw new ClientError(400, 'Incorrect message.');
	}

	let saved = caesarDecryptDb.saveDecrypted(
		username,
		encrypted.key,
		encrypted.cipher,
		message
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await caesarDecryptDb.getTime(
		username,
		encrypted.key,
		message,
		encrypted.cipher
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await caesarDecryptDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

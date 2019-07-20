'use strict';

const ClientError = require('../client-error');
const secure = require('../secure');
const caesarAttackDb = require('../../db/caesar/attack');

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

const attack = (message, cipher) => {
	for (let i=0; i<26; i++) {
		if (cipher === encrypt(message, i)) {
			return i;
		}
	}

	return null;
};

const get = async (username) => {
	let key = await secure.randomNumberInRange(8, 18),
		message = '';

	for (let i=0; i<SIZE; i++) {
		message += LETTERS[await secure.randomNumberInRange(0, LETTERS.length - 1)];
	}

	let cipher = encrypt(message, key);

	let saved = await caesarAttackDb.saveMessageCipher(
		username,
		message,
		cipher
	);

	if (!saved) {
		throw new ClientError();
	}

	return {
		message: message,
		cipher: cipher
	};
};

const submit = async (
	username,
	key
) => {
	if (isNaN(key) ||
		key < 0 ||
		key > 25) {

		throw new ClientError(400, 'Invalid key.');
	}

	let messageCipher = await caesarAttackDb.getMessageCipher(username);

	if (!messageCipher) {
		throw new ClientError(400, 'User has not gotten a message/cipher pair.');
	}

	if (key !== attack(messageCipher.message, messageCipher.cipher)) {
		throw new ClientError(400, 'Incorrect key.');
	}

	let saved = caesarAttackDb.saveKey(
		username,
		messageCipher.message,
		messageCipher.cipher,
		key
	);

	if (!saved) {
		throw new ClientError();
	}

	let seconds = await caesarAttackDb.getTime(
		username,
		key,
		messageCipher.message,
		messageCipher.cipher
	);

	seconds = Math.round(seconds * 10000) / 10000;

	if (isNaN(seconds)) {
		throw new ClientError();
	}

	return `Correct! This attempt took: ${seconds} seconds.`;
};

const getResults = async () => {
	return await caesarAttackDb.getResults();
};

module.exports.get = get;
module.exports.submit = submit;
module.exports.getResults = getResults;

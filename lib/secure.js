'use strict';

const crypto = require('crypto');
const forge = require('node-forge');
const config = require('../config');

const random = (numBytes) => {
	return new Promise((resolve, reject) => {
		crypto.randomBytes(numBytes, (error, buf) => {
			if (error) {
				return reject(error);
			}

			resolve(buf.toString('hex'));
		});
	});
};

const pbkdf2 = (password, salt) => {
	return new Promise((resolve, reject) => {
		crypto.pbkdf2(
			password,
			salt,
			config.pbkdf2.ITERATIONS,
			config.pbkdf2.KEY_LENGTH,
			config.pbkdf2.METHOD,
			(error, derivedKey) => {
				if (error) {
					return reject(error);
				}

				resolve(derivedKey.toString('hex'));
			}
		);
	});
};

const randomNumberInRange = async (min, max) => {
	let maxBytes = 6,
		maxNum = Math.pow(2, 6 * 8),
		distance = max - min;

	for (let i=1; i<=5 && maxBytes == 6; i++) {
		let currentMaxNum = Math.pow(2, i * 8);

		if (distance < currentMaxNum) {
			maxBytes = i;
			maxNum = currentMaxNum;
		}
	}

	let bytes = await random(maxBytes),
		num = parseInt(bytes, 16);

	return Math.floor(num / maxNum * (max - min + 1) + min);
};

const randomPrime = async (bits) => {
	return new Promise((resolve, reject) => {
		forge.prime.generateProbablePrime(bits, (err, num) => {
			if (err) {
				return reject(err);
			}

			resolve(num.data[0]);
		});
	});
};

module.exports.random = random;
module.exports.pbkdf2 = pbkdf2;
module.exports.randomNumberInRange = randomNumberInRange;
module.exports.randomPrime = randomPrime;

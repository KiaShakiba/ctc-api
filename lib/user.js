'use strict';

const config = require('../config');
const ClientError = require('./client-error');
const secure = require('./secure');
const userDb = require('../db/user');

const getWithBearer = async (bearerToken) => {
	return await userDb.getWithBearer(bearerToken);
};

const create = async (
	username,
	password
) => {
	if (!username ||
		!/^[a-z0-9]{2,16}$/i.test(username)) {

		throw new ClientError(400, 'Invalid username.');
	}

	if (!password || password.length < 2) {
		throw new ClientError(400, 'Invalid password.');
	}

	let exists = await userDb.checkExists(username);

	if (exists) {
		throw new ClientError(400, 'Username already exists.');
	}

	let salt = await secure.random(config.pbkdf2.SALT_SIZE),
		hashedPassword = await secure.pbkdf2(password, salt);

	let created = await userDb.create(
		username,
		hashedPassword,
		salt
	);

	if (!created) {
		throw new ClientError();
	}

	let bearerToken = await secure.random(config.user.BEARER_SIZE);

	let bearerCreated = await userDb.createBearer(
		username,
		bearerToken
	);

	if (!bearerCreated) {
		throw new ClientError();
	}

	return bearerToken;
};

const signIn = async (
	username,
	password
) => {
	let salt = await userDb.getSalt(username);

	if (!salt) {
		throw new ClientError(400, 'User does not exist.');
	}

	let hashedPassword = await secure.pbkdf2(password, salt),
		exists = await userDb.checkExistsWithPassword(username, hashedPassword);

	if (!exists) {
		throw new ClientError(400, 'Invalid credentials.');
	}

	let bearerToken = await userDb.getBearer(username);

	if (bearerToken) {
		return bearerToken;
	}

	bearerToken = await secure.random(config.user.BEARER_SIZE);

	let bearerCreated = await userDb.createBearer(
		username,
		bearerToken
	);

	if (!bearerCreated) {
		throw new ClientError();
	}

	return bearerToken;
};

module.exports.getWithBearer = getWithBearer;
module.exports.create = create;
module.exports.signIn = signIn;

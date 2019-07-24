'use strict';

let Query = require('./query');

const getWithBearer = async (bearerToken) => {
	let query = new Query();

	query.command = `
		select U.username, U.type
			from sessions S
				inner join users U
					on U.username=S.username
			where S.token=$1
	`;

	query.values = [bearerToken];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	let user = result.rows[0];

	user.isAdmin = user.type === 'instructor';

	return user;
};

const checkExists = async (username) => {
	let query = new Query();

	query.command = `
		select exists (
			select username
				from users
				where username='${username}'
		)
	`;

	let result = await query.execute();

	return result.rows[0].exists;
};

const checkExistsWithPassword = async (
	username,
	password
) => {
	let query = new Query();

	query.command = `
		select username
				from users
			where username='${username}' and password='${password}'
	`;

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0];
};

const create = async (
	username,
	hashedPassword,
	salt
) => {
	let query = new Query();

	query.command = `
		insert into users
			(username, password, salt)
				values
			($1, $2, $3)
	`;

	query.values = [
		username,
		hashedPassword,
		salt
	];

	let result = await query.execute();

	return result.rowCount === 1;
};

const getSalt = async (username) => {
	let query = new Query();

	query.command = `
		select salt
			from users
			where username='${username}'
	`;

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0].salt;
};

const getBearer = async (username) => {
	let query = new Query();

	query.command = `
		select token
			from sessions
			where username=$1
	`;

	query.values = [username];

	let result = await query.execute();

	if (!result.rowCount) {
		return null;
	}

	return result.rows[0].token;
};

const createBearer = async (
	username,
	bearerToken
) => {
	let query = new Query();

	query.command = `
		insert into sessions
			(username, token)
				values
			($1, $2)
	`;

	query.values = [username, bearerToken];

	let result = await query.execute();

	return result.rowCount === 1;
};

module.exports.getWithBearer = getWithBearer;
module.exports.checkExists = checkExists;
module.exports.checkExistsWithPassword = checkExistsWithPassword;
module.exports.create = create;
module.exports.getSalt = getSalt;
module.exports.getBearer = getBearer;
module.exports.createBearer = createBearer;

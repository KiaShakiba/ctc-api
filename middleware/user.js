'use strict';

const ClientError = require('../lib/client-error');
const user = require('../lib/user');

module.exports = (req, res, next) => {
	let bearerToken = (req.headers.authorization || '')
		.replace(/^Bearer\s/, '');

	if (!bearerToken) {
		return next();
	}

	user.getWithBearer(bearerToken)
		.then((user) => {
			if (!user) {
				throw new ClientError(400, 'Invalid bearer token.');
			}

			req.user = user;
			next();
		})
		.catch(next);
};

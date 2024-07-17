'use strict';

const ClientError = require('../lib/client-error');

const contains = (obj, required) => {
	for (let i=0; i<required.length; i++) {
		if (!obj.hasOwnProperty(required[i])) {
			throw new ClientError(400, `Missing required parameter <${required[i]}>.`);
		}
	}
};

const signedIn = (req) => {
	if (!req.user) {
		throw new ClientError(401, 'Not signed in.');
	}
};

const signedOut = (req) => {
	if (req.user) {
		throw new ClientError(400, 'Already signed in.');
	}
};

const admin = (user) => {
	if (!user.isAdmin) {
		throw new ClientError(403, 'Unauthorized.');
	}
};

const isValidNumber = (num) => {
	if (num.toString().length > 10) {
		throw new ClientError(400, 'The value you have supplied is either not valid or too big.');
	}
};

module.exports.contains = contains;
module.exports.signedIn = signedIn;
module.exports.signedOut = signedOut;
module.exports.admin = admin;
module.exports.isValidNumber = isValidNumber;

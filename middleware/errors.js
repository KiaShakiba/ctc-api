'use strict';

const ClientError = require('../lib/client-error');

module.exports = (err, req, res, next) => {
	if (!err) {
		return next();
	}

	res.set('Content-Type', 'text/plain');

	if (err instanceof ClientError) {
		return res.status(err.code).send(err.message);
	}

	console.error(err);
	res.status(500).send('An error has occurred.');
};

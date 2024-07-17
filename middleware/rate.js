'use strict';

const { rateLimit } = require('express-rate-limit');

const limiter = rateLimit({
	windowMs: 1000,
	limit: 1,
	message: 'You\'re making too many requests! Slow down!',
	standardHeaders: 'draft-7',
	keyGenerator: getBearerToken,
	skip,
});

function skip(req) {
	return !getBearerToken(req);
}

function getBearerToken(req) {
	return (req.headers.authorization || '')
		.replace(/^Bearer\s/, '');
}

module.exports = limiter;

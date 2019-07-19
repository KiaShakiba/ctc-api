'use strict';

const express = require('express');
const check = require('./check');
const user = require('../lib/user');

let router = express.Router();

router.post('/register', (req, res, next) => {
	check.contains(req.body, ['username', 'password']);
	check.signedOut(req);

	let created = user.create(
		req.body.username,
		req.body.password
	);

	created
		.then((data) => {
			res.set('Content-Type', 'text/plain');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/sign-in', (req, res, next) => {
	check.contains(req.body, ['username', 'password']);
	check.signedOut(req);

	let signedIn = user.signIn(
		req.body.username,
		req.body.password
	);

	signedIn
		.then((data) => {
			res.set('Content-Type', 'text/plain');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

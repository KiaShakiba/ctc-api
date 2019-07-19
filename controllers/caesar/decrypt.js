'use strict';

const express = require('express');
const check = require('../check');
const caesarDecrypt = require('../../lib/caesar/decrypt');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	caesarDecrypt.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['message']);
	check.signedIn(req);

	let submitted = caesarDecrypt.submit(
		req.user.username,
		req.body.message
	);

	submitted
		.then((data) => {
			res.set('Content-Type', 'text/plain');
			res.status(200).send(data);
		})
		.catch(next);
});

router.get('/results', (req, res, next) => {
	check.signedIn(req);
	check.admin(req.user);

	caesarDecrypt.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

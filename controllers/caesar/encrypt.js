'use strict';

const express = require('express');
const check = require('../check');
const caesarEncrypt = require('../../lib/caesar/encrypt');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	caesarEncrypt.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['cipher']);
	check.isValidNumber(req.body.cipher);
	check.signedIn(req);

	let submitted = caesarEncrypt.submit(
		req.user.username,
		req.body.cipher
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

	caesarEncrypt.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

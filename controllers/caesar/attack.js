'use strict';

const express = require('express');
const check = require('../check');
const caesarAttack = require('../../lib/caesar/attack');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	caesarAttack.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['key']);
	check.signedIn(req);

	let submitted = caesarAttack.submit(
		req.user.username,
		parseInt(req.body.key)
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

	caesarAttack.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

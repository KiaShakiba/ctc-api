'use strict';

const express = require('express');
const check = require('../check');
const rsaDecrypt = require('../../lib/rsa/decrypt');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	rsaDecrypt.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['message']);
	check.signedIn(req);

	let submitted = rsaDecrypt.submit(
		req.user.username,
		parseInt(req.body.message)
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

	rsaDecrypt.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

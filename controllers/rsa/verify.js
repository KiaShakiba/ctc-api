'use strict';

const express = require('express');
const check = require('../check');
const rsaVerify = require('../../lib/rsa/verify');

let router = express.Router();

router.post('/', (req, res, next) => {
	check.contains(req.body, ['p', 'q', 'e', 'd', 'message', 'cipher']);
	check.isValidNumber(req.body.p);
	check.isValidNumber(req.body.q);
	check.isValidNumber(req.body.e);
	check.isValidNumber(req.body.d);
	check.isValidNumber(req.body.message);
	check.isValidNumber(req.body.cipher);
	check.signedIn(req);

	let submitted = rsaVerify.submit(
		req.user.username,
		parseInt(req.body.p),
		parseInt(req.body.q),
		parseInt(req.body.e),
		parseInt(req.body.d),
		parseInt(req.body.message),
		parseInt(req.body.cipher)
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

	rsaVerify.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

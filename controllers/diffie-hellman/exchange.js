'use strict';

const express = require('express');
const check = require('../check');
const diffieHellmanExchange = require('../../lib/diffie-hellman/exchange');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	diffieHellmanExchange.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['pk', 'k']);
	check.signedIn(req);

	let submitted = diffieHellmanExchange.submit(
		req.user.username,
		parseInt(req.body.pk),
		parseInt(req.body.k)
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

	diffieHellmanExchange.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

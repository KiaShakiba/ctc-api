'use strict';

const express = require('express');
const check = require('../check');
const dssVerify = require('../../lib/dss/verify');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	dssVerify.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['u', 'v', 'w']);
	check.signedIn(req);

	let submitted = dssVerify.submit(
		req.user.username,
		parseInt(req.body.u),
		parseInt(req.body.v),
		parseInt(req.body.w)
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

	dssVerify.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

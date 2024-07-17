'use strict';

const express = require('express');
const check = require('../check');
const dssSign = require('../../lib/dss/sign');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	dssSign.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

router.post('/', (req, res, next) => {
	check.contains(req.body, ['pk', 'r', 's']);
	check.isValidNumber(req.body.pk);
	check.isValidNumber(req.body.r);
	check.isValidNumber(req.body.s);
	check.signedIn(req);

	let submitted = dssSign.submit(
		req.user.username,
		parseInt(req.body.pk),
		parseInt(req.body.r),
		parseInt(req.body.s)
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

	dssSign.getResults()
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

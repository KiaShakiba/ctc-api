'use strict';

const express = require('express');
const check = require('../check');
const rsaEncrypt = require('../../lib/rsa/encrypt');

let router = express.Router();

router.get('/', (req, res, next) => {
	check.signedIn(req);

	rsaEncrypt.get(req.user.username)
		.then((data) => {
			res.set('Content-Type', 'application/json');
			res.status(200).send(data);
		})
		.catch(next);
});

module.exports = router;

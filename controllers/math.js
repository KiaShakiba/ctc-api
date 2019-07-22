'use strict';

const express = require('express');
const check = require('./check');
const math = require('../lib/math');

let router = express.Router();

router.get('/coprime', (req, res, next) => {
	check.contains(req.query, ['number']);

	res.set('Content-Type', 'application/json');
	res.status(200).send(math.getCoprimes(
		parseInt(req.query.number)
	));
});

router.get('/power-mod', (req, res, next) => {
	check.contains(req.query, ['number', 'exponent', 'modulus']);

	res.set('Content-Type', 'text/plain');
	res.status(200).send(math.getPowerMod(
		parseInt(req.query.number),
		parseInt(req.query.exponent),
		parseInt(req.query.modulus)
	).toString());
});

router.get('/inverse-mod', (req, res, next) => {
	check.contains(req.query, ['number', 'modulus']);

	res.set('Content-Type', 'text/plain');
	res.status(200).send(math.getInverseMod(
		parseInt(req.query.number),
		parseInt(req.query.modulus)
	).toString());
});

module.exports = router;

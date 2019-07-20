'use strict';

const express = require('express');

let router = express.Router();

router.use('/encrypt', require('./encrypt'));
router.use('/decrypt', require('./decrypt'));
router.use('/verify', require('./verify'));

module.exports = router;

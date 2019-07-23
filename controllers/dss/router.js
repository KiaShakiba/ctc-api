'use strict';

const express = require('express');

let router = express.Router();

router.use('/verify', require('./verify'));
router.use('/sign', require('./sign'));

module.exports = router;

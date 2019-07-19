'use strict';

const express = require('express');

let router = express.Router();

router.use('/encrypt', require('./encrypt'));
//router.use('/decrypt', require('./decrypt'));
//router.use('/attack', require('./attack'));

module.exports = router;

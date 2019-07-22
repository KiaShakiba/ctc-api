'use strict';

const express = require('express');

let router = express.Router();

router.use('/exchange', require('./exchange'));

module.exports = router;

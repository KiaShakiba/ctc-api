'use strict';

const express = require('express');
const bodyParser = require('body-parser');
const config = require('./config');
const cors = require('./middleware/cors');
const user = require('./middleware/user');
const errors = require('./middleware/errors');

let app = express();

const userController = require('./controllers/user');
const caesarRouter = require('./controllers/caesar/router');
const rsaRouter = require('./controllers/rsa/router');

app.enable('trust proxy');

app.use(cors);
app.use(bodyParser.json());
app.use(user);

app.use('/user', userController);
app.use('/caesar', caesarRouter);
app.use('/rsa', rsaRouter);
app.use(errors);

app.listen(config.PORT, () => {
	console.log(`Listening on port ${config.PORT}`);
});

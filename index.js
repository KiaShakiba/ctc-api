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
const diffieHellmanRouter = require('./controllers/diffie-hellman/router');
const mathController = require('./controllers/math');

app.enable('trust proxy');

app.use(cors);
app.use(bodyParser.json());
app.use(user);

app.get('/', (req, res, next) => {
	res.set('Content-Type', 'text/plain');
	res.status(200).end('Welcome to the Cracking the Code API!');
});

app.use('/user', userController);
app.use('/caesar', caesarRouter);
app.use('/diffie-hellman', diffieHellmanRouter);
app.use('/rsa', rsaRouter);
app.use('/math', mathController);

app.use('*', (req, res, next) => {
	res.set('Content-Type', 'text/plain');
	res.status(200).end('Not found.');
});

app.use(errors);

app.listen(config.PORT, () => {
	console.log(`Listening on port ${config.PORT}`);
});

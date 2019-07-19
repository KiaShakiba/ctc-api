'use strict';

const pg = require('pg');
const config = require('../config');

pg.types.setTypeParser(1700, 'text', parseFloat);

const pgConfig = config.db.URL ?
	{
		connectionString: config.db.URL,
		max: config.db.MAX_POOL,
		idleTimeoutMillis: config.db.POOL_IDLE
	} :
	{
		user: config.db.USERNAME,
		password: config.db.PASSWORD,
		host: config.db.HOST,
		database: config.db.NAME,
		port: config.db.PORT,
		max: config.db.MAX_POOL,
		idleTimeoutMillis: config.db.POOL_IDLE
	};

const pool = new pg.Pool(pgConfig);

class Query {
	constructor() {
		this._command = '';
		this._values = [];
	}

	set command(value) {
		this._command = value;
	}

	set values(value) {
		this._values = value;
	}

	async execute() {
		const client = await pool.connect();

		try {
			const result = await client.query(this._command, this._values);

			result.rows = result.rows || [];

			return result;
		} catch (error) {
			throw new Error(error);
		} finally {
			client.release();
		}
	}
}

module.exports = Query;

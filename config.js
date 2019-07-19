'use strict';

module.exports = {
	PORT: process.env.PORT || 3300,
	db: {
		URL: process.env.DATABSE_URL,
		HOST: process.env.DB_HOST,
		USERNAME: process.env.DB_USERNAME,
		PASSWORD: process.env.DB_PASSWORD,
		NAME: 'cracking-the-code-api',
		PORT: 5432,
		MAX_POOL: 20,
		POOL_IDLE: 1000
	},
	pbkdf2: {
		SALT_SIZE: 64,
		ITERATIONS: 10000,
		METHOD: 'sha512',
		KEY_LENGTH: 512
	},
	user: {
		BEARER_SIZE: 256
	}
};

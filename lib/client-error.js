'use strict';

class ClientError extends Error {
	constructor(code, message) {
		let errorMessage = message || 'An error has occurred.';
		let errorCode = code || 500;

		super(errorMessage);

		this._message = errorMessage;
		this._code = errorCode;
	}

	get code() {
		return this._code;
	}

	get message() {
		return this._message;
	}
}

module.exports = ClientError;

'use strict';

module.exports = (req, res, next) => {
	let host = req.get('host'),
		path = req.path;

	if (process.env.FORCE_TLS && !req.secure) {
		res.redirect(`https://${host}${path}`);
		res.end();
	} else {
		next();
	}
};

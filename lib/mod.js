'use strict';

Number.prototype.mod = function(modulus) {
	return ((this % modulus) + modulus) % modulus;
};

Number.prototype.powerMod = function(exp, modulus) {
	let result = 1,
		num = this;

	num = num.mod(modulus);

	while (exp > 0) {
		if (exp & 1) {
			result = (result * num).mod(modulus);
		}

		exp = exp >> 1;
		num = (num * num).mod(modulus);
	}

	return result;
};

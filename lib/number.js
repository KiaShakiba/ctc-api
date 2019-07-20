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

Number.prototype.inverseMod = function(modulus) {
	let a = this % modulus;

	for (let i=1; i<modulus; i++) {
		if ((a * i) % modulus === 1) {
			return i;
		}
	}

	return null;
};

Number.prototype.gcd = function(num) {
	if (num === 0) {
		return this;
	}

	return num.gcd(this % num);
};

Number.prototype.isPrime = function() {
	for (let i=2; i<this; i++) {
		if (this % i === 0) {
			return false;
		}
	}

	return this > 1;
};

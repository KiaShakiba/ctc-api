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

Number.prototype.coprimes = function() {
	let nums = [];

	for (let i=2; i<this; i++) {
		if (this.gcd(i) === 1) {
			nums.push(i);
		}
	}

	return nums;
};

Number.prototype.primeFactors = function() {
	let factors = [],
		num = this;

	while (num % 2 === 0) {
		factors.push(2);
		num /= 2;
	}

	for (let i=3; i<=Math.floor(Math.sqrt(num)); i+=2) {
		while (num % i === 0) {
			factors.push(i);
			num /= i;
		}
	}

	if (num > 2) {
		factors.push(num);
	}

	return factors;
};

Number.prototype.primitive = function() {
	if (!this.isPrime()) {
		return null;
	}

	let phi = this - 1,
		primeFactors = phi.primeFactors();

	for (let i=2; i<phi; i++) {
		for (let j=0; j<primeFactors.length; j++) {
			let factor = primeFactors[j];

			if (i.powerMod(Math.floor(phi / factor), this) === 1) {
				return i;
			}
		}
	}

	return null;
};

Number.prototype.order = function(p) {
	for (let i=1; i<=p; i++) {
		if (this.powerMod(i, p) === 1) {
			return i;
		}
	}

	return null;
};

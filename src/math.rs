use num_traits::AsPrimitive;
use primal_sieve::Sieve;

pub fn safe_mod(num: impl AsPrimitive<i64>, modulus: impl AsPrimitive<i64>) -> u64 {
	let num = num.as_();
	let modulus = modulus.as_();

	num.rem_euclid(modulus) as u64
}

pub fn power_mod(
	num: impl AsPrimitive<u64>,
	exp: impl AsPrimitive<u64>,
	modulus: impl AsPrimitive<u64>,
) -> u64 {
	let mut exp = exp.as_();
	let modulus = modulus.as_();

	let mut result = 1;
	let mut num = safe_mod(num.as_(), modulus);

	while exp > 0 {
		if exp & 1 > 0 {
			result = safe_mod(result * num, modulus);
		}

		exp >>= 1;
		num = safe_mod(num.pow(2), modulus);
	}

	result
}

pub fn is_prime(num: impl AsPrimitive<usize>) -> bool {
	let num = num.as_();
	Sieve::new(num + 1).is_prime(num)
}

pub fn prime_factors(num: impl AsPrimitive<u64>) -> Vec<u64> {
	let mut factors = Vec::<u64>::new();
	let mut num = num.as_();

	while num & 1 == 0 {
		factors.push(2);
		num /= 2;
	}

	for i in (3..=num.isqrt()).step_by(2) {
		while safe_mod(num, i) == 0 {
			factors.push(i);
			num /= i;
		}
	}

	if num > 2 {
		factors.push(num);
	}

	factors
}

pub fn primitive(num: impl AsPrimitive<u64>) -> Option<u64> {
	let num = num.as_();

	if !is_prime(num) {
		return None;
	}

	let phi = num - 1;
	let factors = prime_factors(phi);

	for i in 2..phi {
		for factor in &factors {
			if power_mod(i, phi / factor, num) == 1 {
				return Some(i);
			}
		}
	}

	None
}

pub fn gcd(a: impl AsPrimitive<u64>, b: impl AsPrimitive<u64>) -> u64 {
	let a = a.as_();
	let b = b.as_();

	if b == 0 {
		return a;
	}

	gcd(b, safe_mod(a, b))
}

pub fn inverse_mod(num: impl AsPrimitive<u64>, modulus: impl AsPrimitive<u64>) -> Option<u64> {
	let num = num.as_();
	let modulus = modulus.as_();

	let a = safe_mod(num, modulus);
	(1..modulus).find(|i| safe_mod(a * i, modulus) == 1)
}

pub fn order(num: impl AsPrimitive<u64>, p: impl AsPrimitive<u64>) -> Option<u64> {
	let num = num.as_();
	let p = p.as_();

	(1..=p).find(|i| power_mod(num, *i, p) == 1)
}

#[cfg(test)]
mod tests {
	use crate::math::*;

	#[test]
	fn it_calculates_safe_mods() {
		assert_eq!(5, safe_mod(125, 20));
		assert_eq!(8, safe_mod(600, 37));
		assert_eq!(10, safe_mod(-523, 13));
	}

	#[test]
	fn it_calculates_power_mods() {
		assert_eq!(101, power_mod(8, 20, 125));
		assert_eq!(12, power_mod(3, 5, 21));
		assert_eq!(68, power_mod(18, 325, 500));
	}

	#[test]
	fn it_checks_prime_numbers() {
		assert!(is_prime(5));
		assert!(is_prime(1543));
		assert!(is_prime(7477));
		assert!(!is_prime(125));
		assert!(!is_prime(620));
	}

	#[test]
	fn it_finds_prime_factors() {
		assert_eq!(vec![2, 2, 5, 5, 5], prime_factors(500));
		assert_eq!(vec![3, 41], prime_factors(123));
		assert_eq!(vec![7, 103], prime_factors(721));
		assert_eq!(vec![1543], prime_factors(1543));
	}

	#[test]
	fn it_finds_primitives() {
		assert_eq!(None, primitive(5));
		assert_eq!(Some(2), primitive(1543));
		assert_eq!(Some(3), primitive(7477));
		assert_eq!(None, primitive(125));
		assert_eq!(None, primitive(620));
	}

	#[test]
	fn it_finds_gcds() {
		assert_eq!(5, gcd(20, 125));
		assert_eq!(25, gcd(600, 325));
		assert_eq!(1, gcd(27, 1543));
	}

	#[test]
	fn it_finds_inverse_mods() {
		assert_eq!(Some(4), inverse_mod(16, 7));
		assert_eq!(Some(13), inverse_mod(225, 17));
		assert_eq!(Some(10), inverse_mod(1543, 37));
	}

	#[test]
	fn it_calculates_orders() {
		assert_eq!(Some(3), order(16, 7));
		assert_eq!(Some(4), order(225, 17));
		assert_eq!(Some(3), order(1543, 37));
	}
}

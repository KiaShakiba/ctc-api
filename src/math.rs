use num_traits::AsPrimitive;
use primal_sieve::Sieve;

pub fn power_mod(
	num: impl AsPrimitive<u64>,
	exp: impl AsPrimitive<u64>,
	modulus: impl AsPrimitive<u64>,
) -> u64 {
	let mut exp = exp.as_();
	let modulus = modulus.as_();

	let mut result = 1;
	let mut num = num.as_() % modulus;

	while exp > 0 {
		if exp & 1 > 0 {
			result = (result * num) % modulus;
		}

		exp >>= 1;
		num = num.pow(2) % modulus;
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
		while num % i == 0 {
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

#[cfg(test)]
mod tests {
	use crate::math::*;

	#[test]
	fn it_calculates_power_mods_correctly() {
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
}

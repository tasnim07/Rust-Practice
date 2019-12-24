pub fn nth(n: u32) -> u32 {
    let mut prime_numbers: Vec<u32> = Vec::new();
    let count = 1;
    while prime_numbers.len() as u32 <= n {
	if count == 1 {
	    prime_numbers.push(count)
	} else if count 
    }
    unimplemented!("What is the 0-indexed {}th prime number?", n)
}


fn is_prime(n: u32, primes: Vec<u32>) -> bool {
    for p in primes {
	if n % p == 0 {
	    return false
	}
    }
    true
}

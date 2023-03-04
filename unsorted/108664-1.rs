fn prime_elem(n: u64) -> u64 {
    let mut primes = Vec::new();
    let mut j = 0;

    for i in 2..n+1 {
        if primes.contains(i) {
            j = i*i;
            while j < n+1 {
                primes.push(j);
                j += i;
            }
        }
    }

    primes[n]
}

fn main() {
    let _prime = prime_elem(10001);
    println!("{_prime}");
}

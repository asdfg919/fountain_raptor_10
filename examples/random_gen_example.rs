use fountain_raptor_10::generator::random_generator::rand;

fn main() {
    println!("=== RFC 5053 Random Generator Example ===\n");

    // Example 1: Basic usage
    println!("Example 1: Basic random number generation");
    let x = 100;
    let i = 5;
    let m = 1000;
    let result = rand(x, i, m);
    println!("  Rand[{}, {}, {}] = {}", x, i, m, result);
    println!("  (Result is in range [0, {}))\n", m);

    // Example 2: Generate a sequence
    println!("Example 2: Generate a sequence with varying i");
    let x = 42;
    let m = 100;
    print!("  Rand[{}, i, {}] for i=0..10: ", x, m);
    for i in 0..10 {
        print!("{} ", rand(x, i, m));
    }
    println!("\n");

    // Example 3: Generate with different X values
    println!("Example 3: Generate with different X values");
    let i = 0;
    let m = 1000;
    println!("  X\t\tRand[X, {}, {}]", i, m);
    for x in [0, 100, 1000, 10000, 100000].iter() {
        println!("  {}\t\t{}", x, rand(*x, i, m));
    }
    println!();

    // Example 4: Demonstrate determinism
    println!("Example 4: Deterministic behavior");
    let x = 12345;
    let i = 67;
    let m = 5000;
    let result1 = rand(x, i, m);
    let result2 = rand(x, i, m);
    println!("  First call:  Rand[{}, {}, {}] = {}", x, i, m, result1);
    println!("  Second call: Rand[{}, {}, {}] = {}", x, i, m, result2);
    println!("  Results match: {}\n", result1 == result2);

    // Example 5: Distribution visualization (simple)
    println!("Example 5: Simple distribution test");
    let m = 10;
    let mut counts = vec![0; m as usize];
    
    // Generate 1000 random numbers and count occurrences
    for x in 0..1000 {
        let value = rand(x, 0, m);
        counts[value as usize] += 1;
    }
    
    println!("  Distribution of Rand[x, 0, {}] for x=0..1000:", m);
    for (value, count) in counts.iter().enumerate() {
        let bar = "=".repeat(count / 2);
        println!("  {} [{:3}]: {}", value, count, bar);
    }

    println!("\n=== Example Complete ===");
}


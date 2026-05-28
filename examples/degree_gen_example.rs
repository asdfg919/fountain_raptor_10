use fountain_raptor_10::generator::{degree_generator::deg, random_generator::rand};

fn main() {
    println!("=== RFC 5053 Degree Generator Example ===\n");

    // Example 1: Basic usage
    println!("Example 1: Basic degree generation");
    let test_values = [0, 5000, 10241, 250000, 500000, 800000, 950000, 1040000, 1048575];
    println!("  v\t\tDeg[v]");
    for v in test_values.iter() {
        println!("  {}\t\t{}", v, deg(*v));
    }
    println!();

    // Example 2: Degree distribution visualization
    println!("Example 2: Degree distribution from RFC 5053 Table 1");
    println!("  Degree | Range Start | Range End   | Range Size | Probability");
    println!("  -------|-------------|-------------|------------|------------");
    
    let ranges = [
        (1,  0,       10240,    10241),
        (2,  10241,   491581,   481341),
        (3,  491582,  712793,   221212),
        (4,  712794,  831694,   118901),
        (10, 831695,  948445,   116751),
        (11, 948446,  1032188,  83743),
        (40, 1032189, 1048575,  16387),
    ];
    
    for (degree, start, end, size) in ranges.iter() {
        let prob = (*size as f64 / 1048576.0) * 100.0;
        println!("  {:6} | {:11} | {:11} | {:10} | {:6.2}%",
                degree, start, end, size, prob);
    }
    println!();

    // Example 3: Generate degrees using random numbers
    println!("Example 3: Generate degrees using Rand[X, i, m]");
    println!("  Combining rand() and deg() to generate random degrees:");
    println!("  X\ti\tRand[X,i,1048576]\tDeg[v]");
    
    for x in 0..10 {
        let v = rand(x * 100, 0, 1048576);
        let d = deg(v);
        println!("  {}\t0\t{}\t\t\t{}", x * 100, v, d);
    }
    println!();

    // Example 4: Statistical analysis
    println!("Example 4: Statistical analysis (10000 samples)");
    let mut degree_counts = [0u32; 41];
    let num_samples = 10000;
    
    // Generate random v values and count degree occurrences
    for i in 0..num_samples {
        let v = rand(i, 0, 1048576);
        let d = deg(v);
        degree_counts[d as usize] += 1;
    }
    
    println!("  Degree | Count | Percentage | Bar Chart");
    println!("  -------|-------|------------|--------------------");
    
    for d in [1, 2, 3, 4, 10, 11, 40].iter() {
        let count = degree_counts[*d as usize];
        let percentage = (count as f64 / num_samples as f64) * 100.0;
        let bar_length = (percentage * 0.5) as usize; // Scale for display
        let bar = "█".repeat(bar_length);
        println!("  {:6} | {:5} | {:8.2}% | {}", d, count, percentage, bar);
    }
    println!();

    // Example 5: Practical encoding example
    println!("Example 5: Simulating encoding symbol degree generation");
    println!("  Generating degrees for first 20 encoding symbols:");
    println!("  ESI (X) | v = Rand[X,1,2^20] | Degree");
    println!("  --------|---------------------|-------");
    
    for x in 0..20 {
        let v = rand(x, 1, 1048576); // Use i=1 as per typical RFC usage
        let d = deg(v);
        println!("  {:7} | {:19} | {:6}", x, v, d);
    }
    println!();

    // Example 6: Degree statistics
    println!("Example 6: Expected degree statistics");
    let total = 1048576u64;
    let ranges = [
        (1,  10241u64),
        (2,  481341u64),
        (3,  221212u64),
        (4,  118901u64),
        (10, 116751u64),
        (11, 83743u64),
        (40, 16387u64),
    ];
    
    let mut expected_degree = 0.0f64;
    for (degree, size) in ranges.iter() {
        let prob = *size as f64 / total as f64;
        expected_degree += (*degree as f64) * prob;
    }
    
    println!("  Expected (average) degree: {:.4}", expected_degree);
    println!("  This means on average, each encoding symbol");
    println!("  connects to ~{:.1} source symbols", expected_degree);

    println!("\n=== Example Complete ===");
}


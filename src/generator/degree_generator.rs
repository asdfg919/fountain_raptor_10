/// Degree generator as specified in RFC 5053 Section 5.4.4.2
/// 
/// Returns the degree for a given value v, where v is an integer
/// in the range [0, 2^20) = [0, 1048576).
/// 
/// # Parameters
/// - `v`: A non-negative integer less than 1048576 (2^20)
/// 
/// # Returns
/// The degree d[j] corresponding to the input v
/// 
/// # Algorithm
/// Find index j such that f[j-1] <= v < f[j], then return d[j]
/// 
/// # Panics
/// Panics if v >= 1048576
pub fn deg(v: u32) -> u32 {
    // Validate input range
    assert!(v < 1048576, "v must be less than 2^20 = 1048576, got {}", v);
    
    // Degree distribution table from RFC 5053 Table 1
    // Each entry is (f[j], d[j]) where f[j] is the cumulative threshold
    // We find j such that f[j-1] <= v < f[j], then return d[j]
    const TABLE: [(u32, u32); 8] = [
        (0,       0),   // j=0: f[0]=0,       d[0]=-- (not used)
        (10241,   1),   // j=1: f[1]=10241,   d[1]=1
        (491582,  2),   // j=2: f[2]=491582,  d[2]=2
        (712794,  3),   // j=3: f[3]=712794,  d[3]=3
        (831695,  4),   // j=4: f[4]=831695,  d[4]=4
        (948446,  10),  // j=5: f[5]=948446,  d[5]=10
        (1032189, 11),  // j=6: f[6]=1032189, d[6]=11
        (1048576, 40),  // j=7: f[7]=1048576, d[7]=40
    ];
    
    // Find the index j such that f[j-1] <= v < f[j]
    // We can use binary search or linear search
    // Since the table is small (8 entries), linear search is efficient
    for j in 1..TABLE.len() {
        if v < TABLE[j].0 {
            return TABLE[j].1;
        }
    }
    
    // This should never happen if v < 1048576
    unreachable!("v={} should have matched a table entry", v);
}

/// Alternative implementation using match for better performance
/// This is more efficient as the compiler can optimize it better
#[allow(dead_code)]
pub fn deg_optimized(v: u32) -> u32 {
    assert!(v < 1048576, "v must be less than 2^20 = 1048576, got {}", v);
    
    match v {
        0..=10240      => 1,   // f[0]=0     <= v < f[1]=10241
        10241..=491581 => 2,   // f[1]=10241 <= v < f[2]=491582
        491582..=712793 => 3,  // f[2]=491582 <= v < f[3]=712794
        712794..=831694 => 4,  // f[3]=712794 <= v < f[4]=831695
        831695..=948445 => 10, // f[4]=831695 <= v < f[5]=948446
        948446..=1032188 => 11, // f[5]=948446 <= v < f[6]=1032189
        1032189..=1048575 => 40, // f[6]=1032189 <= v < f[7]=1048576
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deg_boundary_values() {
        // Test boundary values for each degree
        assert_eq!(deg(0), 1);           // First value, degree 1
        assert_eq!(deg(10240), 1);       // Last value before f[1]
        assert_eq!(deg(10241), 2);       // First value of degree 2
        assert_eq!(deg(491581), 2);      // Last value before f[2]
        assert_eq!(deg(491582), 3);      // First value of degree 3
        assert_eq!(deg(712793), 3);      // Last value before f[3]
        assert_eq!(deg(712794), 4);      // First value of degree 4
        assert_eq!(deg(831694), 4);      // Last value before f[4]
        assert_eq!(deg(831695), 10);     // First value of degree 10
        assert_eq!(deg(948445), 10);     // Last value before f[5]
        assert_eq!(deg(948446), 11);     // First value of degree 11
        assert_eq!(deg(1032188), 11);    // Last value before f[6]
        assert_eq!(deg(1032189), 40);    // First value of degree 40
        assert_eq!(deg(1048575), 40);    // Last valid value
    }

    #[test]
    fn test_deg_middle_values() {
        // Test some middle values in each range
        assert_eq!(deg(5000), 1);
        assert_eq!(deg(250000), 2);
        assert_eq!(deg(600000), 3);
        assert_eq!(deg(770000), 4);
        assert_eq!(deg(890000), 10);
        assert_eq!(deg(990000), 11);
        assert_eq!(deg(1040000), 40);
    }

    #[test]
    #[should_panic(expected = "v must be less than 2^20")]
    fn test_deg_out_of_range() {
        deg(1048576); // Should panic
    }

    #[test]
    #[should_panic(expected = "v must be less than 2^20")]
    fn test_deg_large_value() {
        deg(2000000); // Should panic
    }

    #[test]
    fn test_deg_optimized_matches_deg() {
        // Test that both implementations give the same results
        for v in [0, 100, 10240, 10241, 491581, 491582, 712793, 712794,
                  831694, 831695, 948445, 948446, 1032188, 1032189, 1048575] {
            assert_eq!(deg(v), deg_optimized(v), 
                      "Mismatch at v={}: deg={}, deg_optimized={}", 
                      v, deg(v), deg_optimized(v));
        }
    }

    #[test]
    fn test_degree_distribution() {
        // Test the distribution of degrees
        let mut degree_counts = [0u32; 41]; // Degrees from 0 to 40
        
        // Sample 1000 evenly distributed values
        for i in 0..1000 {
            let v = (i * 1048575) / 999; // Distribute evenly across range
            let degree = deg(v as u32);
            degree_counts[degree as usize] += 1;
        }
        
        // Verify that only expected degrees are used
        for d in 0..=40 {
            match d {
                1 | 2 | 3 | 4 | 10 | 11 | 40 => {
                    assert!(degree_counts[d] > 0, "Degree {} should be used", d);
                }
                _ => {
                    assert_eq!(degree_counts[d], 0, "Degree {} should not be used", d);
                }
            }
        }
        
        // Degree 2 should be most common (largest range: 491582-10241 = 481341)
        assert!(degree_counts[2] > degree_counts[1], "Degree 2 should be more common than 1");
        assert!(degree_counts[2] > degree_counts[40], "Degree 2 should be more common than 40");
    }

    #[test]
    fn test_degree_ranges() {
        // Verify the size of each range
        let ranges = [
            (0, 10240, 1),           // Range size: 10241
            (10241, 491581, 2),      // Range size: 481341
            (491582, 712793, 3),     // Range size: 221212
            (712794, 831694, 4),     // Range size: 118901
            (831695, 948445, 10),    // Range size: 116751
            (948446, 1032188, 11),   // Range size: 83743
            (1032189, 1048575, 40),  // Range size: 16387
        ];
        
        for (start, end, expected_degree) in ranges.iter() {
            assert_eq!(deg(*start), *expected_degree);
            assert_eq!(deg(*end), *expected_degree);
            
            // Test a value in the middle
            if start < end {
                let mid = (start + end) / 2;
                assert_eq!(deg(mid), *expected_degree);
            }
        }
    }
}


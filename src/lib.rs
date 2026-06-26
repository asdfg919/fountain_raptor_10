/// Raptor-10 (RFC 5053) implementation
/// 
/// This library implements the Raptor-10 fountain code as specified in RFC 5053.

pub mod generator;
pub mod raptor_10;
pub mod cached_R10HDPC;

pub use generator::*;
pub use raptor_10::Raptor10SysCode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator() {
        // Basic test to ensure the random generator works
        let result = generator::random_generator::rand(100, 5, 1000);
        assert!(result < 1000);
    }

    #[test]
    fn test_degree_generator() {
        // Basic test to ensure the degree generator works
        assert_eq!(generator::degree_generator::deg(0), 1);
        assert_eq!(generator::degree_generator::deg(500000), 3);
        assert_eq!(generator::degree_generator::deg(1048575), 40);
    }

    #[test]
    fn test_combined_usage() {
        // Test combining rand and deg for typical use case
        for x in 0..100 {
            let v = generator::random_generator::rand(x, 1, 1048576);
            let d = generator::degree_generator::deg(v);
            
            // Degree should be one of the valid values
            assert!(matches!(d, 1 | 2 | 3 | 4 | 10 | 11 | 40),
                   "Invalid degree {} for v={}", d, v);
        }
    }
}

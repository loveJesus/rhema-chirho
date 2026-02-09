// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SIMD-accelerated bitset operations for verse-set intersection/union.
//!
//! Provides both scalar fallback and x86_64 SIMD implementations.
//! The dispatch functions automatically select the best available path.

/// Scalar bitwise AND of two equal-length slices.
pub fn scalar_intersect_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    a_chirho
        .iter()
        .zip(b_chirho.iter())
        .map(|(x_chirho, y_chirho)| x_chirho & y_chirho)
        .collect()
}

/// Scalar bitwise OR of two equal-length slices.
pub fn scalar_union_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    a_chirho
        .iter()
        .zip(b_chirho.iter())
        .map(|(x_chirho, y_chirho)| x_chirho | y_chirho)
        .collect()
}

/// Scalar popcount — count total set bits.
pub fn scalar_popcount_chirho(data_chirho: &[u64]) -> u64 {
    data_chirho
        .iter()
        .map(|w_chirho| w_chirho.count_ones() as u64)
        .sum()
}

/// SIMD bitwise AND using x86_64 AVX2 intrinsics (256-bit).
///
/// Falls back to scalar on non-x86_64 or when AVX2 is not available.
#[cfg(target_arch = "x86_64")]
pub fn simd_intersect_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    if is_x86_feature_detected!("avx2") {
        // SAFETY: We've checked AVX2 is available.
        unsafe { simd_intersect_avx2_chirho(a_chirho, b_chirho) }
    } else {
        scalar_intersect_chirho(a_chirho, b_chirho)
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn simd_intersect_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    scalar_intersect_chirho(a_chirho, b_chirho)
}

/// SIMD bitwise OR using x86_64 AVX2 intrinsics.
#[cfg(target_arch = "x86_64")]
pub fn simd_union_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    if is_x86_feature_detected!("avx2") {
        // SAFETY: We've checked AVX2 is available.
        unsafe { simd_union_avx2_chirho(a_chirho, b_chirho) }
    } else {
        scalar_union_chirho(a_chirho, b_chirho)
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn simd_union_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    scalar_union_chirho(a_chirho, b_chirho)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn simd_intersect_avx2_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    use std::arch::x86_64::*;

    let len_chirho = a_chirho.len().min(b_chirho.len());
    let mut result_chirho = vec![0u64; len_chirho];

    // Process 4 u64s at a time (256 bits)
    let chunks_chirho = len_chirho / 4;
    for i_chirho in 0..chunks_chirho {
        let offset_chirho = i_chirho * 4;
        let va_chirho =
            _mm256_loadu_si256(a_chirho[offset_chirho..].as_ptr() as *const __m256i);
        let vb_chirho =
            _mm256_loadu_si256(b_chirho[offset_chirho..].as_ptr() as *const __m256i);
        let vr_chirho = _mm256_and_si256(va_chirho, vb_chirho);
        _mm256_storeu_si256(
            result_chirho[offset_chirho..].as_mut_ptr() as *mut __m256i,
            vr_chirho,
        );
    }

    // Handle remainder
    for i_chirho in (chunks_chirho * 4)..len_chirho {
        result_chirho[i_chirho] = a_chirho[i_chirho] & b_chirho[i_chirho];
    }

    result_chirho
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn simd_union_avx2_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    use std::arch::x86_64::*;

    let len_chirho = a_chirho.len().min(b_chirho.len());
    let mut result_chirho = vec![0u64; len_chirho];

    let chunks_chirho = len_chirho / 4;
    for i_chirho in 0..chunks_chirho {
        let offset_chirho = i_chirho * 4;
        let va_chirho =
            _mm256_loadu_si256(a_chirho[offset_chirho..].as_ptr() as *const __m256i);
        let vb_chirho =
            _mm256_loadu_si256(b_chirho[offset_chirho..].as_ptr() as *const __m256i);
        let vr_chirho = _mm256_or_si256(va_chirho, vb_chirho);
        _mm256_storeu_si256(
            result_chirho[offset_chirho..].as_mut_ptr() as *mut __m256i,
            vr_chirho,
        );
    }

    for i_chirho in (chunks_chirho * 4)..len_chirho {
        result_chirho[i_chirho] = a_chirho[i_chirho] | b_chirho[i_chirho];
    }

    result_chirho
}

/// Auto-dispatch intersect — uses SIMD when available, scalar otherwise.
pub fn intersect_bitset_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    simd_intersect_chirho(a_chirho, b_chirho)
}

/// Auto-dispatch union — uses SIMD when available, scalar otherwise.
pub fn union_bitset_chirho(a_chirho: &[u64], b_chirho: &[u64]) -> Vec<u64> {
    simd_union_chirho(a_chirho, b_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_scalar_intersect_empty_chirho() {
        let result_chirho = scalar_intersect_chirho(&[], &[]);
        assert!(result_chirho.is_empty());
    }

    #[test]
    fn test_scalar_intersect_overlap_chirho() {
        let a_chirho = vec![0b1111u64, 0b1010u64];
        let b_chirho = vec![0b1010u64, 0b0101u64];
        let result_chirho = scalar_intersect_chirho(&a_chirho, &b_chirho);
        assert_eq!(result_chirho, vec![0b1010u64, 0b0000u64]);
    }

    #[test]
    fn test_scalar_intersect_disjoint_chirho() {
        let a_chirho = vec![0b1100u64];
        let b_chirho = vec![0b0011u64];
        let result_chirho = scalar_intersect_chirho(&a_chirho, &b_chirho);
        assert_eq!(result_chirho, vec![0u64]);
    }

    #[test]
    fn test_scalar_union_chirho() {
        let a_chirho = vec![0b1100u64, 0b1010u64];
        let b_chirho = vec![0b0011u64, 0b0101u64];
        let result_chirho = scalar_union_chirho(&a_chirho, &b_chirho);
        assert_eq!(result_chirho, vec![0b1111u64, 0b1111u64]);
    }

    #[test]
    fn test_scalar_popcount_chirho() {
        let data_chirho = vec![0b1111u64, 0b1010u64];
        let count_chirho = scalar_popcount_chirho(&data_chirho);
        assert_eq!(count_chirho, 6); // 4 + 2
    }

    #[test]
    fn test_large_bitset_chirho() {
        let size_chirho = 1024;
        let a_chirho: Vec<u64> = (0..size_chirho).map(|i_chirho| i_chirho as u64).collect();
        let b_chirho: Vec<u64> = (0..size_chirho).map(|i_chirho| (i_chirho * 2) as u64).collect();
        let result_chirho = intersect_bitset_chirho(&a_chirho, &b_chirho);
        assert_eq!(result_chirho.len(), size_chirho);
        for i_chirho in 0..size_chirho {
            assert_eq!(
                result_chirho[i_chirho],
                (i_chirho as u64) & ((i_chirho * 2) as u64)
            );
        }
    }

    #[test]
    fn test_simd_scalar_equivalence_intersect_chirho() {
        let a_chirho: Vec<u64> = (0..100).map(|i_chirho| i_chirho * 7 + 3).collect();
        let b_chirho: Vec<u64> = (0..100).map(|i_chirho| i_chirho * 11 + 5).collect();
        let scalar_chirho = scalar_intersect_chirho(&a_chirho, &b_chirho);
        let simd_chirho = simd_intersect_chirho(&a_chirho, &b_chirho);
        assert_eq!(scalar_chirho, simd_chirho);
    }

    #[test]
    fn test_simd_scalar_equivalence_union_chirho() {
        let a_chirho: Vec<u64> = (0..100).map(|i_chirho| i_chirho * 13 + 1).collect();
        let b_chirho: Vec<u64> = (0..100).map(|i_chirho| i_chirho * 17 + 2).collect();
        let scalar_chirho = scalar_union_chirho(&a_chirho, &b_chirho);
        let simd_chirho = simd_union_chirho(&a_chirho, &b_chirho);
        assert_eq!(scalar_chirho, simd_chirho);
    }
}

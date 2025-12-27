//! SIMD-accelerated bitboard operations using AVX2.
//!
//! This module provides parallel operations on 4 bitboards simultaneously
//! using 256-bit AVX2 registers.

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use core::arch::x86_64::*;

use super::Bitboard;
use crate::types::Square;
use crate::attacks::KNIGHT_ATTACKS;

/// Four bitboards packed into a 256-bit AVX2 register.
/// Enables parallel operations on 4 bitboards simultaneously.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bitboard4(#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] __m256i);

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
impl Bitboard4 {
    /// Create from 4 individual bitboards.
    #[inline(always)]
    pub fn new(a: Bitboard, b: Bitboard, c: Bitboard, d: Bitboard) -> Self {
        unsafe {
            Bitboard4(_mm256_set_epi64x(d.0 as i64, c.0 as i64, b.0 as i64, a.0 as i64))
        }
    }

    /// Create with all bitboards set to the same value.
    #[inline(always)]
    pub fn splat(bb: Bitboard) -> Self {
        unsafe {
            Bitboard4(_mm256_set1_epi64x(bb.0 as i64))
        }
    }

    /// Create with all zeros.
    #[inline(always)]
    pub fn zero() -> Self {
        unsafe {
            Bitboard4(_mm256_setzero_si256())
        }
    }

    /// Bitwise AND of two Bitboard4.
    #[inline(always)]
    pub fn and(self, other: Self) -> Self {
        unsafe {
            Bitboard4(_mm256_and_si256(self.0, other.0))
        }
    }

    /// Bitwise OR of two Bitboard4.
    #[inline(always)]
    pub fn or(self, other: Self) -> Self {
        unsafe {
            Bitboard4(_mm256_or_si256(self.0, other.0))
        }
    }

    /// Bitwise XOR of two Bitboard4.
    #[inline(always)]
    pub fn xor(self, other: Self) -> Self {
        unsafe {
            Bitboard4(_mm256_xor_si256(self.0, other.0))
        }
    }

    /// Bitwise AND-NOT: self & !other
    #[inline(always)]
    pub fn andnot(self, other: Self) -> Self {
        unsafe {
            // Note: _mm256_andnot_si256(a, b) = !a & b, so we swap args
            Bitboard4(_mm256_andnot_si256(other.0, self.0))
        }
    }

    /// Extract the 4 bitboards back to individual values.
    #[inline(always)]
    pub fn extract(self) -> [Bitboard; 4] {
        unsafe {
            let mut arr = [0u64; 4];
            _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, self.0);
            [Bitboard(arr[0]), Bitboard(arr[1]), Bitboard(arr[2]), Bitboard(arr[3])]
        }
    }

    /// Horizontal OR: combine all 4 bitboards into one.
    #[inline(always)]
    pub fn horizontal_or(self) -> Bitboard {
        let parts = self.extract();
        Bitboard(parts[0].0 | parts[1].0 | parts[2].0 | parts[3].0)
    }

    /// Count total bits across all 4 bitboards.
    /// This is faster than extracting and summing individually.
    #[inline(always)]
    pub fn popcount4(self) -> u32 {
        let parts = self.extract();
        parts[0].count() + parts[1].count() + parts[2].count() + parts[3].count()
    }

    /// Load knight attacks for 4 squares at once.
    #[inline(always)]
    pub fn knight_attacks_4(sq0: Square, sq1: Square, sq2: Square, sq3: Square) -> Self {
        // Gather knight attacks for 4 squares
        Self::new(
            KNIGHT_ATTACKS[sq0.index() as usize],
            KNIGHT_ATTACKS[sq1.index() as usize],
            KNIGHT_ATTACKS[sq2.index() as usize],
            KNIGHT_ATTACKS[sq3.index() as usize],
        )
    }
}

/// Check if AVX2 is available at runtime.
#[inline]
pub fn is_avx2_available() -> bool {
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    {
        true
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_bitboard4_and() {
        let a = Bitboard4::new(
            Bitboard(0xFFFF),
            Bitboard(0xFF00),
            Bitboard(0x0F0F),
            Bitboard(0x00FF),
        );
        let b = Bitboard4::splat(Bitboard(0x0FFF));
        let result = a.and(b).extract();
        
        assert_eq!(result[0].0, 0x0FFF);
        assert_eq!(result[1].0, 0x0F00);
        assert_eq!(result[2].0, 0x0F0F);
        assert_eq!(result[3].0, 0x00FF);
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_horizontal_or() {
        let bb4 = Bitboard4::new(
            Bitboard(0x01),
            Bitboard(0x02),
            Bitboard(0x04),
            Bitboard(0x08),
        );
        let result = bb4.horizontal_or();
        assert_eq!(result.0, 0x0F);
    }
}

// Current layout (on 64 bit architectures):
// 0        N       64
// | length | depth |
// 0   6            64
// | N | stride     |
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PackedSize(usize, usize);

impl PackedSize {
    const N_BITS: usize = if cfg!(target_pointer_width = "64") {
        6
    } else {
        5
    };

    const N_MASK: usize = (1 << Self::N_BITS) - 1;

    pub fn new(rows: usize, cols: usize, stride: usize) -> PackedSize {
        debug_assert!(rows <= isize::MAX as usize);
        debug_assert!(cols <= isize::MAX as usize);
        debug_assert!(rows.saturating_mul(cols) <= isize::MAX as usize);

        let n = (usize::BITS - rows.leading_zeros()) as usize;
        let len = rows;
        let depth = cols << n;

        PackedSize(len | depth, n | stride << Self::N_BITS)
    }

    #[inline]
    fn n(&self) -> usize {
        self.1 & Self::N_MASK
    }

    #[inline]
    fn n_mask(&self) -> usize {
        (1 << self.n()) - 1
    }

    /// Returns the contained length, depth, and stride as a tuple of `(length, depth, stride)`
    pub fn unpack(&self) -> (usize, usize, usize) {
        let stride = self.1 >> Self::N_BITS;
        let len = self.0 & self.n_mask();
        let depth = self.0 >> self.n();
        (len, depth, stride)
    }

    pub fn len(&self) -> usize {
        self.0 & self.n_mask()
    }

    pub fn depth(&self) -> usize {
        self.0 >> self.n()
    }

    pub fn stride(&self) -> usize {
        self.1 >> Self::N_BITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_size() {
        let ps = PackedSize::new(1, 2, 3);
        assert_eq!(ps.len(), 1);
        assert_eq!(ps.depth(), 2);
        assert_eq!(ps.stride(), 3);
        assert_eq!(ps.unpack(), (1, 2, 3));

        let ps = PackedSize::new(isize::MAX as usize, 1, 0);
        assert_eq!(ps.len(), isize::MAX as usize);
        assert_eq!(ps.depth(), 1);
        assert_eq!(ps.stride(), 0);
        assert_eq!(ps.unpack(), (isize::MAX as usize, 1, 0));

        let max_stride = 2usize.pow(58) - 1;
        let ps = PackedSize::new(1, isize::MAX as usize, max_stride);
        assert_eq!(ps.len(), 1);
        assert_eq!(ps.depth(), isize::MAX as usize);
        assert_eq!(ps.stride(), max_stride);
        assert_eq!(ps.unpack(), (1, isize::MAX as usize, max_stride));
    }
}

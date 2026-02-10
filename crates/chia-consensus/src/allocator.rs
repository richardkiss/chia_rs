use clvmr::allocator::Allocator;
use clvmr::chia_dialect::ClvmFlags;

/// Construct an Allocator with a heap-size limit or not, depending on the flags.
pub fn make_allocator(flags: u32) -> Allocator {
    if ClvmFlags::from_bits_truncate(flags).contains(ClvmFlags::LIMIT_HEAP) {
        Allocator::new_limited(500_000_000)
    } else {
        Allocator::new_limited(u32::MAX as usize)
    }
}

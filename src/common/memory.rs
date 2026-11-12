//! Keeping Trindle's own programs small in memory.

pub fn few_heaps() {
    unsafe { libc::mallopt(libc::M_ARENA_MAX, 2) };
}

pub fn give_back() {
    unsafe { libc::malloc_trim(0) };
}

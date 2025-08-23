pub mod commands;
pub mod filesystem;
pub mod mmap;
pub mod signal;

pub(crate) fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

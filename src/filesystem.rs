use clap::ValueEnum;
use std::{fs, io};

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum Advise {
    #[value(alias = "POSIX_FADV_NORMAL")]
    Normal,
    #[value(alias = "POSIX_FADV_SEQUENTIAL")]
    Sequential,
    #[value(alias = "POSIX_FADV_RANDOM")]
    Random,
    #[value(name = "WILLNEED", alias = "POSIX_FADV_WILLNEED")]
    WillNeed,
    #[value(name = "DONTNEED", alias = "POSIX_FADV_DONTNEED")]
    DontNeed,
    #[value(name = "NOREUSE", alias = "POSIX_FADV_NOREUSE")]
    NoReuse,
}

impl std::fmt::Display for Advise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Advise::Normal => "NORMAL",
            Advise::Sequential => "SEQUENTIAL",
            Advise::Random => "RANDOM",
            Advise::WillNeed => "WILLNEED",
            Advise::DontNeed => "DONTNEED",
            Advise::NoReuse => "NOREUSE",
        };
        write!(f, "{}", s)
    }
}

#[cfg(target_os = "linux")]
pub fn fadvise(
    file: &fs::File,
    advise: Advise,
    offset: Option<u64>,
    len: Option<usize>,
) -> io::Result<()> {
    use std::os::fd::AsRawFd;

    let advise = match advise {
        Advise::Normal => libc::POSIX_FADV_NORMAL,
        Advise::Sequential => libc::POSIX_FADV_SEQUENTIAL,
        Advise::Random => libc::POSIX_FADV_RANDOM,
        Advise::WillNeed => libc::POSIX_FADV_WILLNEED,
        Advise::DontNeed => libc::POSIX_FADV_DONTNEED,
        Advise::NoReuse => libc::POSIX_FADV_NOREUSE,
    };

    let offset = offset.unwrap_or(0);
    let len = len.unwrap_or(0);

    match unsafe { libc::posix_fadvise64(file.as_raw_fd(), offset as _, len as _, advise) } {
        0 => Ok(()),
        err => Err(io::Error::from_raw_os_error(err)),
    }
}

#[cfg(not(target_os = "linux"))]
pub fn fadvise(
    _file: &fs::File,
    _advise: Advise,
    _offset: Option<u64>,
    _len: Option<usize>,
) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "fadvise is not supported on this platform",
    ))
}

#[cfg(target_os = "linux")]
pub fn fallocate(file: &fs::File, len: usize) -> io::Result<()> {
    use std::os::fd::AsRawFd;

    // TODO: support more modes
    let mode = libc::FALLOC_FL_KEEP_SIZE;
    match unsafe { libc::fallocate(file.as_raw_fd(), mode, 0, len as _) } {
        0 => Ok(()),
        err => Err(io::Error::from_raw_os_error(err)),
    }
}

#[cfg(not(target_os = "linux"))]
pub fn fallocate(_file: &fs::File, _len: usize) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "fallocate is not supported on this platform",
    ))
}

pub struct FincoreStats {
    pub file_bytes: u64,
    pub cached_bytes: u64,

    pub total_pages: u64,
    pub cached_pages: u64,

    pub pages: Vec<u8>,
}

#[cfg(target_os = "linux")]
pub fn fincore(file: &fs::File) -> io::Result<FincoreStats> {
    use crate::mmap;

    let metadata = file.metadata()?;
    let file_bytes = metadata.len();
    let page_size = crate::page_size() as u64;
    let total_pages = (file_bytes + page_size - 1) / page_size;

    let mem = mmap::Mmap::map(&file, 0, file_bytes as usize)?;
    let mut pages: Vec<u8> = vec![0; total_pages as usize];
    let ret = unsafe { libc::mincore(mem.as_ptr() as _, file_bytes as _, pages.as_mut_ptr() as _) };
    if ret != 0 {
        return Err(io::Error::last_os_error());
    }

    let cached_pages = pages.iter().filter(|&&b| (b & 1) == 1).count() as u64;
    let cached_bytes = cached_pages * page_size;

    Ok(FincoreStats {
        file_bytes,
        cached_bytes,
        total_pages,
        cached_pages,
        pages,
    })
}

#[cfg(not(target_os = "linux"))]
pub fn fincore(_file: &fs::File) -> io::Result<FincoreStats> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "fincore is not supported on this platform",
    ))
}

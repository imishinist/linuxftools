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

use std::fs::File;
use std::io;
use std::os::fd::RawFd;

pub struct Mmap {
    inner: MmapInner,
}

impl Mmap {
    pub fn map(file: &File, offset: usize, length: usize) -> io::Result<Self> {
        let inner = MmapInner::map(file, offset, length)?;
        Ok(Mmap { inner })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner.ptr as *const u8
    }

    pub fn len(&self) -> usize {
        self.inner.len
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

struct MmapInner {
    ptr: *mut libc::c_void,
    len: usize,
}

impl MmapInner {
    fn new(
        length: usize,
        proto: libc::c_int,
        flags: libc::c_int,
        fd: RawFd,
        offset: usize,
    ) -> io::Result<Self> {
        let page_size = crate::page_size();
        let align = offset % page_size;
        let aligned_offset = offset - align;
        let aligned_length = length + align;

        if aligned_length == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "memory map must have a non-zero length",
            ));
        }

        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                aligned_length,
                proto,
                flags,
                fd,
                aligned_offset as libc::off_t,
            )
        };
        if ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        Ok(MmapInner { ptr, len: length })
    }

    fn map(file: &File, offset: usize, length: usize) -> io::Result<Self> {
        use std::os::fd::AsRawFd;
        Self::new(
            length,
            libc::PROT_NONE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            offset,
        )
    }
}

impl Drop for MmapInner {
    fn drop(&mut self) {
        let alignment = self.ptr as usize % crate::page_size();
        unsafe {
            assert_eq!(
                libc::munmap(
                    self.ptr.offset(-(alignment as isize)),
                    (self.len + alignment) as libc::size_t
                ),
                0,
                "unable to unmap mmap: {}",
                io::Error::last_os_error()
            );
        }
    }
}

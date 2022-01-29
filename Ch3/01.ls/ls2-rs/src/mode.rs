use libc::{S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFSOCK};

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

use libc::mode_t;

pub fn is_dir(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFDIR
}

pub fn is_chr(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFCHR
}

pub fn is_blk(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFBLK
}

pub fn is_lnk(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFLNK
}

pub fn is_sock(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFSOCK
}

pub fn is_fifo(mode: mode_t) -> bool {
    mode & S_IFMT == S_IFIFO
}

pub fn irusr(mode: mode_t) -> bool {
    mode & S_IRUSR != 0
}

pub fn iwusr(mode: mode_t) -> bool {
    mode & S_IWUSR != 0
}

pub fn ixusr(mode: mode_t) -> bool {
    mode & S_IXUSR != 0
}

pub fn irgrp(mode: mode_t) -> bool {
    mode & S_IRGRP != 0
}

pub fn iwgrp(mode: mode_t) -> bool {
    mode & S_IWGRP != 0
}

pub fn ixgrp(mode: mode_t) -> bool {
    mode & S_IXGRP != 0
}

pub fn iroth(mode: mode_t) -> bool {
    mode & S_IROTH != 0
}

pub fn iwoth(mode: mode_t) -> bool {
    mode & S_IWOTH != 0
}

pub fn ixoth(mode: mode_t) -> bool {
    mode & S_IXOTH != 0
}

use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

pub fn get_mode(meta: &Metadata) -> String {
    #[cfg(unix)]
    {
        format_mode(meta.mode())
    }

    #[cfg(windows)]
    {
        format_mode(meta.file_attributes())
    }
}

pub fn format_mode(mode: u32) -> String {
    #[cfg(unix)]
    {
        get_mode_unix(mode)
    }

    #[cfg(windows)]
    {
        get_mode_windows(mode)
    }
}

#[cfg(unix)]
fn get_mode_unix(mode: u32) -> String {
    let mut s = String::new();

    // user

    if mode_user_read(mode) {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode_user_write(mode) {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode_user_exec(mode) {
        s.push('x')
    } else {
        s.push('-');
    }

    // group

    if mode_group_read(mode) {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode_group_write(mode) {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode_group_exec(mode) {
        s.push('x')
    } else {
        s.push('-');
    }

    // other

    if mode_other_read(mode) {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode_other_write(mode) {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode_other_exec(mode) {
        s.push('x')
    } else {
        s.push('-');
    }

    s
}

#[allow(unused)]
pub fn get_mode_from_boxed_unix_int(meta: &Metadata) -> Option<u32> {
    #[cfg(unix)]
    {
        Some(meta.mode())
    }

    #[cfg(not(unix))]
    {
        None
    }
}

pub fn user_read(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_user_read(mode),
        None => false
    }
}

pub fn mode_user_read(mode: u32) -> bool {
    mode & S_IRUSR == S_IRUSR
}

pub fn user_write(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_user_write(mode),
        None => false
    }
}

pub fn mode_user_write(mode: u32) -> bool {
    mode & S_IWUSR == S_IWUSR
}

pub fn user_exec(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_user_exec(mode),
        None => false
    }
}

pub fn mode_user_exec(mode: u32) -> bool {
    mode & S_IXUSR == S_IXUSR
}

pub fn user_all(meta: &Metadata) -> bool {
    user_read(meta) && user_write(meta) && user_exec(meta)
}

pub fn mode_user_all(mode: u32) -> bool {
    mode_user_read(mode) && mode_user_write(mode) && mode_user_exec(mode)
}

pub fn group_read(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_group_read(mode),
        None => false
    }
}

pub fn mode_group_read(mode: u32) -> bool {
    mode & S_IRGRP == S_IRGRP
}

pub fn group_write(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_group_write(mode),
        None => false
    }
}

pub fn mode_group_write(mode: u32) -> bool {
    mode & S_IWGRP == S_IWGRP
}

pub fn group_exec(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_group_exec(mode),
        None => false
    }
}

pub fn mode_group_exec(mode: u32) -> bool {
    mode & S_IXGRP == S_IXGRP
}

pub fn group_all(meta: &Metadata) -> bool {
    group_read(meta) && group_write(meta) && group_exec(meta)
}

pub fn mode_group_all(mode: u32) -> bool {
    mode_group_read(mode) && mode_group_write(mode) && mode_group_exec(mode)
}

pub fn other_read(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_other_read(mode),
        None => false
    }
}

pub fn mode_other_read(mode: u32) -> bool {
    mode & S_IROTH == S_IROTH
}

pub fn other_write(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_other_write(mode),
        None => false
    }
}

pub fn mode_other_write(mode: u32) -> bool {
    mode & S_IWOTH == S_IWOTH
}

pub fn other_exec(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_other_exec(mode),
        None => false
    }
}

pub fn mode_other_exec(mode: u32) -> bool {
    mode & S_IXOTH == S_IXOTH
}

pub fn other_all(meta: &Metadata) -> bool {
    other_read(meta) && other_write(meta) && other_exec(meta)
}

pub fn mode_other_all(mode: u32) -> bool {
    mode_other_read(mode) && mode_other_write(mode) && mode_other_exec(mode)
}

pub fn suid_bit_set(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_suid(mode),
        None => false
    }
}

pub fn mode_suid(mode: u32) -> bool {
    mode & S_IFMT & S_ISUID == S_IFMT & S_ISUID
}

pub fn sgid_bit_set(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_sgid(mode),
        None => false
    }
}

pub fn mode_sgid(mode: u32) -> bool {
    mode & S_IFMT & S_ISGID == S_IFMT & S_ISGID
}

pub fn is_pipe(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_is_pipe(mode),
        None => false
    }
}

pub fn mode_is_pipe(mode: u32) -> bool {
    mode & S_IFMT & S_IFIFO == S_IFMT & S_IFIFO
}

pub fn is_char_device(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_is_char_device(mode),
        None => false
    }
}

pub fn mode_is_char_device(mode: u32) -> bool {
    mode & S_IFMT & S_IFCHR == S_IFMT & S_IFCHR
}

pub fn is_block_device(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_is_block_device(mode),
        None => false
    }
}

pub fn mode_is_block_device(mode: u32) -> bool {
    mode & S_IFMT & S_IFBLK == S_IFMT & S_IFBLK
}

pub fn is_socket(meta: &Metadata) -> bool {
    match get_mode_from_boxed_unix_int(meta) {
        Some(mode) => mode_is_socket(mode),
        None => false
    }
}

pub fn mode_is_socket(mode: u32) -> bool {
    mode & S_IFMT & S_IFSOCK == S_IFMT & S_IFSOCK
}

const S_IRUSR: u32 = 400;
const S_IWUSR: u32 = 200;
const S_IXUSR: u32 = 100;

const S_IRGRP: u32 = 40;
const S_IWGRP: u32 = 20;
const S_IXGRP: u32 = 10;

const S_IROTH: u32 = 4;
const S_IWOTH: u32 = 2;
const S_IXOTH: u32 = 1;

const S_ISUID: u32 = 4000;
const S_ISGID: u32 = 2000;
#[allow(unused)]
const S_ISVTX: u32 = 1000;

const S_IFMT: u32 = 170000;
const S_IFIFO: u32 = 10000;
const S_IFCHR: u32 = 20000;
const S_IFBLK: u32 = 60000;
const S_IFSOCK: u32 = 140000;

#[cfg(windows)]
fn get_mode_windows(mode: u32) -> String {
    const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
    const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x800;
    const FILE_ATTRIBUTE_DEVICE: u32 = 0x40;
    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
    const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x4000;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 0x8000;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
    const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x2000;
    const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x20000;
    const FILE_ATTRIBUTE_OFFLINE: u32 = 0x1000;
    const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
    const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x400000;
    const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x40000;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x200;
    const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
    const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x100;
    const FILE_ATTRIBUTE_VIRTUAL: u32 = 0x10000;

    let mut v = vec![];

    if mode & FILE_ATTRIBUTE_ARCHIVE == FILE_ATTRIBUTE_ARCHIVE {
        v.push("Archive");
    }

    if mode & FILE_ATTRIBUTE_COMPRESSED == FILE_ATTRIBUTE_COMPRESSED {
        v.push("Compressed");
    }

    if mode & FILE_ATTRIBUTE_DEVICE == FILE_ATTRIBUTE_DEVICE {
        v.push("Device");
    }

    if mode & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY {
        v.push("Directory");
    }

    if mode & FILE_ATTRIBUTE_ENCRYPTED == FILE_ATTRIBUTE_ENCRYPTED {
        v.push("Encrypted");
    }

    if mode & FILE_ATTRIBUTE_HIDDEN == FILE_ATTRIBUTE_HIDDEN {
        v.push("Hidden");
    }

    if mode & FILE_ATTRIBUTE_INTEGRITY_STREAM == FILE_ATTRIBUTE_INTEGRITY_STREAM {
        v.push("Integrity Stream");
    }

    if mode & FILE_ATTRIBUTE_NORMAL == FILE_ATTRIBUTE_NORMAL {
        v.push("Normal");
    }

    if mode & FILE_ATTRIBUTE_NOT_CONTENT_INDEXED == FILE_ATTRIBUTE_NOT_CONTENT_INDEXED {
        v.push("Not indexed");
    }

    if mode & FILE_ATTRIBUTE_NO_SCRUB_DATA == FILE_ATTRIBUTE_NO_SCRUB_DATA {
        v.push("No scrub data");
    }

    if mode & FILE_ATTRIBUTE_OFFLINE == FILE_ATTRIBUTE_OFFLINE {
        v.push("Offline");
    }

    if mode & FILE_ATTRIBUTE_READONLY == FILE_ATTRIBUTE_READONLY {
        v.push("Readonly");
    }

    if mode & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS == FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS {
        v.push("Recall on data access");
    }

    if mode & FILE_ATTRIBUTE_RECALL_ON_OPEN == FILE_ATTRIBUTE_RECALL_ON_OPEN {
        v.push("Recall on open");
    }

    if mode & FILE_ATTRIBUTE_REPARSE_POINT == FILE_ATTRIBUTE_REPARSE_POINT {
        v.push("Reparse point");
    }

    if mode & FILE_ATTRIBUTE_SPARSE_FILE == FILE_ATTRIBUTE_SPARSE_FILE {
        v.push("Sparse");
    }

    if mode & FILE_ATTRIBUTE_SYSTEM == FILE_ATTRIBUTE_SYSTEM {
        v.push("System");
    }

    if mode & FILE_ATTRIBUTE_TEMPORARY == FILE_ATTRIBUTE_TEMPORARY {
        v.push("Temporary");
    }

    if mode & FILE_ATTRIBUTE_VIRTUAL == FILE_ATTRIBUTE_VIRTUAL {
        v.push("Virtual");
    }

    v.join(", ")
}

#[allow(unused)]
pub fn get_uid(meta: &Metadata) -> Option<u32> {
    #[cfg(unix)]
    {
        Some(meta.uid())
    }

    #[cfg(not(unix))]
    {
        None
    }
}

#[allow(unused)]
pub fn get_gid(meta: &Metadata) -> Option<u32> {
    #[cfg(unix)]
    {
        Some(meta.gid())
    }

    #[cfg(not(unix))]
    {
        None
    }
}
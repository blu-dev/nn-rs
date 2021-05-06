use super::Result as NxResult;
use super::get_rust_result;
use super::c_str;

use libc::*;

bitflags! {
    #[repr(C)]
    pub struct WriteOptions : u32 {
        const FLUSH = 0b0000_0001;
    }
}

bitflags! {
    #[repr(C)]
    pub struct OpenMode : u32 {
        const READ         = 0b0000_0001;
        const WRITE        = 0b0000_0010;
        const ALLOW_APPEND = 0b0000_0100;
    }
}

bitflags! {
    #[repr(C)]
    pub struct OpenDirectoryMode : u32 {
        const DIRECTORY = 0b0000_0001;
        const FILE      = 0b0000_0010;
        const ALL       = 0b0000_0011;
    }
}

#[repr(i8)]
#[derive(Copy, Clone)]
pub enum DirectoryEntryType {
    Directory,
    File
}

#[repr(C)]
pub struct DirectoryEntry {
    pub name: [c_char; ENTRY_NAME_BYTE_LENGTH_MAX + 1],
    _x301: [u8; 3],
    pub entry_type: DirectoryEntryType,
    _x305: u8,
    pub size: isize,
}

impl Clone for DirectoryEntry {
    fn clone(&self) -> Self {
        let mut ret = Self {
            name: [0; ENTRY_NAME_BYTE_LENGTH_MAX + 1],
            _x301: [0; 3],
            entry_type: self.entry_type,
            _x305: 0,
            size: self.size
        };
        unsafe {
            libc::memcpy(ret.name.as_mut_ptr() as _, self.name.as_ptr() as _, self.name.len());
        }
        ret
    }
}

pub const ENTRY_NAME_BYTE_LENGTH_MAX: usize = 768;
pub const MOUNT_NAME_LENGTH_MAX: usize      = 15;

pub type AllocatorFunction = extern "C" fn(usize) -> *mut c_void;
pub type DeallocatorFunction = extern "C" fn(*mut c_void, usize);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FileHandle(pub u64);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct DirectoryHandle(pub u64);

pub mod fs_impl {
    use super::super::Result;
    use libc::*;
    use super::{FileHandle, DirectoryEntryType};

    extern "C" {
        // General
        #[link_name = "\u{1}_ZN2nn2fs12SetAllocatorEPFPvmEPFvS1_mE"]
        pub fn SetAllocator(
            allocator: super::AllocatorFunction,
            deallocator: super::DeallocatorFunction
        );

        // Mounting
        #[link_name = "\u{1}_ZN2nn2fs21MountSaveDataForDebugEPKc"]
        pub fn MountSaveDataForDebug(
            name: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs19MountSdCardForDebugEPKc"]
        pub fn MountSdCardForDebug(
            name: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs22QueryMountRomCacheSizeEPm"]
        pub fn QueryMountRomCacheSize(
            out: *mut usize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs8MountRomEPKcPvm"]
        pub fn MountRom(
            name: *const c_char,
            buffer: *mut c_void,
            buffer_size: usize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs7UnmountEPKc"]
        pub fn Unmount(
            name: *const c_char
        );

        // File Operations
        #[link_name = "\u{1}_ZN2nn2fs10CreateFileEPKcl"]
        pub fn CreateFile(
            path: *const c_char,
            size: isize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs10DeleteFileEPKc"]
        pub fn DeleteFile(
            path: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs10RenameFileEPKcS2_"]
        pub fn RenameFile(
            current: *const c_char,
            new: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci"]
        pub fn OpenFile(
            out: *mut FileHandle,
            path: *const c_char,
            mode: super::OpenMode
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs9CloseFileENS0_10FileHandleE"]
        pub fn CloseFile(
            handle: FileHandle
        );

        #[link_name = "\u{1}_ZN2nn2fs8ReadFileENS0_10FileHandleElPvm"]
        pub fn ReadFileFixedSize(
            handle: FileHandle,
            offset: isize,
            buffer: *mut c_void,
            buffer_size: usize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs8ReadFileEPmNS0_10FileHandleElPvm"]
        pub fn ReadFile(
            read_size: *mut usize,
            handle: FileHandle,
            offset: isize,
            buffer: *mut c_void,
            buffer_size: usize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs9WriteFileENS0_10FileHandleElPKvmRKNS0_11WriteOptionE"]
        pub fn WriteFile(
            handle: FileHandle,
            offset: isize,
            buffer: *const c_void,
            buffer_size: usize,
            options: *const super::WriteOptions
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs9FlushFileENS0_10FileHandleE"]
        pub fn FlushFile(
            handle: FileHandle
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs11SetFileSizeENS0_10FileHandleEl"]
        pub fn SetFileSize(
            handle: FileHandle,
            size: isize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs11GetFileSizeEPlNS0_10FileHandleE"]
        pub fn GetFileSize(
            out: *mut isize,
            handle: FileHandle
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs15GetFileOpenModeENS0_10FileHandleE"]
        pub fn GetFileOpenMode(
            handle: FileHandle
        ) -> super::OpenMode;

        // Directory Utils

        #[link_name = "\u{1}_ZN2nn2fs15CreateDirectoryEPKc"]
        pub fn CreateDirectory(
            path: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs15DeleteDirectoryEPKc"]
        pub fn DeleteDirectory(
            path: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs26DeleteDirectoryRecursivelyEPKc"]
        pub fn DeleteDirectoryRecursively(
            path: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs15RenameDirectoryEPKcS2_"]
        pub fn RenameDirectory(
            current: *const c_char,
            new: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs12GetEntryTypeEPNS0_18DirectoryEntryTypeEPKc"]
        pub fn GetEntryType(
            out: *mut DirectoryEntryType,
            path: *const c_char
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs13ReadDirectoryEPlPNS0_14DirectoryEntryENS0_15DirectoryHandleEl"]
        pub fn ReadDirectory(
            stored_entries: *mut isize,
            entry_buffer: *mut super::DirectoryEntry,
            handle: super::DirectoryHandle,
            buffer_count: isize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs22GetDirectoryEntryCountEPlNS0_15DirectoryHandleE"]
        pub fn GetDirectoryEntryCount(
            out_count: *mut isize,
            handle: super::DirectoryHandle
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2fs14CloseDirectoryENS0_15DirectoryHandleE"]
        pub fn CloseDirectory(
            handle: super::DirectoryHandle
        );

        #[link_name = "\u{1}_ZN2nn2fs13OpenDirectoryEPNS0_15DirectoryHandleEPKci"]
        pub fn OpenDirectory(
            out_handle: *mut super::DirectoryHandle,
            path: *const c_char,
            mode: super::OpenDirectoryMode
        ) -> Result;
    }
}

#[dev_inline]
pub fn set_alloc_funcs(allocator: AllocatorFunction, deallocator: DeallocatorFunction) {
    unsafe {
        fs_impl::SetAllocator(allocator, deallocator)
    }
}

#[dev_inline]
pub fn mount_save_data<S: AsRef<str>>(name: S) -> Result<(), NxResult> {
    unsafe {
        let name = name.as_ref();
        let result = fs_impl::MountSaveDataForDebug(c_str!(name));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn mount_sd_card<S: AsRef<str>>(name: S) -> Result<(), NxResult> {
    unsafe {
        let name = name.as_ref();
        let result = fs_impl::MountSdCardForDebug(c_str!(name));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn mount_rom<B: AsMut<[u8]>, S: AsRef<str>>(name: S, mut buffer: B) -> Result<(), NxResult> {
    unsafe {
        let name = name.as_ref();
        let buffer = buffer.as_mut();
        let result = fs_impl::MountRom(c_str!(name), buffer.as_mut_ptr() as _, buffer.len());
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn query_rom_cache_size() -> Result<usize, NxResult> {
    unsafe {
        let mut size = 0;
        let result = fs_impl::QueryMountRomCacheSize(&mut size);
        get_rust_result!(result, size)
    }
}

#[dev_inline]
pub fn unmount<S: AsRef<str>>(name: S) {
    unsafe {
        let name = name.as_ref();
        fs_impl::Unmount(c_str!(name));
    }
}

#[dev_inline]
pub fn create_file<S: AsRef<str>>(name: S, initial_size: isize) -> Result<(), NxResult> {
    unsafe {
        let name = name.as_ref();
        let result = fs_impl::CreateFile(c_str!(name), initial_size);
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn delete_file<S: AsRef<str>>(name: S) -> Result<(), NxResult> {
    unsafe {
        let name = name.as_ref();
        let result = fs_impl::DeleteFile(c_str!(name));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn rename_file<S: AsRef<str>, R: AsRef<str>>(old: S, new: R) -> Result<(), NxResult> {
    unsafe {
        let old = old.as_ref();
        let new = new.as_ref();
        let result = fs_impl::RenameFile(c_str!(old), c_str!(new));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn open_file<S: AsRef<str>>(path: S, mode: OpenMode) -> Result<FileHandle, NxResult> {
    unsafe {
        let path = path.as_ref();
        let mut handle = FileHandle(0);
        let result = fs_impl::OpenFile(&mut handle, c_str!(path), mode);
        get_rust_result!(result, handle)
    }
}

#[dev_inline]
pub fn close_file(handle: FileHandle) {
    unsafe {
        fs_impl::CloseFile(handle)
    }
}

#[dev_inline]
pub fn read_file(handle: FileHandle, offset: isize, buffer: *mut c_void, buffer_size: usize) -> Result<usize, NxResult> {
    unsafe {
        let mut read_bytes = 0;
        let result = fs_impl::ReadFile(&mut read_bytes, handle, offset, buffer, buffer_size);
        get_rust_result!(result, read_bytes)
    }
}

#[dev_inline]
pub fn write_file(handle: FileHandle, offset: isize, buffer: *const c_void, buffer_size: usize, options: WriteOptions) -> Result<(), NxResult> {
    unsafe {
        let result = fs_impl::WriteFile(handle, offset, buffer, buffer_size, &options);
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn flush_file(handle: FileHandle) -> Result<(), NxResult> {
    unsafe {
        let result = fs_impl::FlushFile(handle);
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn resize_file(handle: FileHandle, new_size: isize) -> Result<(), NxResult> {
    unsafe {
        let result = fs_impl::SetFileSize(handle, new_size);
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn get_file_size(handle: FileHandle) -> Result<isize, NxResult> {
    unsafe {
        let mut size = 0;
        let result = fs_impl::GetFileSize(&mut size, handle);
        get_rust_result!(result, size)
    }
}

#[dev_inline]
pub fn file_open_mode(handle: FileHandle) -> OpenMode {
    unsafe {
        fs_impl::GetFileOpenMode(handle)
    }
}

#[dev_inline]
pub fn create_directory<S: AsRef<str>>(path: S) -> Result<(), NxResult> {
    unsafe {
        let path = path.as_ref();
        let result = fs_impl::CreateDirectory(c_str!(path));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn delete_directory<S: AsRef<str>>(path: S, recursive: bool) -> Result<(), NxResult> {
    unsafe {
        let path = path.as_ref();
        let result = if recursive {
            fs_impl::DeleteDirectoryRecursively(c_str!(path))
        } else {
            fs_impl::DeleteDirectory(c_str!(path))
        };
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn rename_directory<S: AsRef<str>, R: AsRef<str>>(old: S, new: S) -> Result<(), NxResult> {
    unsafe {
        let old = old.as_ref();
        let new = new.as_ref();

        let result = fs_impl::RenameDirectory(c_str!(old), c_str!(new));
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn get_entry_type<S: AsRef<str>>(path: S) -> Result<DirectoryEntryType, NxResult> {
    unsafe {
        let path = path.as_ref();

        let mut entry_type = DirectoryEntryType::File;
        let result = fs_impl::GetEntryType(&mut entry_type, c_str!(path));
        get_rust_result!(result, entry_type)
    }
}

#[dev_inline]
pub fn read_directory_entries<M: AsMut<[DirectoryEntry]>>(mut entries: M, handle: DirectoryHandle) -> Result<isize, NxResult> {
    unsafe {
        let entries = entries.as_mut();
        let mut entries_read = 0;
        let result = fs_impl::ReadDirectory(&mut entries_read, entries.as_mut_ptr(), handle, entries.len() as isize);
        get_rust_result!(result, entries_read)
    }
}

#[dev_inline]
pub fn get_directory_entry_count(handle: DirectoryHandle) -> Result<isize, NxResult> {
    unsafe {
        let mut count = 0;
        let result = fs_impl::GetDirectoryEntryCount(&mut count, handle);
        get_rust_result!(result, count)
    }
}

#[dev_inline]
pub fn close_directory(handle: DirectoryHandle) {
    unsafe {
        fs_impl::CloseDirectory(handle)
    }
}

#[dev_inline]
pub fn open_directory<S: AsRef<str>>(path: S, mode: OpenDirectoryMode) -> Result<DirectoryHandle, NxResult> {
    unsafe {
        let path = path.as_ref();
        
        let mut handle = DirectoryHandle(0);
        let result = fs_impl::OpenDirectory(&mut handle, c_str!(path), mode);
        get_rust_result!(result, handle)
    }
}
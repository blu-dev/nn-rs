use libc::*;
use super::Result as NxResult;
use super::{c_str, get_rust_result};

mod os_impl {
    use super::super::{TimeSpan, Result};
    use super::{ThreadType, ThreadFn};
    use libc::*;

    extern "C" {
        #[link_name = "\u{1}_ZN2nn2os11GetHostArgcEv"]
        pub fn GetHostArgc() -> i32;

        #[link_name = "\u{1}_ZN2nn2os11GetHostArgvEv"]
        pub fn GetHostArgv() -> *const *const c_char;

        #[link_name = "\u{1}_ZN2nn2os12CreateThreadEPNS0_10ThreadTypeEPFvPvES3_S3_mi"]
        pub fn CreateThread(
            thread: *mut ThreadType,
            entrypoint: ThreadFn,
            arg: *mut c_void,
            stack: *mut c_void,
            stack_size: usize,
            priority: i32
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2os12CreateThreadEPNS0_10ThreadTypeEPFvPvES3_S3_mii"]
        pub fn CreateThreadOnCore(
            thread: *mut ThreadType,
            entrypoint: ThreadFn,
            arg: *mut c_void,
            stack: *mut c_void,
            stack_size: usize,
            priority: i32,
            core: i32
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2os13DestroyThreadEPNS0_10ThreadTypeE"]
        pub fn DestroyThread(
            thread: *mut ThreadType
        );

        #[link_name = "\u{1}_ZN2nn2os11StartThreadEPNS0_10ThreadTypeE"]
        pub fn StartThread(
            thread: *mut ThreadType
        );

        #[link_name = "\u{1}_ZN2nn2os10WaitThreadEPNS0_10ThreadTypeE"]
        pub fn WaitThread(
            thread: *mut ThreadType
        );

        #[link_name = "\u{1}_ZN2nn2os11SleepThreadENS_8TimeSpanE"]
        pub fn SleepThread(
            time: TimeSpan
        );

        #[link_name = "\u{1}_ZN2nn2os11YieldThreadEv"]
        pub fn YieldThread();

        #[link_name = "\u{1}_ZN2nn2os16GetCurrentThreadEv"]
        pub fn GetCurrentThread() -> *mut ThreadType;

        #[link_name = "\u{1}_ZN2nn2os20ChangeThreadPriorityEPNS0_10ThreadTypeEi"]
        pub fn ChangeThreadPriority(
            thread: *mut ThreadType,
            priority: i32
        ) -> i32;

        #[link_name = "\u{1}_ZN2nn2os17GetThreadPriorityEPKNS0_10ThreadTypeE"]
        pub fn GetThreadPriority(
            thread: *const ThreadType
        ) -> i32;

        #[link_name = "\u{1}_ZN2nn2os24GetThreadCurrentPriorityEPKNS0_10ThreadTypeE"]
        pub fn GetThreadCurrentPriority(
            thread: *const ThreadType
        ) -> i32;

        #[link_name = "\u{1}_ZN2nn2os13SetThreadNameEPNS0_10ThreadTypeEPKc"]
        pub fn SetThreadName(
            thread: *mut ThreadType,
            name: *const c_char
        );

        #[link_name = "\u{1}_ZN2nn2os20SetThreadNamePointerEPNS0_10ThreadTypeEPKc"]
        pub fn SetThreadNamePointer(
            thread: *mut ThreadType,
            name: *const c_char
        );

        #[link_name = "\u{1}_ZN2nn2os20GetThreadNamePointerEPKNS0_10ThreadTypeE"]
        pub fn GetThreadNamePointer(
            thread: *const ThreadType
        ) -> *const c_char;

        #[link_name = "\u{1}_ZN2nn2os20GetCurrentCoreNumberEv"]
        pub fn GetCurrentCoreNumber() -> i32;

        // Other OS stuff

        #[link_name = "\u{1}_ZN2nn2os17SetMemoryHeapSizeEm"]
        pub fn SetMemoryHeapSize(
            size: usize
        ) -> Result;

        #[link_name = "\u{1}_ZN2nn2os19AllocateMemoryBlockEPmm"]
        pub fn AllocateMemoryBlock(
            address: *mut *mut c_void,
            size: usize
        ) -> Result;
    }
}

#[dev_inline]
#[deprecated(note = "No developer unit, host args mean nothing to us.")]
pub fn get_host_args() -> Vec<String> {
    unsafe {
        let arg_count = os_impl::GetHostArgc() as usize;
        let args = core::slice::from_raw_parts(os_impl::GetHostArgv(), arg_count);
        args.iter().map(|x| super::from_c_str(*x).unwrap()).collect()
    }
}

#[dev_inline]
pub fn set_heap_size(size: usize) -> Result<(), NxResult> {
    unsafe {
        let result = os_impl::SetMemoryHeapSize(size);
        get_rust_result!(result, ())
    }
}

#[dev_inline]
pub fn alloc_from_heap(size: usize) -> Result<*mut c_void, NxResult> {
    unsafe {
        let mut out_ptr = 0 as _;
        let result = os_impl::AllocateMemoryBlock(&mut out_ptr, size);
        get_rust_result!(result, out_ptr)
    }
}


pub type ThreadFn = extern "C" fn(*mut c_void);

#[repr(C)]
struct ThreadType {
    _x0: [u8; 0x1C0]
}

pub struct Thread(*mut ThreadType);

impl Thread {
    pub const PRIORITY_MAX: i32 = 0;
    pub const PRIORITY_MIN: i32 = 31;
    pub const PRIORITY_DEFAULT: i32 = 16;

    pub const MAX_NAME_LEN: usize = 32;

    fn free_name(&mut self) {
        unsafe {
            let range = (self.0 as usize)..(self.0 as usize + core::mem::size_of::<Self>());
            let name_ptr = os_impl::GetThreadNamePointer(self.0);
            if !range.contains(&(name_ptr as usize)) {
                os_impl::SetThreadNamePointer(self.0, core::ptr::null());
                free(name_ptr as _);
            }
        }
    }

    #[dev_inline]
    pub fn current() -> Self {
        unsafe {
            Self(os_impl::GetCurrentThread())
        }
    }

    #[dev_inline]
    pub fn yield_now() {
        unsafe {
            os_impl::YieldThread()
        }
    }

    #[dev_inline]
    pub fn sleep(time: super::TimeSpan) {
        unsafe {
            os_impl::SleepThread(time)
        }
    }

    #[dev_inline]
    pub fn get_current_core() -> i32 {
        unsafe {
            os_impl::GetCurrentCoreNumber()
        }
    }

    #[dev_inline]
    pub fn new(main: ThreadFn, arg: *mut c_void, stack: *mut c_void, stack_size: usize, priority: i32) -> Result<Self, NxResult> {
        unsafe {
            let thread = calloc(1, core::mem::size_of::<ThreadType>()) as *mut ThreadType;
            let result = os_impl::CreateThread(thread, main, arg, stack, stack_size, priority);
            if result.is_success() {
                Ok(Self(thread))
            } else {
                free(thread as _);
                Err(result)
            }
        }
    }

    #[dev_inline]
    pub fn new_on_core(main: ThreadFn, arg: *mut c_void, stack: *mut c_void, stack_size: usize, priority: i32, core: i32) -> Result<Self, NxResult> {
        unsafe {
            let thread = calloc(1, core::mem::size_of::<ThreadType>()) as *mut ThreadType;
            let result = os_impl::CreateThreadOnCore(thread, main, arg, stack, stack_size, priority, core);
            if result.is_success() {
                Ok(Self(thread))
            } else {
                free(thread as _);
                Err(result)
            }
        }
    }

    #[dev_inline]
    pub fn destroy(mut self) {
        unsafe {
            os_impl::DestroyThread(self.0);
            self.free_name();
            free(self.0 as _);
        }
    }

    #[dev_inline]
    pub fn start(&mut self) {
        unsafe {
            os_impl::StartThread(self.0)
        }
    }

    #[dev_inline]
    pub fn wait(&mut self) {
        unsafe {
            os_impl::WaitThread(self.0)
        }
    }

    #[dev_inline]
    pub fn set_priority(&mut self, priority: i32) -> i32 {
        unsafe {
            os_impl::ChangeThreadPriority(self.0, priority)
        }
    }

    #[dev_inline]
    pub fn get_original_priority(&self) -> i32 {
        unsafe {
            os_impl::GetThreadPriority(self.0)
        }
    }

    #[dev_inline]
    pub fn get_current_priority(&self) -> i32 {
        unsafe {
            os_impl::GetThreadCurrentPriority(self.0)
        }
    }

    #[dev_inline]
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        unsafe {
            self.free_name();
            let name = name.as_ref();
            if name.len() < Self::MAX_NAME_LEN {
                os_impl::SetThreadName(self.0, c_str!(name));
            } else {
                let name_ptr = calloc(1, name.len() + 1);
                memcpy(name_ptr, name.as_ptr() as _, name.len());
                os_impl::SetThreadNamePointer(self.0, name_ptr as _);
            }
        }
    }

    #[dev_inline]
    pub fn get_name(&self) -> String {
        unsafe {
            let name_ptr = os_impl::GetThreadNamePointer(self.0);
            match super::from_c_str(name_ptr) {
                Ok(v) => v,
                Err(_) => String::from("")
            }
        }
    }
}
use libc::*;

mod standard_allocator_impl {
    use libc::*;
    use super::{AllocatorHash, AllocatorWalkCallback, StandardAllocator};

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator8AllocateEmm"]
        pub fn AlignedAllocate(
            this: *mut StandardAllocator,
            size: usize,
            alignment: usize
        ) -> *mut c_void;

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator8AllocateEm"]
        pub fn Allocate(
            this: *mut StandardAllocator,
            size: usize
        ) -> *mut c_void;

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator21CleanUpManagementAreaEv"]
        pub fn CleanUpManagementArea(
            this: *const StandardAllocator
        );

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator16ClearThreadCacheEv"]
        pub fn ClearThreadCache(
            this: *const StandardAllocator
        );

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator4DumpEv"]
        pub fn Dump(
            this: *const StandardAllocator
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator8FinalizeEv"]
        pub fn Finalize(
            this: *mut StandardAllocator
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator4FreeEPv"]
        pub fn Free(
            this: *mut StandardAllocator,
            address: *mut c_void
        );

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator18GetAllocatableSizeEv"]
        pub fn GetAllocatableSize(
            this: *const StandardAllocator
        ) -> usize;

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator9GetSizeOfEPKv"]
        pub fn GetSizeOf(
            this: *const StandardAllocator,
            address: *const c_void
        ) -> usize;

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator16GetTotalFreeSizeEv"]
        pub fn GetTotalFreeSize(
            this: *const StandardAllocator
        ) -> usize;

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator4HashEv"]
        pub fn Hash(
            this: *const StandardAllocator
        ) -> AllocatorHash;

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator10InitializeEPvm"]
        pub fn Initialize(
            this: *mut StandardAllocator,
            memory: *mut c_void,
            size: usize
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator10InitializeEPvmb"]
        pub fn InitializeCached(
            this: *mut StandardAllocator,
            memory: *mut c_void,
            size: usize,
            is_cache_enable: bool
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocator10ReallocateEPvm"]
        pub fn Reallocate(
            this: *mut StandardAllocator,
            address: *mut c_void,
            new_size: usize
        ) -> *mut c_void;

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocatorC1Ev"]
        pub fn Constructor(
            this: *mut StandardAllocator
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocatorC1EPvm"]
        pub fn Constructor_Initialize(
            this: *mut StandardAllocator,
            memory: *mut c_void,
            size: usize
        );

        #[link_name = "\u{1}_ZN2nn3mem17StandardAllocatorC1EPvmb"]
        pub fn Constructor_InitializeCached(
            this: *mut StandardAllocator,
            memory: *mut c_void,
            size: usize,
            is_cache_enable: bool
        );

        #[link_name = "\u{1}_ZNK2nn3mem17StandardAllocator19WalkAllocatedBlocksEPFiPvmS2_ES2_"]
        pub fn WalkAllocatedBlocks(
            this: *const StandardAllocator,
            callback: AllocatorWalkCallback,
            user_data: *mut c_void
        );
    }
}

pub type AllocatorWalkCallback = extern "C" fn(address: *mut c_void, size: usize, user_data: *mut c_void) -> bool;

#[repr(C)]
pub struct AllocatorHash {
    pub regions: usize,
    pub total_size: usize,
    pub hash: usize
}

#[repr(C)]
pub struct StandardAllocator {
    _x0: [u8; 0x38]
}

impl StandardAllocator {
    pub const fn default() -> Self {
        Self {
            _x0: [0; 0x38]
        }
    }

    #[dev_inline]
    pub fn new() -> Self {
        unsafe {
            let mut new = std::mem::MaybeUninit::uninit();
            standard_allocator_impl::Constructor(new.as_mut_ptr());
            new.assume_init()
        }
    }

    #[dev_inline]
    pub fn new_init(memory: *mut c_void, size: usize) -> Self {
        unsafe {
            let mut new = std::mem::MaybeUninit::uninit();
            standard_allocator_impl::Constructor_Initialize(new.as_mut_ptr(), memory, size);
            new.assume_init()
        }
    }

    #[dev_inline]
    pub fn new_init_cache(memory: *mut c_void, size: usize, is_cache_enable: bool) -> Self {
        unsafe {
            let mut new = std::mem::MaybeUninit::uninit();
            standard_allocator_impl::Constructor_InitializeCached(new.as_mut_ptr(), memory, size, is_cache_enable);
            new.assume_init()
        }
    }

    #[dev_inline]
    pub fn init(&mut self, memory: *mut c_void, size: usize) {
        unsafe {
            standard_allocator_impl::Initialize(self, memory, size)
        }
    }

    #[dev_inline]
    pub fn init_enable_cache(&mut self, memory: *mut c_void, size: usize, is_cache_enable: bool) {
        unsafe {
            standard_allocator_impl::InitializeCached(self, memory, size, is_cache_enable)
        }
    }

    #[dev_inline]
    pub fn alloc(&mut self, size: usize) -> *mut c_void {
        unsafe {
            standard_allocator_impl::Allocate(self, size)
        }
    }

    #[dev_inline]
    pub fn alloc_aligned(&mut self, size: usize, alignment: usize) -> *mut c_void {
        unsafe {
            standard_allocator_impl::AlignedAllocate(self, size, alignment)
        }
    }

    #[dev_inline]
    pub fn free(&mut self, address: *mut c_void) {
        unsafe {
            standard_allocator_impl::Free(self, address)
        }
    }

    #[dev_inline]
    pub fn realloc(&mut self, address: *mut c_void, new_size: usize) -> *mut c_void {
        unsafe {
            standard_allocator_impl::Reallocate(self, address, new_size)
        }
    }

    #[dev_inline]
    #[deprecated(note = "StandardAllocator::dump does not function release versions of nnsdk")]
    pub fn dump(&self) {
        unsafe {
            standard_allocator_impl::Dump(self)
        }
    }

    #[dev_inline]
    pub fn cleanup(&self) {
        unsafe {
            standard_allocator_impl::CleanUpManagementArea(self)
        }
    }

    #[dev_inline]
    pub fn clear_thread_cache(&self) {
        unsafe {
            standard_allocator_impl::ClearThreadCache(self)
        }
    }

    #[inline(always)]
    pub fn get_allocatable_size(&self) -> usize {
        unsafe {
            standard_allocator_impl::GetAllocatableSize(self)
        }
    }

    #[inline(always)]
    pub fn get_size_of_block(&self, block_start: *const c_void) -> usize {
        unsafe {
            standard_allocator_impl::GetSizeOf(self, block_start)
        }
    }

    #[inline(always)]
    pub fn get_total_free_size(&self) -> usize {
        unsafe {
            standard_allocator_impl::GetTotalFreeSize(self)
        }
    }

    #[inline(always)]
    pub fn get_hash(&self) -> AllocatorHash {
        unsafe {
            standard_allocator_impl::Hash(self)
        }
    }

    #[inline(always)]
    pub fn walk_allocated_blocks(&self, callback: AllocatorWalkCallback, user_data: *mut c_void) {
        unsafe {
            standard_allocator_impl::WalkAllocatedBlocks(self, callback, user_data)
        }
    }
}

impl Drop for StandardAllocator {
    fn drop(&mut self) {
        unsafe {
            standard_allocator_impl::Finalize(self)
        }
    }
}
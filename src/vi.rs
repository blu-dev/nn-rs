use libc::*;
use super::Result as NxResult;
use super::get_rust_result;
// It appears that nn::vi operates off of a lot of ptr-ptrs, so instead of not knowing the size of the struct or making the user
// deal with raw pointers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeWindowHandle(u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Layer(u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Display(u64);



extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi10InitializeEv"]
    fn Initialize();
    
    #[link_name = "\u{1}_ZN2nn2vi8FinalizeEv"]
    fn Finalize();

    #[link_name = "\u{1}_ZN2nn2vi18OpenDefaultDisplayEPPNS0_7DisplayE"]
    fn OpenDefaultDisplay(
        display: *mut Display
    ) -> NxResult;

    #[link_name = "\u{1}_ZN2nn2vi12CloseDisplayEPNS0_7DisplayE"]
    fn CloseDisplay(
        display: Display
    );

    #[link_name = "\u{1}_ZN2nn2vi11CreateLayerEPPNS0_5LayerEPNS0_7DisplayE"]
    fn CreateLayer(
        layer: *mut Layer,
        display: Display
    ) -> NxResult;

    #[link_name = "\u{1}_ZN2nn2vi12DestroyLayerEPNS0_5LayerE"]
    fn DestroyLayer(
        layer: Layer
    );

    #[link_name = "\u{1}_ZN2nn2vi15GetNativeWindowEPPvPNS0_5LayerE"]
    fn GetNativeWindow(
        handle: *mut NativeWindowHandle,
        layer: Layer
    ) -> NxResult;
}

pub fn init() {
    unsafe {
        Initialize();
    }
}

pub fn fini() {
    unsafe {
        Finalize();
    }
}

impl Layer {
    pub const fn uninit() -> Self {
        Self(0)
    }

    pub fn new(display: Display) -> Result<Self, NxResult> {
        unsafe {
            let mut layer = Layer(0);
            let result = CreateLayer(&mut layer, display);
            get_rust_result!(result, layer)
        }
    }

    pub fn native_handle(&self) -> Result<NativeWindowHandle, NxResult> {
        unsafe {
            let mut handle = NativeWindowHandle(0);
            let result = GetNativeWindow(&mut handle, *self);
            get_rust_result!(result, handle)
        }
    }

    // TODO: Make this drop maybe? not sure
    pub fn destroy(self) {
        unsafe {
            DestroyLayer(self)
        }
    }
}

impl Display {
    pub const fn uninit() -> Self {
        Self(0)
    }

    pub fn open_default() -> Result<Self, NxResult> {
        unsafe {
            let mut display = Display(0);
            let result = OpenDefaultDisplay(&mut display);
            get_rust_result!(result, display)
        }
    }

    pub fn close(self) {
        unsafe {
            CloseDisplay(self)
        }
    }
}

impl NativeWindowHandle {
    pub const fn uninit() -> Self {
        Self(0)
    }
}
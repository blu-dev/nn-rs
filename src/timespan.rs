// Intended to replicate core::time::Duration with #[repr(C)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimeSpan {
    pub nanos: u64
}

impl TimeSpan {
    pub const fn from_nanos(nanos: u64) -> Self {
        Self {
            nanos
        }
    }

    pub const fn from_micros(micros: u64) -> Self {
        Self {
            nanos: micros * 1000
        }
    }

    pub const fn from_millis(millis: u64) -> Self {
        Self {
            nanos: millis * 1000 * 1000
        }
    }

    pub const fn from_secs(secs: u64) -> Self {
        Self {
            nanos: secs * 1000 * 1000 * 1000
        }
    }

    pub const fn from_secs_f32(secs: f32) -> Self {
        Self {
            nanos: (secs * 1000.0 * 1000.0 * 1000.0) as u64
        }
    }

    pub const fn from_secs_f64(secs: f64) -> Self {
        Self {
            nanos: (secs * 1000.0 * 1000.0 * 1000.0) as u64
        }
    }

    pub const fn as_nanos(&self) -> u64 {
        self.nanos
    }

    pub const fn as_micros(&self) -> u64 {
        self.nanos / 1000
    }

    pub const fn as_millis(&self) -> u64 {
        self.as_micros() / 1000
    }

    pub const fn as_secs(&self) -> u64 {
        self.as_millis() / 1000
    }

    pub const fn as_secs_f32(&self) -> f32 {
        (self.nanos as f32) / (1000.0 * 1000.0 * 1000.0)
    }

    pub const fn as_secs_f64(&self) -> f64 {
        (self.nanos as f64) / (1000.0 * 1000.0 * 1000.0)
    }
}
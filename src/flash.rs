//! FLASH

use stm32f30x::{flash, FLASH};

/// Extension trait to constraint the FLASH peripheral
pub trait FlashExt {
    /// Constraints the FLASH peripheral to play nicely with the other abstractions
    fn constraint(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constraint(self) -> Parts {
        Parts {
            ACR: ACR { _0: () },
        }
    }
}

/// Constraint FLASH peripheral
#[allow(non_snake_case)]
pub struct Parts {
    /// Opaque ACR register
    pub ACR: ACR,
}

/// Opaque ACR register
pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}

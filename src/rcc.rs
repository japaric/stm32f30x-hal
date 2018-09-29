//! Reset and Clock Control

use core::cmp;

use cast::u32;
use stm32f30x::{rcc, RCC};

use flash::ACR;
use time::Hertz;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGR {
                hse: None,
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    /// Clock configuration
    pub cfgr: CFGR,
}

/// AMBA High-performance Bus (AHB) registers
pub struct AHB {
    _0: (),
}

impl AHB {
    pub(crate) fn enr(&mut self) -> &rcc::AHBENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahbenr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHBRSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahbrstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}

/// HSE Configuration of clock, divider and bypass
struct HseConfig {
    /// Clock speed of HSE
    speed: u32,
    /// Divider to be used, output will be HSE / divider
    divider: u32,
    /// If the clock driving circuitry is bypassed i.e. using an oscillator, not a crystal or
    /// resonator
    bypass: bool,
}

/// HSE bypass selector
pub enum HseBypass {
    /// If the clock driving circuitry is bypassed i.e. using an oscillator
    Enable,
    /// If the clock driving circuitry is not bypassed i.e. using a crystal or resonator
    Disable,
}

/// HSE divide selector
pub enum HseDivider {
    /// Do not divide HSE clock
    NoDivision,
    /// Divide HSE clock by 2
    DivideBy2,
    /// Divide HSE clock by 3
    DivideBy3,
    /// Divide HSE clock by 4
    DivideBy4,
    /// Divide HSE clock by 5
    DivideBy5,
    /// Divide HSE clock by 6
    DivideBy6,
    /// Divide HSE clock by 7
    DivideBy7,
    /// Divide HSE clock by 8
    DivideBy8,
    /// Divide HSE clock by 9
    DivideBy9,
    /// Divide HSE clock by 10
    DivideBy10,
    /// Divide HSE clock by 11
    DivideBy11,
    /// Divide HSE clock by 12
    DivideBy12,
    /// Divide HSE clock by 13
    DivideBy13,
    /// Divide HSE clock by 14
    DivideBy14,
    /// Divide HSE clock by 15
    DivideBy15,
    /// Divide HSE clock by 16
    DivideBy16,
}

const HSI: u32 = 8_000_000; // Hz

/// Clock configuration
pub struct CFGR {
    hse: Option<HseConfig>,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    /// Sets a HseConfig that checks that the HSE divider is valid and if the clock is in bypass
    /// mode
    pub fn hse<F>(mut self, freq: F, divider: HseDivider, bypass: HseBypass) -> Self
    where
        F: Into<Hertz>,
    {
        self.hse = Some(HseConfig {
            speed: freq.into().0,
            divider: match divider {
                HseDivider::NoDivision => 1,
                HseDivider::DivideBy2 => 2,
                HseDivider::DivideBy3 => 3,
                HseDivider::DivideBy4 => 4,
                HseDivider::DivideBy5 => 5,
                HseDivider::DivideBy6 => 6,
                HseDivider::DivideBy7 => 7,
                HseDivider::DivideBy8 => 8,
                HseDivider::DivideBy9 => 9,
                HseDivider::DivideBy10 => 10,
                HseDivider::DivideBy11 => 11,
                HseDivider::DivideBy12 => 12,
                HseDivider::DivideBy13 => 13,
                HseDivider::DivideBy14 => 14,
                HseDivider::DivideBy15 => 15,
                HseDivider::DivideBy16 => 16,
            },
            bypass: match bypass {
                HseBypass::Disable => false,
                HseBypass::Enable => true,
            },
        });

        self
    }

    /// Sets a frequency for the AHB bus
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB1 bus
    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB2 bus
    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    /// Sets the system (core) frequency
    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }

    /// Freezes the clock configuration, making it effective
    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        let pllmul = match &self.hse {
            Some(hse_cfg) => {
                let hse = hse_cfg.speed / hse_cfg.divider;
                self.sysclk.unwrap_or(hse) / hse // HSE has settable divider
            }
            None => (2 * self.sysclk.unwrap_or(HSI)) / HSI, // HSI is always divided by 2
        };

        let pllmul = cmp::min(cmp::max(pllmul, 2), 16);
        let pllmul_bits = if pllmul == 2 {
            None
        } else {
            Some(pllmul as u8 - 2)
        };

        let sysclk = match &self.hse {
            Some(hse_cfg) => {
                let hse = hse_cfg.speed / hse_cfg.divider;
                pllmul * hse
            }
            None => pllmul * HSI / 2,
        };

        assert!(sysclk <= 72_000_000);

        let hpre_bits = self
            .hclk
            .map(|hclk| match sysclk / hclk {
                0 => panic!("Requested HCLK is higher than generated SYSCLK"),
                1 => 0b0111,
                2 => 0b1000,
                3...5 => 0b1001,
                6...11 => 0b1010,
                12...39 => 0b1011,
                40...95 => 0b1100,
                96...191 => 0b1101,
                192...383 => 0b1110,
                _ => 0b1111,
            })
            .unwrap_or(0b0111);

        let hclk = sysclk / (1 << (hpre_bits - 0b0111));

        assert!(hclk <= 72_000_000);

        let ppre1_bits = self
            .pclk1
            .map(|pclk1| match hclk / pclk1 {
                0 => panic!("Requested PCLK1 is higher than generated HCLK"),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre1 = 1 << (ppre1_bits - 0b011);
        let pclk1 = hclk / u32(ppre1);

        assert!(pclk1 <= 36_000_000);

        let ppre2_bits = self
            .pclk2
            .map(|pclk2| match hclk / pclk2 {
                0 => panic!("Requested PCLK2 is higher than generated HCLK"),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre2 = 1 << (ppre2_bits - 0b011);
        let pclk2 = hclk / u32(ppre2);

        assert!(pclk2 <= 72_000_000);

        // adjust flash wait states
        unsafe {
            acr.acr().write(|w| {
                w.latency().bits(if sysclk <= 24_000_000 {
                    0b000
                } else if sysclk <= 48_000_000 {
                    0b001
                } else {
                    0b010
                })
            })
        }

        let rcc = unsafe { &*RCC::ptr() };
        // If HSE is available, set it up
        if let Some(hse_cfg) = &self.hse {
            if hse_cfg.bypass {
                // Bypass clock
                rcc.cr.write(|w| w.hseon().set_bit().hsebyp().set_bit());
            } else {
                rcc.cr.write(|w| w.hseon().set_bit());
            }

            // WARNING! Bit 0 in cfgr2 is connected to bit 17 in cfgr (due to MCU compatibility),
            // if bit 0 is set here it must also be set in any subsequent write to cfgr and
            // vise-versa
            rcc.cfgr2
                .write(|w| unsafe { w.prediv().bits(hse_cfg.divider as u8 - 1) });

            while rcc.cr.read().hserdy().bit_is_clear() {}
        }

        if let Some(pllmul_bits) = pllmul_bits {
            // use PLL as source

            if let Some(_) = &self.hse {
                // HSE as PLL input
                rcc.cfgr
                    .modify(|_, w| unsafe { w.pllsrc().set_bit().pllmul().bits(pllmul_bits) });
            } else {
                // HSI as PLL input
                rcc.cfgr.write(|w| unsafe { w.pllmul().bits(pllmul_bits) });
            }

            rcc.cr.modify(|_, w| w.pllon().set_bit());

            while rcc.cr.read().pllrdy().bit_is_clear() {}

            // SW: PLL selected as system clock
            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .bits(0b10)
            });
        } else {
            let sw_bits = if let Some(_) = &self.hse {
                // use HSE as source
                0b01
            } else {
                // use HSI as source
                0b00
            };

            rcc.cfgr.write(|w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .bits(sw_bits)
            });
        }

        // Running from HSE or PLL fed by HSE, disable HSI
        if let Some(_) = &self.hse {
            rcc.cr.modify(|_, w| w.hsion().clear_bit());
        }

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    // TODO remove `allow`
    #[allow(dead_code)]
    ppre2: u8,
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}

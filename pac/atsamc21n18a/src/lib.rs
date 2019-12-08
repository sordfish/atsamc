#![doc = "Peripheral access API for ATSAMC21N18A microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT();
    fn RTC();
    fn EIC();
    fn FREQM();
    fn TSENS();
    fn NVMCTRL();
    fn DMAC();
    fn EVSYS();
    fn SERCOM2();
    fn SERCOM3();
    fn SERCOM4();
    fn SERCOM5();
    fn CAN0();
    fn CAN1();
    fn TCC0();
    fn TCC1();
    fn TCC2();
    fn TC3();
    fn TC4();
    fn ADC0();
    fn ADC1();
    fn AC();
    fn DAC();
    fn SDADC();
    fn PTC();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _reserved: 0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: EIC },
    Vector { _handler: FREQM },
    Vector { _handler: TSENS },
    Vector { _handler: NVMCTRL },
    Vector { _handler: DMAC },
    Vector { _handler: EVSYS },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SERCOM2 },
    Vector { _handler: SERCOM3 },
    Vector { _handler: SERCOM4 },
    Vector { _handler: SERCOM5 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: TCC0 },
    Vector { _handler: TCC1 },
    Vector { _handler: TCC2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: AC },
    Vector { _handler: DAC },
    Vector { _handler: SDADC },
    Vector { _handler: PTC },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "1 - WDT"]
    WDT,
    #[doc = "2 - RTC"]
    RTC,
    #[doc = "3 - EIC"]
    EIC,
    #[doc = "4 - FREQM"]
    FREQM,
    #[doc = "5 - TSENS"]
    TSENS,
    #[doc = "6 - NVMCTRL"]
    NVMCTRL,
    #[doc = "7 - DMAC"]
    DMAC,
    #[doc = "8 - EVSYS"]
    EVSYS,
    #[doc = "11 - SERCOM2"]
    SERCOM2,
    #[doc = "12 - SERCOM3"]
    SERCOM3,
    #[doc = "13 - SERCOM4"]
    SERCOM4,
    #[doc = "14 - SERCOM5"]
    SERCOM5,
    #[doc = "15 - CAN0"]
    CAN0,
    #[doc = "16 - CAN1"]
    CAN1,
    #[doc = "17 - TCC0"]
    TCC0,
    #[doc = "18 - TCC1"]
    TCC1,
    #[doc = "19 - TCC2"]
    TCC2,
    #[doc = "23 - TC3"]
    TC3,
    #[doc = "24 - TC4"]
    TC4,
    #[doc = "25 - ADC0"]
    ADC0,
    #[doc = "26 - ADC1"]
    ADC1,
    #[doc = "27 - AC"]
    AC,
    #[doc = "28 - DAC"]
    DAC,
    #[doc = "29 - SDADC"]
    SDADC,
    #[doc = "30 - PTC"]
    PTC,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT => 1,
            Interrupt::RTC => 2,
            Interrupt::EIC => 3,
            Interrupt::FREQM => 4,
            Interrupt::TSENS => 5,
            Interrupt::NVMCTRL => 6,
            Interrupt::DMAC => 7,
            Interrupt::EVSYS => 8,
            Interrupt::SERCOM2 => 11,
            Interrupt::SERCOM3 => 12,
            Interrupt::SERCOM4 => 13,
            Interrupt::SERCOM5 => 14,
            Interrupt::CAN0 => 15,
            Interrupt::CAN1 => 16,
            Interrupt::TCC0 => 17,
            Interrupt::TCC1 => 18,
            Interrupt::TCC2 => 19,
            Interrupt::TC3 => 23,
            Interrupt::TC4 => 24,
            Interrupt::ADC0 => 25,
            Interrupt::ADC1 => 26,
            Interrupt::AC => 27,
            Interrupt::DAC => 28,
            Interrupt::SDADC => 29,
            Interrupt::PTC => 30,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0x4200_5000 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4200_4400 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog Digital Converter"]
pub mod adc0;
#[doc = "Analog Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4200_4800 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Control Area Network"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4200_1c00 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Control Area Network"]
pub mod can0;
#[doc = "Control Area Network"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4200_2000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub struct CCL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCL {}
impl CCL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccl::RegisterBlock {
        0x4200_5c00 as *const _
    }
}
impl Deref for CCL {
    type Target = ccl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCL::ptr() }
    }
}
#[doc = "Configurable Custom Logic"]
pub mod ccl;
#[doc = "Digital Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4200_5400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital Analog Converter"]
pub mod dac;
#[doc = "Divide and Square Root Accelerator"]
pub struct DIVAS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DIVAS {}
impl DIVAS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const divas::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for DIVAS {
    type Target = divas::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DIVAS::ptr() }
    }
}
#[doc = "Divide and Square Root Accelerator"]
pub mod divas;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x4100_6000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsu::RegisterBlock {
        0x4100_2000 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        0x4200_0000 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const freqm::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gclk::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "HSB Matrix"]
pub struct HMATRIXHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIXHS {}
impl HMATRIXHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrixhs::RegisterBlock {
        0x4100_a000 as *const _
    }
}
impl Deref for HMATRIXHS {
    type Target = hmatrixhs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMATRIXHS::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrixhs;
#[doc = "Main Clock"]
pub struct MCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCLK {}
impl MCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mclk::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for MCLK {
    type Target = mclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCLK::ptr() }
    }
}
#[doc = "Main Clock"]
pub mod mclk;
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        0x4100_8000 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub mod mtb;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        0x4100_4000 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Oscillators Control"]
pub struct OSCCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSCCTRL {}
impl OSCCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oscctrl::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for OSCCTRL {
    type Target = oscctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSCCTRL::ptr() }
    }
}
#[doc = "Oscillators Control"]
pub mod oscctrl;
#[doc = "32k Oscillators Control"]
pub struct OSC32KCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC32KCTRL {}
impl OSC32KCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc32kctrl::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for OSC32KCTRL {
    type Target = osc32kctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC32KCTRL::ptr() }
    }
}
#[doc = "32k Oscillators Control"]
pub mod osc32kctrl;
#[doc = "Peripheral Access Controller"]
pub struct PAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC {}
impl PAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PAC {
    type Target = pac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC::ptr() }
    }
}
#[doc = "Peripheral Access Controller"]
pub mod pac;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Port Module"]
pub struct PORT_IOBUS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS {}
impl PORT_IOBUS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x6000_0000 as *const _
    }
}
impl Deref for PORT_IOBUS {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_IOBUS::ptr() }
    }
}
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Sigma-Delta Analog Digital Converter"]
pub struct SDADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDADC {}
impl SDADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdadc::RegisterBlock {
        0x4200_4c00 as *const _
    }
}
impl Deref for SDADC {
    type Target = sdadc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDADC::ptr() }
    }
}
#[doc = "Sigma-Delta Analog Digital Converter"]
pub mod sdadc;
#[doc = "Serial Communication Interface"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_0400 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod sercom0;
#[doc = "Serial Communication Interface"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_0800 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_0c00 as *const _
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM2::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM3 {}
impl SERCOM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1000 as *const _
    }
}
impl Deref for SERCOM3 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM3::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM4 {}
impl SERCOM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1400 as *const _
    }
}
impl Deref for SERCOM4 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM4::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM5 {}
impl SERCOM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1800 as *const _
    }
}
impl Deref for SERCOM5 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM5::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM6 {}
impl SERCOM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0000 as *const _
    }
}
impl Deref for SERCOM6 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM6::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub struct SERCOM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM7 {}
impl SERCOM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4300_0400 as *const _
    }
}
impl Deref for SERCOM7 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM7::ptr() }
    }
}
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Basic Timer Counter"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_3000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub mod tc0;
#[doc = "Basic Timer Counter"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_3400 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_3800 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_3c00 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC4 {}
impl TC4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4200_4000 as *const _
    }
}
impl Deref for TC4 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC4::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC5 {}
impl TC5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4300_0800 as *const _
    }
}
impl Deref for TC5 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC5::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC6 {}
impl TC6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4300_0c00 as *const _
    }
}
impl Deref for TC6 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC6::ptr() }
    }
}
#[doc = "Basic Timer Counter"]
pub struct TC7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC7 {}
impl TC7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4300_1000 as *const _
    }
}
impl Deref for TC7 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC7::ptr() }
    }
}
#[doc = "Timer Counter Control"]
pub struct TCC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC0 {}
impl TCC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2400 as *const _
    }
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC0::ptr() }
    }
}
#[doc = "Timer Counter Control"]
pub mod tcc0;
#[doc = "Timer Counter Control"]
pub struct TCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC1 {}
impl TCC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2800 as *const _
    }
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC1::ptr() }
    }
}
#[doc = "Timer Counter Control"]
pub struct TCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC2 {}
impl TCC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2c00 as *const _
    }
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC2::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub struct TSENS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSENS {}
impl TSENS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tsens::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TSENS {
    type Target = tsens::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TSENS::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub mod tsens;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "System Control Registers"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Registers"]
pub mod system_control;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CCL"]
    pub CCL: CCL,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DIVAS"]
    pub DIVAS: DIVAS,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "HMATRIXHS"]
    pub HMATRIXHS: HMATRIXHS,
    #[doc = "MCLK"]
    pub MCLK: MCLK,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "OSCCTRL"]
    pub OSCCTRL: OSCCTRL,
    #[doc = "OSC32KCTRL"]
    pub OSC32KCTRL: OSC32KCTRL,
    #[doc = "PAC"]
    pub PAC: PAC,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "PORT_IOBUS"]
    pub PORT_IOBUS: PORT_IOBUS,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDADC"]
    pub SDADC: SDADC,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SERCOM3"]
    pub SERCOM3: SERCOM3,
    #[doc = "SERCOM4"]
    pub SERCOM4: SERCOM4,
    #[doc = "SERCOM5"]
    pub SERCOM5: SERCOM5,
    #[doc = "SERCOM6"]
    pub SERCOM6: SERCOM6,
    #[doc = "SERCOM7"]
    pub SERCOM7: SERCOM7,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TC4"]
    pub TC4: TC4,
    #[doc = "TC5"]
    pub TC5: TC5,
    #[doc = "TC6"]
    pub TC6: TC6,
    #[doc = "TC7"]
    pub TC7: TC7,
    #[doc = "TCC0"]
    pub TCC0: TCC0,
    #[doc = "TCC1"]
    pub TCC1: TCC1,
    #[doc = "TCC2"]
    pub TCC2: TCC2,
    #[doc = "TSENS"]
    pub TSENS: TSENS,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CCL: CCL {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DIVAS: DIVAS {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            HMATRIXHS: HMATRIXHS {
                _marker: PhantomData,
            },
            MCLK: MCLK {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            OSCCTRL: OSCCTRL {
                _marker: PhantomData,
            },
            OSC32KCTRL: OSC32KCTRL {
                _marker: PhantomData,
            },
            PAC: PAC {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            PORT_IOBUS: PORT_IOBUS {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SDADC: SDADC {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SERCOM3: SERCOM3 {
                _marker: PhantomData,
            },
            SERCOM4: SERCOM4 {
                _marker: PhantomData,
            },
            SERCOM5: SERCOM5 {
                _marker: PhantomData,
            },
            SERCOM6: SERCOM6 {
                _marker: PhantomData,
            },
            SERCOM7: SERCOM7 {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TC4: TC4 {
                _marker: PhantomData,
            },
            TC5: TC5 {
                _marker: PhantomData,
            },
            TC6: TC6 {
                _marker: PhantomData,
            },
            TC7: TC7 {
                _marker: PhantomData,
            },
            TCC0: TCC0 {
                _marker: PhantomData,
            },
            TCC1: TCC1 {
                _marker: PhantomData,
            },
            TCC2: TCC2 {
                _marker: PhantomData,
            },
            TSENS: TSENS {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
        }
    }
}

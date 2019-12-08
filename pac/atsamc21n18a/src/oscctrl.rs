#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: XOSCCTRL,
    #[doc = "0x12 - Clock Failure Detector Prescaler"]
    pub cfdpresc: CFDPRESC,
    #[doc = "0x13 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x14 - 48MHz Internal Oscillator (OSC48M) Control"]
    pub osc48mctrl: OSC48MCTRL,
    #[doc = "0x15 - OSC48M Divider"]
    pub osc48mdiv: OSC48MDIV,
    #[doc = "0x16 - OSC48M Startup Time"]
    pub osc48mstup: OSC48MSTUP,
    _reserved10: [u8; 1usize],
    #[doc = "0x18 - OSC48M Synchronization Busy"]
    pub osc48msyncbusy: OSC48MSYNCBUSY,
    #[doc = "0x1c - DPLL Control"]
    pub dpllctrla: DPLLCTRLA,
    _reserved12: [u8; 3usize],
    #[doc = "0x20 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x24 - Digital Core Configuration"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x28 - DPLL Prescaler"]
    pub dpllpresc: DPLLPRESC,
    _reserved15: [u8; 3usize],
    #[doc = "0x2c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    _reserved16: [u8; 3usize],
    #[doc = "0x30 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
    _reserved17: [u8; 7usize],
    #[doc = "0x38 - 48MHz Oscillator Calibration"]
    pub cal48m: CAL48M,
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u32, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoscctrl](xoscctrl) module"]
pub type XOSCCTRL = crate::Reg<u16, _XOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSCCTRL;
#[doc = "`read()` method returns [xoscctrl::R](xoscctrl::R) reader structure"]
impl crate::Readable for XOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](xoscctrl::W) writer structure"]
impl crate::Writable for XOSCCTRL {}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "Clock Failure Detector Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdpresc](cfdpresc) module"]
pub type CFDPRESC = crate::Reg<u8, _CFDPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFDPRESC;
#[doc = "`read()` method returns [cfdpresc::R](cfdpresc::R) reader structure"]
impl crate::Readable for CFDPRESC {}
#[doc = "`write(|w| ..)` method takes [cfdpresc::W](cfdpresc::W) writer structure"]
impl crate::Writable for CFDPRESC {}
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "48MHz Internal Oscillator (OSC48M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48mctrl](osc48mctrl) module"]
pub type OSC48MCTRL = crate::Reg<u8, _OSC48MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC48MCTRL;
#[doc = "`read()` method returns [osc48mctrl::R](osc48mctrl::R) reader structure"]
impl crate::Readable for OSC48MCTRL {}
#[doc = "`write(|w| ..)` method takes [osc48mctrl::W](osc48mctrl::W) writer structure"]
impl crate::Writable for OSC48MCTRL {}
#[doc = "48MHz Internal Oscillator (OSC48M) Control"]
pub mod osc48mctrl;
#[doc = "OSC48M Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48mdiv](osc48mdiv) module"]
pub type OSC48MDIV = crate::Reg<u8, _OSC48MDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC48MDIV;
#[doc = "`read()` method returns [osc48mdiv::R](osc48mdiv::R) reader structure"]
impl crate::Readable for OSC48MDIV {}
#[doc = "`write(|w| ..)` method takes [osc48mdiv::W](osc48mdiv::W) writer structure"]
impl crate::Writable for OSC48MDIV {}
#[doc = "OSC48M Divider"]
pub mod osc48mdiv;
#[doc = "OSC48M Startup Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48mstup](osc48mstup) module"]
pub type OSC48MSTUP = crate::Reg<u8, _OSC48MSTUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC48MSTUP;
#[doc = "`read()` method returns [osc48mstup::R](osc48mstup::R) reader structure"]
impl crate::Readable for OSC48MSTUP {}
#[doc = "`write(|w| ..)` method takes [osc48mstup::W](osc48mstup::W) writer structure"]
impl crate::Writable for OSC48MSTUP {}
#[doc = "OSC48M Startup Time"]
pub mod osc48mstup;
#[doc = "OSC48M Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48msyncbusy](osc48msyncbusy) module"]
pub type OSC48MSYNCBUSY = crate::Reg<u32, _OSC48MSYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC48MSYNCBUSY;
#[doc = "`read()` method returns [osc48msyncbusy::R](osc48msyncbusy::R) reader structure"]
impl crate::Readable for OSC48MSYNCBUSY {}
#[doc = "OSC48M Synchronization Busy"]
pub mod osc48msyncbusy;
#[doc = "DPLL Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrla](dpllctrla) module"]
pub type DPLLCTRLA = crate::Reg<u8, _DPLLCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLA;
#[doc = "`read()` method returns [dpllctrla::R](dpllctrla::R) reader structure"]
impl crate::Readable for DPLLCTRLA {}
#[doc = "`write(|w| ..)` method takes [dpllctrla::W](dpllctrla::W) writer structure"]
impl crate::Writable for DPLLCTRLA {}
#[doc = "DPLL Control"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllratio](dpllratio) module"]
pub type DPLLRATIO = crate::Reg<u32, _DPLLRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLRATIO;
#[doc = "`read()` method returns [dpllratio::R](dpllratio::R) reader structure"]
impl crate::Readable for DPLLRATIO {}
#[doc = "`write(|w| ..)` method takes [dpllratio::W](dpllratio::W) writer structure"]
impl crate::Writable for DPLLRATIO {}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "Digital Core Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](dpllctrlb) module"]
pub type DPLLCTRLB = crate::Reg<u32, _DPLLCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLB;
#[doc = "`read()` method returns [dpllctrlb::R](dpllctrlb::R) reader structure"]
impl crate::Readable for DPLLCTRLB {}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](dpllctrlb::W) writer structure"]
impl crate::Writable for DPLLCTRLB {}
#[doc = "Digital Core Configuration"]
pub mod dpllctrlb;
#[doc = "DPLL Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllpresc](dpllpresc) module"]
pub type DPLLPRESC = crate::Reg<u8, _DPLLPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLPRESC;
#[doc = "`read()` method returns [dpllpresc::R](dpllpresc::R) reader structure"]
impl crate::Readable for DPLLPRESC {}
#[doc = "`write(|w| ..)` method takes [dpllpresc::W](dpllpresc::W) writer structure"]
impl crate::Writable for DPLLPRESC {}
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLL Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllsyncbusy](dpllsyncbusy) module"]
pub type DPLLSYNCBUSY = crate::Reg<u8, _DPLLSYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSYNCBUSY;
#[doc = "`read()` method returns [dpllsyncbusy::R](dpllsyncbusy::R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY {}
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllstatus](dpllstatus) module"]
pub type DPLLSTATUS = crate::Reg<u8, _DPLLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSTATUS;
#[doc = "`read()` method returns [dpllstatus::R](dpllstatus::R) reader structure"]
impl crate::Readable for DPLLSTATUS {}
#[doc = "DPLL Status"]
pub mod dpllstatus;
#[doc = "48MHz Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal48m](cal48m) module"]
pub type CAL48M = crate::Reg<u32, _CAL48M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL48M;
#[doc = "`read()` method returns [cal48m::R](cal48m::R) reader structure"]
impl crate::Readable for CAL48M {}
#[doc = "`write(|w| ..)` method takes [cal48m::W](cal48m::W) writer structure"]
impl crate::Writable for CAL48M {}
#[doc = "48MHz Oscillator Calibration"]
pub mod cal48m;

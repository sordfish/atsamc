#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x02 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x04 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x05 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x06 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x07 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x08 - Sequence Status"]
    pub seqstatus: SEQSTATUS,
    #[doc = "0x09 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x0a - Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x0b - Window Monitor Control"]
    pub winctrl: WINCTRL,
    #[doc = "0x0c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    #[doc = "0x10 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    #[doc = "0x14 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x18 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x1a - Shift Correction"]
    pub shiftcorr: SHIFTCORR,
    _reserved16: [u8; 1usize],
    #[doc = "0x1c - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved17: [u8; 3usize],
    #[doc = "0x20 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x24 - Result"]
    pub result: RESULT,
    #[doc = "0x28 - Sequence Control"]
    pub seqctrl: SEQCTRL,
    _reserved20: [u8; 3usize],
    #[doc = "0x2c - Analog Control"]
    pub anactrl: ANACTRL,
    _reserved21: [u8; 1usize],
    #[doc = "0x2e - Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Reference Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctrl](refctrl) module"]
pub type REFCTRL = crate::Reg<u8, _REFCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTRL;
#[doc = "`read()` method returns [refctrl::R](refctrl::R) reader structure"]
impl crate::Readable for REFCTRL {}
#[doc = "`write(|w| ..)` method takes [refctrl::W](refctrl::W) writer structure"]
impl crate::Writable for REFCTRL {}
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
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
pub type INTENSET = crate::Reg<u8, _INTENSET>;
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
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Sequence Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqstatus](seqstatus) module"]
pub type SEQSTATUS = crate::Reg<u8, _SEQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQSTATUS;
#[doc = "`read()` method returns [seqstatus::R](seqstatus::R) reader structure"]
impl crate::Readable for SEQSTATUS {}
#[doc = "Sequence Status"]
pub mod seqstatus;
#[doc = "Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrl](inputctrl) module"]
pub type INPUTCTRL = crate::Reg<u8, _INPUTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTCTRL;
#[doc = "`read()` method returns [inputctrl::R](inputctrl::R) reader structure"]
impl crate::Readable for INPUTCTRL {}
#[doc = "`write(|w| ..)` method takes [inputctrl::W](inputctrl::W) writer structure"]
impl crate::Writable for INPUTCTRL {}
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](ctrlc) module"]
pub type CTRLC = crate::Reg<u8, _CTRLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLC;
#[doc = "`read()` method returns [ctrlc::R](ctrlc::R) reader structure"]
impl crate::Readable for CTRLC {}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](ctrlc::W) writer structure"]
impl crate::Writable for CTRLC {}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Window Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winctrl](winctrl) module"]
pub type WINCTRL = crate::Reg<u8, _WINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINCTRL;
#[doc = "`read()` method returns [winctrl::R](winctrl::R) reader structure"]
impl crate::Readable for WINCTRL {}
#[doc = "`write(|w| ..)` method takes [winctrl::W](winctrl::W) writer structure"]
impl crate::Writable for WINCTRL {}
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "Window Monitor Lower Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winlt](winlt) module"]
pub type WINLT = crate::Reg<u32, _WINLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINLT;
#[doc = "`read()` method returns [winlt::R](winlt::R) reader structure"]
impl crate::Readable for WINLT {}
#[doc = "`write(|w| ..)` method takes [winlt::W](winlt::W) writer structure"]
impl crate::Writable for WINLT {}
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "Window Monitor Upper Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winut](winut) module"]
pub type WINUT = crate::Reg<u32, _WINUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINUT;
#[doc = "`read()` method returns [winut::R](winut::R) reader structure"]
impl crate::Readable for WINUT {}
#[doc = "`write(|w| ..)` method takes [winut::W](winut::W) writer structure"]
impl crate::Writable for WINUT {}
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "Offset Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsetcorr](offsetcorr) module"]
pub type OFFSETCORR = crate::Reg<u32, _OFFSETCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFSETCORR;
#[doc = "`read()` method returns [offsetcorr::R](offsetcorr::R) reader structure"]
impl crate::Readable for OFFSETCORR {}
#[doc = "`write(|w| ..)` method takes [offsetcorr::W](offsetcorr::W) writer structure"]
impl crate::Writable for OFFSETCORR {}
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "Gain Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaincorr](gaincorr) module"]
pub type GAINCORR = crate::Reg<u16, _GAINCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAINCORR;
#[doc = "`read()` method returns [gaincorr::R](gaincorr::R) reader structure"]
impl crate::Readable for GAINCORR {}
#[doc = "`write(|w| ..)` method takes [gaincorr::W](gaincorr::W) writer structure"]
impl crate::Writable for GAINCORR {}
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "Shift Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcorr](shiftcorr) module"]
pub type SHIFTCORR = crate::Reg<u8, _SHIFTCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCORR;
#[doc = "`read()` method returns [shiftcorr::R](shiftcorr::R) reader structure"]
impl crate::Readable for SHIFTCORR {}
#[doc = "`write(|w| ..)` method takes [shiftcorr::W](shiftcorr::W) writer structure"]
impl crate::Writable for SHIFTCORR {}
#[doc = "Shift Correction"]
pub mod shiftcorr;
#[doc = "Software Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](swtrig) module"]
pub type SWTRIG = crate::Reg<u8, _SWTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIG;
#[doc = "`read()` method returns [swtrig::R](swtrig::R) reader structure"]
impl crate::Readable for SWTRIG {}
#[doc = "`write(|w| ..)` method takes [swtrig::W](swtrig::W) writer structure"]
impl crate::Writable for SWTRIG {}
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result"]
pub mod result;
#[doc = "Sequence Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](seqctrl) module"]
pub type SEQCTRL = crate::Reg<u8, _SEQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCTRL;
#[doc = "`read()` method returns [seqctrl::R](seqctrl::R) reader structure"]
impl crate::Readable for SEQCTRL {}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](seqctrl::W) writer structure"]
impl crate::Writable for SEQCTRL {}
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "Analog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl](anactrl) module"]
pub type ANACTRL = crate::Reg<u8, _ANACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL;
#[doc = "`read()` method returns [anactrl::R](anactrl::R) reader structure"]
impl crate::Readable for ANACTRL {}
#[doc = "`write(|w| ..)` method takes [anactrl::W](anactrl::W) writer structure"]
impl crate::Writable for ANACTRL {}
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;

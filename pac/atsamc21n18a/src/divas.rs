#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Dividend"]
    pub dividend: DIVIDEND,
    #[doc = "0x0c - Divisor"]
    pub divisor: DIVISOR,
    #[doc = "0x10 - Result"]
    pub result: RESULT,
    #[doc = "0x14 - Remainder"]
    pub rem: REM,
    #[doc = "0x18 - Square Root Input"]
    pub sqrnum: SQRNUM,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Dividend\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dividend](dividend) module"]
pub type DIVIDEND = crate::Reg<u32, _DIVIDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVIDEND;
#[doc = "`read()` method returns [dividend::R](dividend::R) reader structure"]
impl crate::Readable for DIVIDEND {}
#[doc = "`write(|w| ..)` method takes [dividend::W](dividend::W) writer structure"]
impl crate::Writable for DIVIDEND {}
#[doc = "Dividend"]
pub mod dividend;
#[doc = "Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divisor](divisor) module"]
pub type DIVISOR = crate::Reg<u32, _DIVISOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVISOR;
#[doc = "`read()` method returns [divisor::R](divisor::R) reader structure"]
impl crate::Readable for DIVISOR {}
#[doc = "`write(|w| ..)` method takes [divisor::W](divisor::W) writer structure"]
impl crate::Writable for DIVISOR {}
#[doc = "Divisor"]
pub mod divisor;
#[doc = "Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result"]
pub mod result;
#[doc = "Remainder\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rem](rem) module"]
pub type REM = crate::Reg<u32, _REM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REM;
#[doc = "`read()` method returns [rem::R](rem::R) reader structure"]
impl crate::Readable for REM {}
#[doc = "Remainder"]
pub mod rem;
#[doc = "Square Root Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqrnum](sqrnum) module"]
pub type SQRNUM = crate::Reg<u32, _SQRNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQRNUM;
#[doc = "`read()` method returns [sqrnum::R](sqrnum::R) reader structure"]
impl crate::Readable for SQRNUM {}
#[doc = "`write(|w| ..)` method takes [sqrnum::W](sqrnum::W) writer structure"]
impl crate::Writable for SQRNUM {}
#[doc = "Square Root Input"]
pub mod sqrnum;

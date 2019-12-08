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
    #[doc = "0x10 - BODVDD Control"]
    pub bodvdd: BODVDD,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - VREG Control"]
    pub vreg: VREG,
    #[doc = "0x1c - VREF Control"]
    pub vref: VREF,
    #[doc = "0x20 - VREG33 Control"]
    pub vreg33: VREG33,
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
#[doc = "BODVDD Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodvdd](bodvdd) module"]
pub type BODVDD = crate::Reg<u32, _BODVDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODVDD;
#[doc = "`read()` method returns [bodvdd::R](bodvdd::R) reader structure"]
impl crate::Readable for BODVDD {}
#[doc = "`write(|w| ..)` method takes [bodvdd::W](bodvdd::W) writer structure"]
impl crate::Writable for BODVDD {}
#[doc = "BODVDD Control"]
pub mod bodvdd;
#[doc = "VREG Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg](vreg) module"]
pub type VREG = crate::Reg<u32, _VREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREG;
#[doc = "`read()` method returns [vreg::R](vreg::R) reader structure"]
impl crate::Readable for VREG {}
#[doc = "`write(|w| ..)` method takes [vreg::W](vreg::W) writer structure"]
impl crate::Writable for VREG {}
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](vref) module"]
pub type VREF = crate::Reg<u32, _VREF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF;
#[doc = "`read()` method returns [vref::R](vref::R) reader structure"]
impl crate::Readable for VREF {}
#[doc = "`write(|w| ..)` method takes [vref::W](vref::W) writer structure"]
impl crate::Writable for VREF {}
#[doc = "VREF Control"]
pub mod vref;
#[doc = "VREG33 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg33](vreg33) module"]
pub type VREG33 = crate::Reg<u32, _VREG33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREG33;
#[doc = "`read()` method returns [vreg33::R](vreg33::R) reader structure"]
impl crate::Readable for VREG33 {}
#[doc = "`write(|w| ..)` method takes [vreg33::W](vreg33::W) writer structure"]
impl crate::Writable for VREG33 {}
#[doc = "VREG33 Control"]
pub mod vreg33;

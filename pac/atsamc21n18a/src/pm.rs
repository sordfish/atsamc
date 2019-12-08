#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    _reserved1: [u8; 6usize],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
}
#[doc = "Sleep Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcfg](sleepcfg) module"]
pub type SLEEPCFG = crate::Reg<u8, _SLEEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCFG;
#[doc = "`read()` method returns [sleepcfg::R](sleepcfg::R) reader structure"]
impl crate::Readable for SLEEPCFG {}
#[doc = "`write(|w| ..)` method takes [sleepcfg::W](sleepcfg::W) writer structure"]
impl crate::Writable for SLEEPCFG {}
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](stdbycfg) module"]
pub type STDBYCFG = crate::Reg<u16, _STDBYCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STDBYCFG;
#[doc = "`read()` method returns [stdbycfg::R](stdbycfg::R) reader structure"]
impl crate::Readable for STDBYCFG {}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](stdbycfg::W) writer structure"]
impl crate::Writable for STDBYCFG {}
#[doc = "Standby Configuration"]
pub mod stdbycfg;

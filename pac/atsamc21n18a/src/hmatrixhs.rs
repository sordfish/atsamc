#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration"]
    pub mcfg: [MCFG; 16],
    #[doc = "0x40 - Slave Configuration"]
    pub scfg: [SCFG; 16],
    #[doc = "0x80 - cluster"]
    pub prs: [PRS; 4],
    _reserved3: [u8; 96usize],
    #[doc = "0x100 - Master Remap Control"]
    pub mrcr: MRCR,
    _reserved4: [u8; 12usize],
    #[doc = "0x110 - Special Function"]
    pub sfr: [SFR; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRS {
    #[doc = "0x00 - Priority A for Slave"]
    pub pras: self::prs::PRAS,
    #[doc = "0x04 - Priority B for Slave"]
    pub prbs: self::prs::PRBS,
}
#[doc = r"Register block"]
#[doc = "cluster"]
pub mod prs;
#[doc = "Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "Master Configuration"]
pub mod mcfg;
#[doc = "Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](scfg) module"]
pub type SCFG = crate::Reg<u32, _SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFG;
#[doc = "`read()` method returns [scfg::R](scfg::R) reader structure"]
impl crate::Readable for SCFG {}
#[doc = "`write(|w| ..)` method takes [scfg::W](scfg::W) writer structure"]
impl crate::Writable for SCFG {}
#[doc = "Slave Configuration"]
pub mod scfg;
#[doc = "Master Remap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcr](mrcr) module"]
pub type MRCR = crate::Reg<u32, _MRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRCR;
#[doc = "`read()` method returns [mrcr::R](mrcr::R) reader structure"]
impl crate::Readable for MRCR {}
#[doc = "`write(|w| ..)` method takes [mrcr::W](mrcr::W) writer structure"]
impl crate::Writable for MRCR {}
#[doc = "Master Remap Control"]
pub mod mrcr;
#[doc = "Special Function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`read()` method returns [sfr::R](sfr::R) reader structure"]
impl crate::Readable for SFR {}
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "Special Function"]
pub mod sfr;

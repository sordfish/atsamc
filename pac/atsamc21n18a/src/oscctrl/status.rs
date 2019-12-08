#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCFAIL`"]
pub type XOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCCKSW`"]
pub type XOSCCKSW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC48MRDY`"]
pub type OSC48MRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLTO`"]
pub type DPLLTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLDRTO`"]
pub type DPLLLDRTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XOSCCKSW_R {
        XOSCCKSW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> OSC48MRDY_R {
        OSC48MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DPLL Timeout"]
    #[inline(always)]
    pub fn dpllto(&self) -> DPLLTO_R {
        DPLLTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}

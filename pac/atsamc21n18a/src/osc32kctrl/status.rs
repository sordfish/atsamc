#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC32KRDY`"]
pub type OSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKFAIL`"]
pub type CLKFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKSW`"]
pub type CLKSW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn clkfail(&self) -> CLKFAIL_R {
        CLKFAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XOSC32K Clock switch"]
    #[inline(always)]
    pub fn clksw(&self) -> CLKSW_R {
        CLKSW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}

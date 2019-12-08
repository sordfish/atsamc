#[doc = "Reader of register STATUSD"]
pub type R = crate::R<u32, super::STATUSD>;
#[doc = "Reader of field `SERCOM6_`"]
pub type SERCOM6__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM7_`"]
pub type SERCOM7__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC5_`"]
pub type TC5__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC6_`"]
pub type TC6__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC7_`"]
pub type TC7__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SERCOM6 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> SERCOM6__R {
        SERCOM6__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM7 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> SERCOM7__R {
        SERCOM7__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC6 APB Protect Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> TC6__R {
        TC6__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC7 APB Protect Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> TC7__R {
        TC7__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}

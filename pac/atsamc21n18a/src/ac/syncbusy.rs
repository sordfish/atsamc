#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WINCTRL`"]
pub type WINCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPCTRL0`"]
pub type COMPCTRL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPCTRL1`"]
pub type COMPCTRL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPCTRL2`"]
pub type COMPCTRL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMPCTRL3`"]
pub type COMPCTRL3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WINCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn winctrl(&self) -> WINCTRL_R {
        WINCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COMPCTRL 0 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl0(&self) -> COMPCTRL0_R {
        COMPCTRL0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - COMPCTRL 1 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl1(&self) -> COMPCTRL1_R {
        COMPCTRL1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COMPCTRL 2 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl2(&self) -> COMPCTRL2_R {
        COMPCTRL2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - COMPCTRL 3 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl3(&self) -> COMPCTRL3_R {
        COMPCTRL3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}

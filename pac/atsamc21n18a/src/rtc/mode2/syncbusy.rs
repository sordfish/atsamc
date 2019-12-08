#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FREQCORR`"]
pub type FREQCORR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLOCK`"]
pub type CLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM0`"]
pub type ALARM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK0`"]
pub type MASK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLOCKSYNC`"]
pub type CLOCKSYNC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Bit Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FREQCORR_R {
        FREQCORR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLOCK Register Busy"]
    #[inline(always)]
    pub fn clock(&self) -> CLOCK_R {
        CLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ALARM 0 Register Busy"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MASK 0 Register Busy"]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn clocksync(&self) -> CLOCKSYNC_R {
        CLOCKSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}

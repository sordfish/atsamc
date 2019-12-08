#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FREQCORR`"]
pub type FREQCORR_R = crate::R<bool, bool>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP0`"]
pub type COMP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `COUNTSYNC`"]
pub type COUNTSYNC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Busy"]
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
    #[doc = "Bit 3 - COUNT Register Busy"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COMP 0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Count Read Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn countsync(&self) -> COUNTSYNC_R {
        COUNTSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}

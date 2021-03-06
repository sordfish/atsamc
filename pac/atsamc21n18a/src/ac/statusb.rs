#[doc = "Reader of register STATUSB"]
pub type R = crate::R<u8, super::STATUSB>;
#[doc = "Reader of field `READY0`"]
pub type READY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY1`"]
pub type READY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY2`"]
pub type READY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY3`"]
pub type READY3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Ready"]
    #[inline(always)]
    pub fn ready2(&self) -> READY2_R {
        READY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Ready"]
    #[inline(always)]
    pub fn ready3(&self) -> READY3_R {
        READY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}

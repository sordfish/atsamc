#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BODVDDRDY`"]
pub type BODVDDRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODVDDDET`"]
pub type BODVDDDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `BVDDSRDY`"]
pub type BVDDSRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREG33RDY`"]
pub type VREG33RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BODVDDRDY_R {
        BODVDDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&self) -> BODVDDDET_R {
        BODVDDDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&self) -> BVDDSRDY_R {
        BVDDSRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&self) -> VREG33RDY_R {
        VREG33RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}

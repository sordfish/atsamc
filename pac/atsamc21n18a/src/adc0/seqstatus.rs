#[doc = "Reader of register SEQSTATUS"]
pub type R = crate::R<u8, super::SEQSTATUS>;
#[doc = "Reader of field `SEQSTATE`"]
pub type SEQSTATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEQBUSY`"]
pub type SEQBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Sequence State"]
    #[inline(always)]
    pub fn seqstate(&self) -> SEQSTATE_R {
        SEQSTATE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Sequence Busy"]
    #[inline(always)]
    pub fn seqbusy(&self) -> SEQBUSY_R {
        SEQBUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}

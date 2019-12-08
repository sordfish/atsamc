#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Result Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 0x01) != 0)
    }
}

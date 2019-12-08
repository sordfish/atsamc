#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
}

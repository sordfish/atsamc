#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

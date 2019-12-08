#[doc = "Reader of register REM"]
pub type R = crate::R<u32, super::REM>;
#[doc = "Reader of field `REM`"]
pub type REM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - REM"]
    #[inline(always)]
    pub fn rem(&self) -> REM_R {
        REM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

#[doc = "Reader of register SHIFTCORR"]
pub type R = crate::R<u8, super::SHIFTCORR>;
#[doc = "Writer for register SHIFTCORR"]
pub type W = crate::W<u8, super::SHIFTCORR>;
#[doc = "Register SHIFTCORR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTCORR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTCORR`"]
pub type SHIFTCORR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFTCORR`"]
pub struct SHIFTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    pub fn shiftcorr(&self) -> SHIFTCORR_R {
        SHIFTCORR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    pub fn shiftcorr(&mut self) -> SHIFTCORR_W {
        SHIFTCORR_W { w: self }
    }
}

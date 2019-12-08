#[doc = "Reader of register OFFSET"]
pub type R = crate::R<u32, super::OFFSET>;
#[doc = "Writer for register OFFSET"]
pub type W = crate::W<u32, super::OFFSET>;
#[doc = "Register OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSETC`"]
pub type OFFSETC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OFFSETC`"]
pub struct OFFSETC_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    pub fn offsetc(&self) -> OFFSETC_R {
        OFFSETC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    pub fn offsetc(&mut self) -> OFFSETC_W {
        OFFSETC_W { w: self }
    }
}

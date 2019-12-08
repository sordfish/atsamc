#[doc = "Reader of register DIVISOR"]
pub type R = crate::R<u32, super::DIVISOR>;
#[doc = "Writer for register DIVISOR"]
pub type W = crate::W<u32, super::DIVISOR>;
#[doc = "Register DIVISOR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIVISOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVISOR`"]
pub type DIVISOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIVISOR`"]
pub struct DIVISOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DIVISOR"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DIVISOR"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W { w: self }
    }
}

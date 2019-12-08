#[doc = "Reader of register WINLT"]
pub type R = crate::R<u32, super::WINLT>;
#[doc = "Writer for register WINLT"]
pub type W = crate::W<u32, super::WINLT>;
#[doc = "Register WINLT `reset()`'s with value 0"]
impl crate::ResetValue for super::WINLT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINLT`"]
pub type WINLT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WINLT`"]
pub struct WINLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&mut self) -> WINLT_W {
        WINLT_W { w: self }
    }
}

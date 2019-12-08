#[doc = "Reader of register SQRNUM"]
pub type R = crate::R<u32, super::SQRNUM>;
#[doc = "Writer for register SQRNUM"]
pub type W = crate::W<u32, super::SQRNUM>;
#[doc = "Register SQRNUM `reset()`'s with value 0"]
impl crate::ResetValue for super::SQRNUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQRNUM`"]
pub type SQRNUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SQRNUM`"]
pub struct SQRNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SQRNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    pub fn sqrnum(&self) -> SQRNUM_R {
        SQRNUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    pub fn sqrnum(&mut self) -> SQRNUM_W {
        SQRNUM_W { w: self }
    }
}

#[doc = "Reader of register SEQCTRL"]
pub type R = crate::R<u32, super::SEQCTRL>;
#[doc = "Writer for register SEQCTRL"]
pub type W = crate::W<u32, super::SEQCTRL>;
#[doc = "Register SEQCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEQEN`"]
pub type SEQEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEQEN`"]
pub struct SEQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    pub fn seqen(&self) -> SEQEN_R {
        SEQEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    pub fn seqen(&mut self) -> SEQEN_W {
        SEQEN_W { w: self }
    }
}

#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Writer for register CAL"]
pub type W = crate::W<u32, super::CAL>;
#[doc = "Register CAL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCAL`"]
pub type FCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCAL`"]
pub struct FCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TCAL`"]
pub type TCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCAL`"]
pub struct TCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Frequency Calibration"]
    #[inline(always)]
    pub fn fcal(&self) -> FCAL_R {
        FCAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Temperature Calibration"]
    #[inline(always)]
    pub fn tcal(&self) -> TCAL_R {
        TCAL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency Calibration"]
    #[inline(always)]
    pub fn fcal(&mut self) -> FCAL_W {
        FCAL_W { w: self }
    }
    #[doc = "Bits 8:13 - Temperature Calibration"]
    #[inline(always)]
    pub fn tcal(&mut self) -> TCAL_W {
        TCAL_W { w: self }
    }
}

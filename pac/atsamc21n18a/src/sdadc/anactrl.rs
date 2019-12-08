#[doc = "Reader of register ANACTRL"]
pub type R = crate::R<u8, super::ANACTRL>;
#[doc = "Writer for register ANACTRL"]
pub type W = crate::W<u8, super::ANACTRL>;
#[doc = "Register ANACTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ANACTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTRSDADC`"]
pub type CTRSDADC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRSDADC`"]
pub struct CTRSDADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRSDADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ONCHOP`"]
pub type ONCHOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONCHOP`"]
pub struct ONCHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ONCHOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BUFTEST`"]
pub type BUFTEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFTEST`"]
pub struct BUFTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFTEST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    pub fn ctrsdadc(&self) -> CTRSDADC_R {
        CTRSDADC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    pub fn onchop(&self) -> ONCHOP_R {
        ONCHOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    pub fn buftest(&self) -> BUFTEST_R {
        BUFTEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    pub fn ctrsdadc(&mut self) -> CTRSDADC_W {
        CTRSDADC_W { w: self }
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    pub fn onchop(&mut self) -> ONCHOP_W {
        ONCHOP_W { w: self }
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    pub fn buftest(&mut self) -> BUFTEST_W {
        BUFTEST_W { w: self }
    }
}

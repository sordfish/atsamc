#[doc = "Reader of register CFGA"]
pub type R = crate::R<u16, super::CFGA>;
#[doc = "Writer for register CFGA"]
pub type W = crate::W<u16, super::CFGA>;
#[doc = "Register CFGA `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFNUM`"]
pub type REFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFNUM`"]
pub struct REFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DIVREF`"]
pub type DIVREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVREF`"]
pub struct DIVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Divide Reference Clock"]
    #[inline(always)]
    pub fn divref(&self) -> DIVREF_R {
        DIVREF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&mut self) -> REFNUM_W {
        REFNUM_W { w: self }
    }
    #[doc = "Bit 15 - Divide Reference Clock"]
    #[inline(always)]
    pub fn divref(&mut self) -> DIVREF_W {
        DIVREF_W { w: self }
    }
}

#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u8, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u8, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIGNED`"]
pub type SIGNED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGNED`"]
pub struct SIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNED_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DLZ`"]
pub type DLZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLZ`"]
pub struct DLZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DLZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Signed"]
    #[inline(always)]
    pub fn signed(&self) -> SIGNED_R {
        SIGNED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable Leading Zero Optimization"]
    #[inline(always)]
    pub fn dlz(&self) -> DLZ_R {
        DLZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Signed"]
    #[inline(always)]
    pub fn signed(&mut self) -> SIGNED_W {
        SIGNED_W { w: self }
    }
    #[doc = "Bit 1 - Disable Leading Zero Optimization"]
    #[inline(always)]
    pub fn dlz(&mut self) -> DLZ_W {
        DLZ_W { w: self }
    }
}

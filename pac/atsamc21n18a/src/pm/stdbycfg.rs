#[doc = "Reader of register STDBYCFG"]
pub type R = crate::R<u16, super::STDBYCFG>;
#[doc = "Writer for register STDBYCFG"]
pub type W = crate::W<u16, super::STDBYCFG>;
#[doc = "Register STDBYCFG `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::STDBYCFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Voltage Regulator Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREGSMOD_A {
    #[doc = "0: Automatic mode"]
    AUTO = 0,
    #[doc = "1: Performance oriented"]
    PERFORMANCE = 1,
    #[doc = "2: Low Power oriented"]
    LP = 2,
}
impl From<VREGSMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: VREGSMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VREGSMOD`"]
pub type VREGSMOD_R = crate::R<u8, VREGSMOD_A>;
impl VREGSMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VREGSMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VREGSMOD_A::AUTO),
            1 => Val(VREGSMOD_A::PERFORMANCE),
            2 => Val(VREGSMOD_A::LP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMOD_A::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMOD_A::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMOD_A::LP
    }
}
#[doc = "Write proxy for field `VREGSMOD`"]
pub struct VREGSMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGSMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREGSMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMOD_A::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMOD_A::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMOD_A::LP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BBIASHS`"]
pub type BBIASHS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BBIASHS`"]
pub struct BBIASHS_W<'a> {
    w: &'a mut W,
}
impl<'a> BBIASHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VREGSMOD_R {
        VREGSMOD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BBIASHS_R {
        BBIASHS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&mut self) -> VREGSMOD_W {
        VREGSMOD_W { w: self }
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    pub fn bbiashs(&mut self) -> BBIASHS_W {
        BBIASHS_W { w: self }
    }
}

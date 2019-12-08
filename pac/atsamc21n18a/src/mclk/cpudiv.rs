#[doc = "Reader of register CPUDIV"]
pub type R = crate::R<u8, super::CPUDIV>;
#[doc = "Writer for register CPUDIV"]
pub type W = crate::W<u8, super::CPUDIV>;
#[doc = "Register CPUDIV `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CPUDIV {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "CPU Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUDIV_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
    #[doc = "2: Divide by 2"]
    DIV2 = 2,
    #[doc = "4: Divide by 4"]
    DIV4 = 4,
    #[doc = "8: Divide by 8"]
    DIV8 = 8,
    #[doc = "16: Divide by 16"]
    DIV16 = 16,
    #[doc = "32: Divide by 32"]
    DIV32 = 32,
    #[doc = "64: Divide by 64"]
    DIV64 = 64,
    #[doc = "128: Divide by 128"]
    DIV128 = 128,
}
impl From<CPUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPUDIV`"]
pub type CPUDIV_R = crate::R<u8, CPUDIV_A>;
impl CPUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPUDIV_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPUDIV_A::DIV1),
            2 => Val(CPUDIV_A::DIV2),
            4 => Val(CPUDIV_A::DIV4),
            8 => Val(CPUDIV_A::DIV8),
            16 => Val(CPUDIV_A::DIV16),
            32 => Val(CPUDIV_A::DIV32),
            64 => Val(CPUDIV_A::DIV64),
            128 => Val(CPUDIV_A::DIV128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CPUDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CPUDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CPUDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CPUDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CPUDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CPUDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CPUDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CPUDIV_A::DIV128
    }
}
#[doc = "Write proxy for field `CPUDIV`"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
}

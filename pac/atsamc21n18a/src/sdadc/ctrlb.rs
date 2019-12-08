#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u16, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u16, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CTRLB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Peripheral clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: Peripheral clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: Peripheral clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: Peripheral clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: Peripheral clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: Peripheral clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: Peripheral clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: Peripheral clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALER_A::DIV2),
            1 => Val(PRESCALER_A::DIV4),
            2 => Val(PRESCALER_A::DIV8),
            3 => Val(PRESCALER_A::DIV16),
            4 => Val(PRESCALER_A::DIV32),
            5 => Val(PRESCALER_A::DIV64),
            6 => Val(PRESCALER_A::DIV128),
            7 => Val(PRESCALER_A::DIV256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Over Sampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: Over Sampling Ratio is 64"]
    OSR64 = 0,
    #[doc = "1: Over Sampling Ratio is 128"]
    OSR128 = 1,
    #[doc = "2: Over Sampling Ratio is 256"]
    OSR256 = 2,
    #[doc = "3: Over Sampling Ratio is 512"]
    OSR512 = 3,
    #[doc = "4: Over Sampling Ratio is 1024"]
    OSR1024 = 4,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, OSR_A>;
impl OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSR_A::OSR64),
            1 => Val(OSR_A::OSR128),
            2 => Val(OSR_A::OSR256),
            3 => Val(OSR_A::OSR512),
            4 => Val(OSR_A::OSR1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == OSR_A::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR128`"]
    #[inline(always)]
    pub fn is_osr128(&self) -> bool {
        *self == OSR_A::OSR128
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == OSR_A::OSR256
    }
    #[doc = "Checks if the value of the field is `OSR512`"]
    #[inline(always)]
    pub fn is_osr512(&self) -> bool {
        *self == OSR_A::OSR512
    }
    #[doc = "Checks if the value of the field is `OSR1024`"]
    #[inline(always)]
    pub fn is_osr1024(&self) -> bool {
        *self == OSR_A::OSR1024
    }
}
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Over Sampling Ratio is 64"]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut W {
        self.variant(OSR_A::OSR64)
    }
    #[doc = "Over Sampling Ratio is 128"]
    #[inline(always)]
    pub fn osr128(self) -> &'a mut W {
        self.variant(OSR_A::OSR128)
    }
    #[doc = "Over Sampling Ratio is 256"]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut W {
        self.variant(OSR_A::OSR256)
    }
    #[doc = "Over Sampling Ratio is 512"]
    #[inline(always)]
    pub fn osr512(self) -> &'a mut W {
        self.variant(OSR_A::OSR512)
    }
    #[doc = "Over Sampling Ratio is 1024"]
    #[inline(always)]
    pub fn osr1024(self) -> &'a mut W {
        self.variant(OSR_A::OSR1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SKPCNT`"]
pub type SKPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SKPCNT`"]
pub struct SKPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SKPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Skip Sample Count"]
    #[inline(always)]
    pub fn skpcnt(&self) -> SKPCNT_R {
        SKPCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bits 8:10 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bits 12:15 - Skip Sample Count"]
    #[inline(always)]
    pub fn skpcnt(&mut self) -> SKPCNT_W {
        SKPCNT_W { w: self }
    }
}

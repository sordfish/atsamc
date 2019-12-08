#[doc = "Reader of register XOSCCTRL"]
pub type R = crate::R<u16, super::XOSCCTRL>;
#[doc = "Writer for register XOSCCTRL"]
pub type W = crate::W<u16, super::XOSCCTRL>;
#[doc = "Register XOSCCTRL `reset()`'s with value 0x80"]
impl crate::ResetValue for super::XOSCCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `XTALEN`"]
pub type XTALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTALEN`"]
pub struct XTALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CFDEN`"]
pub type CFDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEN`"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SWBEN`"]
pub type SWBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWBEN`"]
pub struct SWBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ONDEMAND`"]
pub type ONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONDEMAND`"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
    #[doc = "0: 2 MHz"]
    GAIN2 = 0,
    #[doc = "1: 4 MHz"]
    GAIN4 = 1,
    #[doc = "2: 8 MHz"]
    GAIN8 = 2,
    #[doc = "3: 16 MHz"]
    GAIN16 = 3,
    #[doc = "4: 30 MHz"]
    GAIN30 = 4,
}
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, GAIN_A>;
impl GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN_A::GAIN2),
            1 => Val(GAIN_A::GAIN4),
            2 => Val(GAIN_A::GAIN8),
            3 => Val(GAIN_A::GAIN16),
            4 => Val(GAIN_A::GAIN30),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == GAIN_A::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == GAIN_A::GAIN4
    }
    #[doc = "Checks if the value of the field is `GAIN8`"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == GAIN_A::GAIN8
    }
    #[doc = "Checks if the value of the field is `GAIN16`"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == GAIN_A::GAIN16
    }
    #[doc = "Checks if the value of the field is `GAIN30`"]
    #[inline(always)]
    pub fn is_gain30(&self) -> bool {
        *self == GAIN_A::GAIN30
    }
}
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN2)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN4)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN8)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN16)
    }
    #[doc = "30 MHz"]
    #[inline(always)]
    pub fn gain30(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN30)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `AMPGC`"]
pub type AMPGC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPGC`"]
pub struct AMPGC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPGC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 31 us"]
    CYCLE1 = 0,
    #[doc = "1: 61 us"]
    CYCLE2 = 1,
    #[doc = "2: 122 us"]
    CYCLE4 = 2,
    #[doc = "3: 244 us"]
    CYCLE8 = 3,
    #[doc = "4: 488 us"]
    CYCLE16 = 4,
    #[doc = "5: 977 us"]
    CYCLE32 = 5,
    #[doc = "6: 1953 us"]
    CYCLE64 = 6,
    #[doc = "7: 3906 us"]
    CYCLE128 = 7,
    #[doc = "8: 7813 us"]
    CYCLE256 = 8,
    #[doc = "9: 15625 us"]
    CYCLE512 = 9,
    #[doc = "10: 31250 us"]
    CYCLE1024 = 10,
    #[doc = "11: 62500 us"]
    CYCLE2048 = 11,
    #[doc = "12: 125000 us"]
    CYCLE4096 = 12,
    #[doc = "13: 250000 us"]
    CYCLE8192 = 13,
    #[doc = "14: 500000 us"]
    CYCLE16384 = 14,
    #[doc = "15: 1000000 us"]
    CYCLE32768 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u8, STARTUP_A>;
impl STARTUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::CYCLE1,
            1 => STARTUP_A::CYCLE2,
            2 => STARTUP_A::CYCLE4,
            3 => STARTUP_A::CYCLE8,
            4 => STARTUP_A::CYCLE16,
            5 => STARTUP_A::CYCLE32,
            6 => STARTUP_A::CYCLE64,
            7 => STARTUP_A::CYCLE128,
            8 => STARTUP_A::CYCLE256,
            9 => STARTUP_A::CYCLE512,
            10 => STARTUP_A::CYCLE1024,
            11 => STARTUP_A::CYCLE2048,
            12 => STARTUP_A::CYCLE4096,
            13 => STARTUP_A::CYCLE8192,
            14 => STARTUP_A::CYCLE16384,
            15 => STARTUP_A::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE1`"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUP_A::CYCLE1
    }
    #[doc = "Checks if the value of the field is `CYCLE2`"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == STARTUP_A::CYCLE2
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUP_A::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE8`"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == STARTUP_A::CYCLE8
    }
    #[doc = "Checks if the value of the field is `CYCLE16`"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == STARTUP_A::CYCLE16
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUP_A::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE64`"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == STARTUP_A::CYCLE64
    }
    #[doc = "Checks if the value of the field is `CYCLE128`"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == STARTUP_A::CYCLE128
    }
    #[doc = "Checks if the value of the field is `CYCLE256`"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == STARTUP_A::CYCLE256
    }
    #[doc = "Checks if the value of the field is `CYCLE512`"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == STARTUP_A::CYCLE512
    }
    #[doc = "Checks if the value of the field is `CYCLE1024`"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == STARTUP_A::CYCLE1024
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUP_A::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUP_A::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE8192`"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == STARTUP_A::CYCLE8192
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUP_A::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUP_A::CYCLE32768
    }
}
#[doc = "Write proxy for field `STARTUP`"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SWBEN_R {
        SWBEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XTALEN_W {
        XTALEN_W { w: self }
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&mut self) -> SWBEN_W {
        SWBEN_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&mut self) -> AMPGC_W {
        AMPGC_W { w: self }
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
}

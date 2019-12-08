#[doc = "Reader of register OSC32K"]
pub type R = crate::R<u32, super::OSC32K>;
#[doc = "Writer for register OSC32K"]
pub type W = crate::W<u32, super::OSC32K>;
#[doc = "Register OSC32K `reset()`'s with value 0x003f_0080"]
impl crate::ResetValue for super::OSC32K {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003f_0080
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN32K`"]
pub type EN32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN32K`"]
pub struct EN32K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EN1K`"]
pub type EN1K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN1K`"]
pub struct EN1K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0.092 ms"]
    CYCLE3 = 0,
    #[doc = "1: 0.122 ms"]
    CYCLE4 = 1,
    #[doc = "2: 0.183 ms"]
    CYCLE6 = 2,
    #[doc = "3: 0.305 ms"]
    CYCLE10 = 3,
    #[doc = "4: 0.549 ms"]
    CYCLE18 = 4,
    #[doc = "5: 1.038 ms"]
    CYCLE34 = 5,
    #[doc = "6: 2.014 ms"]
    CYCLE66 = 6,
    #[doc = "7: 3.967 ms"]
    CYCLE130 = 7,
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
            0 => STARTUP_A::CYCLE3,
            1 => STARTUP_A::CYCLE4,
            2 => STARTUP_A::CYCLE6,
            3 => STARTUP_A::CYCLE10,
            4 => STARTUP_A::CYCLE18,
            5 => STARTUP_A::CYCLE34,
            6 => STARTUP_A::CYCLE66,
            7 => STARTUP_A::CYCLE130,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE3`"]
    #[inline(always)]
    pub fn is_cycle3(&self) -> bool {
        *self == STARTUP_A::CYCLE3
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUP_A::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE6`"]
    #[inline(always)]
    pub fn is_cycle6(&self) -> bool {
        *self == STARTUP_A::CYCLE6
    }
    #[doc = "Checks if the value of the field is `CYCLE10`"]
    #[inline(always)]
    pub fn is_cycle10(&self) -> bool {
        *self == STARTUP_A::CYCLE10
    }
    #[doc = "Checks if the value of the field is `CYCLE18`"]
    #[inline(always)]
    pub fn is_cycle18(&self) -> bool {
        *self == STARTUP_A::CYCLE18
    }
    #[doc = "Checks if the value of the field is `CYCLE34`"]
    #[inline(always)]
    pub fn is_cycle34(&self) -> bool {
        *self == STARTUP_A::CYCLE34
    }
    #[doc = "Checks if the value of the field is `CYCLE66`"]
    #[inline(always)]
    pub fn is_cycle66(&self) -> bool {
        *self == STARTUP_A::CYCLE66
    }
    #[doc = "Checks if the value of the field is `CYCLE130`"]
    #[inline(always)]
    pub fn is_cycle130(&self) -> bool {
        *self == STARTUP_A::CYCLE130
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
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn cycle3(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE3)
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4)
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn cycle6(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE6)
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn cycle10(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE10)
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn cycle18(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE18)
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn cycle34(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE34)
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn cycle66(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE66)
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn cycle130(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE130)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `WRTLOCK`"]
pub type WRTLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRTLOCK`"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CALIB`"]
pub type CALIB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALIB`"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&mut self) -> EN1K_W {
        EN1K_W { w: self }
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
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
}

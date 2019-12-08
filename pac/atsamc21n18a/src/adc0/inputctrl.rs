#[doc = "Reader of register INPUTCTRL"]
pub type R = crate::R<u16, super::INPUTCTRL>;
#[doc = "Writer for register INPUTCTRL"]
pub type W = crate::W<u16, super::INPUTCTRL>;
#[doc = "Register INPUTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    AIN6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    AIN7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    AIN8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    AIN9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    AIN10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    AIN11 = 11,
    #[doc = "24: Temperature Sensor"]
    TEMP = 24,
    #[doc = "25: Bandgap Voltage"]
    BANDGAP = 25,
    #[doc = "26: 1/4 Scaled Core Supply"]
    SCALEDCOREVCC = 26,
    #[doc = "27: 1/4 Scaled I/O Supply"]
    SCALEDIOVCC = 27,
    #[doc = "28: DAC Output"]
    DAC = 28,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUXPOS`"]
pub type MUXPOS_R = crate::R<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXPOS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXPOS_A::AIN0),
            1 => Val(MUXPOS_A::AIN1),
            2 => Val(MUXPOS_A::AIN2),
            3 => Val(MUXPOS_A::AIN3),
            4 => Val(MUXPOS_A::AIN4),
            5 => Val(MUXPOS_A::AIN5),
            6 => Val(MUXPOS_A::AIN6),
            7 => Val(MUXPOS_A::AIN7),
            8 => Val(MUXPOS_A::AIN8),
            9 => Val(MUXPOS_A::AIN9),
            10 => Val(MUXPOS_A::AIN10),
            11 => Val(MUXPOS_A::AIN11),
            24 => Val(MUXPOS_A::TEMP),
            25 => Val(MUXPOS_A::BANDGAP),
            26 => Val(MUXPOS_A::SCALEDCOREVCC),
            27 => Val(MUXPOS_A::SCALEDIOVCC),
            28 => Val(MUXPOS_A::DAC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOS_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOS_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOS_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOS_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOS_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOS_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOS_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOS_A::AIN7
    }
    #[doc = "Checks if the value of the field is `AIN8`"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOS_A::AIN8
    }
    #[doc = "Checks if the value of the field is `AIN9`"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOS_A::AIN9
    }
    #[doc = "Checks if the value of the field is `AIN10`"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOS_A::AIN10
    }
    #[doc = "Checks if the value of the field is `AIN11`"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOS_A::AIN11
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == MUXPOS_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXPOS_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `SCALEDCOREVCC`"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDCOREVCC
    }
    #[doc = "Checks if the value of the field is `SCALEDIOVCC`"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == MUXPOS_A::SCALEDIOVCC
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == MUXPOS_A::DAC
    }
}
#[doc = "Write proxy for field `MUXPOS`"]
pub struct MUXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXPOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN11)
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(MUXPOS_A::TEMP)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXPOS_A::BANDGAP)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDCOREVCC)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut W {
        self.variant(MUXPOS_A::SCALEDIOVCC)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(MUXPOS_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: ADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    AIN2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    AIN3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    AIN4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    AIN5 = 5,
    #[doc = "24: Internal Ground"]
    GND = 24,
}
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUXNEG`"]
pub type MUXNEG_R = crate::R<u8, MUXNEG_A>;
impl MUXNEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXNEG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXNEG_A::AIN0),
            1 => Val(MUXNEG_A::AIN1),
            2 => Val(MUXNEG_A::AIN2),
            3 => Val(MUXNEG_A::AIN3),
            4 => Val(MUXNEG_A::AIN4),
            5 => Val(MUXNEG_A::AIN5),
            24 => Val(MUXNEG_A::GND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXNEG_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXNEG_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXNEG_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXNEG_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXNEG_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXNEG_A::AIN5
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEG_A::GND
    }
}
#[doc = "Write proxy for field `MUXNEG`"]
pub struct MUXNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXNEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXNEG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXNEG_A::AIN5)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W { w: self }
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W { w: self }
    }
}

#[doc = "Reader of register INPUTCTRL"]
pub type R = crate::R<u8, super::INPUTCTRL>;
#[doc = "Writer for register INPUTCTRL"]
pub type W = crate::W<u8, super::INPUTCTRL>;
#[doc = "Register INPUTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDADC Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXSEL_A {
    #[doc = "0: SDADC AIN0 Pin"]
    AIN0 = 0,
    #[doc = "1: SDADC AIN1 Pin"]
    AIN1 = 1,
    #[doc = "2: SDADC AIN2 Pin"]
    AIN2 = 2,
}
impl From<MUXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUXSEL`"]
pub type MUXSEL_R = crate::R<u8, MUXSEL_A>;
impl MUXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUXSEL_A::AIN0),
            1 => Val(MUXSEL_A::AIN1),
            2 => Val(MUXSEL_A::AIN2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXSEL_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXSEL_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXSEL_A::AIN2
    }
}
#[doc = "Write proxy for field `MUXSEL`"]
pub struct MUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXSEL_A::AIN0)
    }
    #[doc = "SDADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXSEL_A::AIN1)
    }
    #[doc = "SDADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXSEL_A::AIN2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SDADC Input Selection"]
    #[inline(always)]
    pub fn muxsel(&self) -> MUXSEL_R {
        MUXSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDADC Input Selection"]
    #[inline(always)]
    pub fn muxsel(&mut self) -> MUXSEL_W {
        MUXSEL_W { w: self }
    }
}

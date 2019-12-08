#[doc = "Reader of register CTRLC"]
pub type R = crate::R<u8, super::CTRLC>;
#[doc = "Writer for register CTRLC"]
pub type W = crate::W<u8, super::CTRLC>;
#[doc = "Register CTRLC `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINMODE_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: VALUE greater than WINLT"]
    ABOVE = 1,
    #[doc = "2: VALUE less than WINUT"]
    BELOW = 2,
    #[doc = "3: VALUE greater than WINLT and VALUE less than WINUT"]
    INSIDE = 3,
    #[doc = "4: VALUE less than WINLT or VALUE greater than WINUT"]
    OUTSIDE = 4,
    #[doc = "5: VALUE greater than WINUT with hysteresis to WINLT"]
    HYST_ABOVE = 5,
    #[doc = "6: VALUE less than WINLST with hysteresis to WINUT"]
    HYST_BELOW = 6,
}
impl From<WINMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WINMODE`"]
pub type WINMODE_R = crate::R<u8, WINMODE_A>;
impl WINMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WINMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WINMODE_A::DISABLE),
            1 => Val(WINMODE_A::ABOVE),
            2 => Val(WINMODE_A::BELOW),
            3 => Val(WINMODE_A::INSIDE),
            4 => Val(WINMODE_A::OUTSIDE),
            5 => Val(WINMODE_A::HYST_ABOVE),
            6 => Val(WINMODE_A::HYST_BELOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WINMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINMODE_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINMODE_A::BELOW
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINMODE_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINMODE_A::OUTSIDE
    }
    #[doc = "Checks if the value of the field is `HYST_ABOVE`"]
    #[inline(always)]
    pub fn is_hyst_above(&self) -> bool {
        *self == WINMODE_A::HYST_ABOVE
    }
    #[doc = "Checks if the value of the field is `HYST_BELOW`"]
    #[inline(always)]
    pub fn is_hyst_below(&self) -> bool {
        *self == WINMODE_A::HYST_BELOW
    }
}
#[doc = "Write proxy for field `WINMODE`"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODE_A::DISABLE)
    }
    #[doc = "VALUE greater than WINLT"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINMODE_A::ABOVE)
    }
    #[doc = "VALUE less than WINUT"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINMODE_A::BELOW)
    }
    #[doc = "VALUE greater than WINLT and VALUE less than WINUT"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINMODE_A::INSIDE)
    }
    #[doc = "VALUE less than WINLT or VALUE greater than WINUT"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINMODE_A::OUTSIDE)
    }
    #[doc = "VALUE greater than WINUT with hysteresis to WINLT"]
    #[inline(always)]
    pub fn hyst_above(self) -> &'a mut W {
        self.variant(WINMODE_A::HYST_ABOVE)
    }
    #[doc = "VALUE less than WINLST with hysteresis to WINUT"]
    #[inline(always)]
    pub fn hyst_below(self) -> &'a mut W {
        self.variant(WINMODE_A::HYST_BELOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FREERUN`"]
pub type FREERUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREERUN`"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
}

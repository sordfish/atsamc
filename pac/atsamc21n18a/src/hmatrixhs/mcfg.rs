#[doc = "Reader of register MCFG[%s]"]
pub type R = crate::R<u32, super::MCFG>;
#[doc = "Writer for register MCFG[%s]"]
pub type W = crate::W<u32, super::MCFG>;
#[doc = "Register MCFG[%s] `reset()`'s with value 0x02"]
impl crate::ResetValue for super::MCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Undefined Length Burst Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ULBT_A {
    #[doc = "0: Infinite Length"]
    INFINITE = 0,
    #[doc = "1: Single Access"]
    SINGLE = 1,
    #[doc = "2: Four Beat Burst"]
    FOUR_BEAT = 2,
    #[doc = "3: Eight Beat Burst"]
    EIGHT_BEAT = 3,
    #[doc = "4: Sixteen Beat Burst"]
    SIXTEEN_BEAT = 4,
}
impl From<ULBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ULBT`"]
pub type ULBT_R = crate::R<u8, ULBT_A>;
impl ULBT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ULBT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ULBT_A::INFINITE),
            1 => Val(ULBT_A::SINGLE),
            2 => Val(ULBT_A::FOUR_BEAT),
            3 => Val(ULBT_A::EIGHT_BEAT),
            4 => Val(ULBT_A::SIXTEEN_BEAT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INFINITE`"]
    #[inline(always)]
    pub fn is_infinite(&self) -> bool {
        *self == ULBT_A::INFINITE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ULBT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR_BEAT`"]
    #[inline(always)]
    pub fn is_four_beat(&self) -> bool {
        *self == ULBT_A::FOUR_BEAT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BEAT`"]
    #[inline(always)]
    pub fn is_eight_beat(&self) -> bool {
        *self == ULBT_A::EIGHT_BEAT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BEAT`"]
    #[inline(always)]
    pub fn is_sixteen_beat(&self) -> bool {
        *self == ULBT_A::SIXTEEN_BEAT
    }
}
#[doc = "Write proxy for field `ULBT`"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULBT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Infinite Length"]
    #[inline(always)]
    pub fn infinite(self) -> &'a mut W {
        self.variant(ULBT_A::INFINITE)
    }
    #[doc = "Single Access"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBT_A::SINGLE)
    }
    #[doc = "Four Beat Burst"]
    #[inline(always)]
    pub fn four_beat(self) -> &'a mut W {
        self.variant(ULBT_A::FOUR_BEAT)
    }
    #[doc = "Eight Beat Burst"]
    #[inline(always)]
    pub fn eight_beat(self) -> &'a mut W {
        self.variant(ULBT_A::EIGHT_BEAT)
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline(always)]
    pub fn sixteen_beat(self) -> &'a mut W {
        self.variant(ULBT_A::SIXTEEN_BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
}

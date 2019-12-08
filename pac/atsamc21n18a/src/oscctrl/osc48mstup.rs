#[doc = "Reader of register OSC48MSTUP"]
pub type R = crate::R<u8, super::OSC48MSTUP>;
#[doc = "Writer for register OSC48MSTUP"]
pub type W = crate::W<u8, super::OSC48MSTUP>;
#[doc = "Register OSC48MSTUP `reset()`'s with value 0x07"]
impl crate::ResetValue for super::OSC48MSTUP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Startup Time\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 166 ns"]
    CYCLE8 = 0,
    #[doc = "1: 333 ns"]
    CYCLE16 = 1,
    #[doc = "2: 667 ns"]
    CYCLE32 = 2,
    #[doc = "3: 1.333 us"]
    CYCLE64 = 3,
    #[doc = "4: 2.667 us"]
    CYCLE128 = 4,
    #[doc = "5: 5.333 us"]
    CYCLE256 = 5,
    #[doc = "6: 10.667 us"]
    CYCLE512 = 6,
    #[doc = "7: 21.333 us"]
    CYCLE1024 = 7,
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
            0 => STARTUP_A::CYCLE8,
            1 => STARTUP_A::CYCLE16,
            2 => STARTUP_A::CYCLE32,
            3 => STARTUP_A::CYCLE64,
            4 => STARTUP_A::CYCLE128,
            5 => STARTUP_A::CYCLE256,
            6 => STARTUP_A::CYCLE512,
            7 => STARTUP_A::CYCLE1024,
            _ => unreachable!(),
        }
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
    #[doc = "166 ns"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8)
    }
    #[doc = "333 ns"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16)
    }
    #[doc = "667 ns"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "1.333 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE64)
    }
    #[doc = "2.667 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE128)
    }
    #[doc = "5.333 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE256)
    }
    #[doc = "10.667 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE512)
    }
    #[doc = "21.333 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
}

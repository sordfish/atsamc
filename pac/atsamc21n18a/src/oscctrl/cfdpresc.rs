#[doc = "Reader of register CFDPRESC"]
pub type R = crate::R<u8, super::CFDPRESC>;
#[doc = "Writer for register CFDPRESC"]
pub type W = crate::W<u8, super::CFDPRESC>;
#[doc = "Register CFDPRESC `reset()`'s with value 0"]
impl crate::ResetValue for super::CFDPRESC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFDPRESC_A {
    #[doc = "0: 48 MHz"]
    DIV1 = 0,
    #[doc = "1: 24 MHz"]
    DIV2 = 1,
    #[doc = "2: 12 MHz"]
    DIV4 = 2,
    #[doc = "3: 6 MHz"]
    DIV8 = 3,
    #[doc = "4: 3 MHz"]
    DIV16 = 4,
    #[doc = "5: 1.5 MHz"]
    DIV32 = 5,
    #[doc = "6: 0.75 MHz"]
    DIV64 = 6,
    #[doc = "7: 0.3125 MHz"]
    DIV128 = 7,
}
impl From<CFDPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFDPRESC`"]
pub type CFDPRESC_R = crate::R<u8, CFDPRESC_A>;
impl CFDPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDPRESC_A {
        match self.bits {
            0 => CFDPRESC_A::DIV1,
            1 => CFDPRESC_A::DIV2,
            2 => CFDPRESC_A::DIV4,
            3 => CFDPRESC_A::DIV8,
            4 => CFDPRESC_A::DIV16,
            5 => CFDPRESC_A::DIV32,
            6 => CFDPRESC_A::DIV64,
            7 => CFDPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CFDPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CFDPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CFDPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CFDPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CFDPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CFDPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CFDPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CFDPRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `CFDPRESC`"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFDPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
}

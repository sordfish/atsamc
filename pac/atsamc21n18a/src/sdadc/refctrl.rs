#[doc = "Reader of register REFCTRL"]
pub type R = crate::R<u8, super::REFCTRL>;
#[doc = "Writer for register REFCTRL"]
pub type W = crate::W<u8, super::REFCTRL>;
#[doc = "Register REFCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::REFCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal Bandgap Reference"]
    INTREF = 0,
    #[doc = "1: External Reference"]
    AREFB = 1,
    #[doc = "2: Internal DAC Output"]
    DAC = 2,
    #[doc = "3: VDDANA"]
    INTVCC = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::INTREF,
            1 => REFSEL_A::AREFB,
            2 => REFSEL_A::DAC,
            3 => REFSEL_A::INTVCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSEL_A::INTREF
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == REFSEL_A::AREFB
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == REFSEL_A::DAC
    }
    #[doc = "Checks if the value of the field is `INTVCC`"]
    #[inline(always)]
    pub fn is_intvcc(&self) -> bool {
        *self == REFSEL_A::INTVCC
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFB)
    }
    #[doc = "Internal DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(REFSEL_A::DAC)
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn intvcc(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `REFRANGE`"]
pub type REFRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRANGE`"]
pub struct REFRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ONREFBUF`"]
pub type ONREFBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONREFBUF`"]
pub struct ONREFBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> ONREFBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    pub fn refrange(&self) -> REFRANGE_R {
        REFRANGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    pub fn onrefbuf(&self) -> ONREFBUF_R {
        ONREFBUF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    pub fn refrange(&mut self) -> REFRANGE_W {
        REFRANGE_W { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    pub fn onrefbuf(&mut self) -> ONREFBUF_W {
        ONREFBUF_W { w: self }
    }
}

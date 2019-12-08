#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCRDY`"]
pub struct XOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `XOSCFAIL`"]
pub type XOSCFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCFAIL`"]
pub struct XOSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL_W<'a> {
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
#[doc = "Reader of field `OSC48MRDY`"]
pub type OSC48MRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC48MRDY`"]
pub struct OSC48MRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC48MRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKR`"]
pub struct DPLLLCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKF`"]
pub struct DPLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DPLLLTO`"]
pub type DPLLLTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLTO`"]
pub struct DPLLLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPLLLDRTO`"]
pub type DPLLLDRTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLDRTO`"]
pub struct DPLLLDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLDRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> OSC48MRDY_R {
        OSC48MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DPLL Time Out Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W {
        XOSCRDY_W { w: self }
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XOSCFAIL_W {
        XOSCFAIL_W { w: self }
    }
    #[doc = "Bit 4 - OSC48M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc48mrdy(&mut self) -> OSC48MRDY_W {
        OSC48MRDY_W { w: self }
    }
    #[doc = "Bit 8 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W {
        DPLLLCKR_W { w: self }
    }
    #[doc = "Bit 9 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W {
        DPLLLCKF_W { w: self }
    }
    #[doc = "Bit 10 - DPLL Time Out Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&mut self) -> DPLLLTO_W {
        DPLLLTO_W { w: self }
    }
    #[doc = "Bit 11 - DPLL Ratio Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dpllldrto(&mut self) -> DPLLLDRTO_W {
        DPLLLDRTO_W { w: self }
    }
}

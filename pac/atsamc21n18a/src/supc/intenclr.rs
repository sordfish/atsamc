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
#[doc = "Reader of field `BODVDDRDY`"]
pub type BODVDDRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODVDDRDY`"]
pub struct BODVDDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVDDRDY_W<'a> {
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
#[doc = "Reader of field `BODVDDDET`"]
pub type BODVDDDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODVDDDET`"]
pub struct BODVDDDET_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVDDDET_W<'a> {
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
#[doc = "Reader of field `BVDDSRDY`"]
pub type BVDDSRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVDDSRDY`"]
pub struct BVDDSRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BVDDSRDY_W<'a> {
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
#[doc = "Reader of field `VREG33RDY`"]
pub type VREG33RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREG33RDY`"]
pub struct VREG33RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG33RDY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BODVDDRDY_R {
        BODVDDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&self) -> BODVDDDET_R {
        BODVDDDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&self) -> BVDDSRDY_R {
        BVDDSRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&self) -> VREG33RDY_R {
        VREG33RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&mut self) -> BODVDDRDY_W {
        BODVDDRDY_W { w: self }
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&mut self) -> BODVDDDET_W {
        BODVDDDET_W { w: self }
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&mut self) -> BVDDSRDY_W {
        BVDDSRDY_W { w: self }
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&mut self) -> VREG33RDY_W {
        VREG33RDY_W { w: self }
    }
}

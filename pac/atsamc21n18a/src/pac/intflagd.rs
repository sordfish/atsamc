#[doc = "Reader of register INTFLAGD"]
pub type R = crate::R<u32, super::INTFLAGD>;
#[doc = "Writer for register INTFLAGD"]
pub type W = crate::W<u32, super::INTFLAGD>;
#[doc = "Register INTFLAGD `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAGD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERCOM6_`"]
pub type SERCOM6__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM6_`"]
pub struct SERCOM6__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM6__W<'a> {
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
#[doc = "Reader of field `SERCOM7_`"]
pub type SERCOM7__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM7_`"]
pub struct SERCOM7__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM7__W<'a> {
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
#[doc = "Reader of field `TC5_`"]
pub type TC5__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC5_`"]
pub struct TC5__W<'a> {
    w: &'a mut W,
}
impl<'a> TC5__W<'a> {
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
#[doc = "Reader of field `TC6_`"]
pub type TC6__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC6_`"]
pub struct TC6__W<'a> {
    w: &'a mut W,
}
impl<'a> TC6__W<'a> {
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
#[doc = "Reader of field `TC7_`"]
pub type TC7__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC7_`"]
pub struct TC7__W<'a> {
    w: &'a mut W,
}
impl<'a> TC7__W<'a> {
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
impl R {
    #[doc = "Bit 0 - SERCOM6"]
    #[inline(always)]
    pub fn sercom6_(&self) -> SERCOM6__R {
        SERCOM6__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM7"]
    #[inline(always)]
    pub fn sercom7_(&self) -> SERCOM7__R {
        SERCOM7__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC5"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC6"]
    #[inline(always)]
    pub fn tc6_(&self) -> TC6__R {
        TC6__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC7"]
    #[inline(always)]
    pub fn tc7_(&self) -> TC7__R {
        TC7__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM6"]
    #[inline(always)]
    pub fn sercom6_(&mut self) -> SERCOM6__W {
        SERCOM6__W { w: self }
    }
    #[doc = "Bit 1 - SERCOM7"]
    #[inline(always)]
    pub fn sercom7_(&mut self) -> SERCOM7__W {
        SERCOM7__W { w: self }
    }
    #[doc = "Bit 2 - TC5"]
    #[inline(always)]
    pub fn tc5_(&mut self) -> TC5__W {
        TC5__W { w: self }
    }
    #[doc = "Bit 3 - TC6"]
    #[inline(always)]
    pub fn tc6_(&mut self) -> TC6__W {
        TC6__W { w: self }
    }
    #[doc = "Bit 4 - TC7"]
    #[inline(always)]
    pub fn tc7_(&mut self) -> TC7__W {
        TC7__W { w: self }
    }
}

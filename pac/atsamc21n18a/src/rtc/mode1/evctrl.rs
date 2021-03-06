#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEREO0`"]
pub type PEREO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO0`"]
pub struct PEREO0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO0_W<'a> {
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
#[doc = "Reader of field `PEREO1`"]
pub type PEREO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO1`"]
pub struct PEREO1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO1_W<'a> {
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
#[doc = "Reader of field `PEREO2`"]
pub type PEREO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO2`"]
pub struct PEREO2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO2_W<'a> {
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
#[doc = "Reader of field `PEREO3`"]
pub type PEREO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO3`"]
pub struct PEREO3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO3_W<'a> {
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
#[doc = "Reader of field `PEREO4`"]
pub type PEREO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO4`"]
pub struct PEREO4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO4_W<'a> {
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
#[doc = "Reader of field `PEREO5`"]
pub type PEREO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO5`"]
pub struct PEREO5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PEREO6`"]
pub type PEREO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO6`"]
pub struct PEREO6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO6_W<'a> {
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
#[doc = "Reader of field `PEREO7`"]
pub type PEREO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREO7`"]
pub struct PEREO7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREO7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CMPEO0`"]
pub type CMPEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEO0`"]
pub struct CMPEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO0_W<'a> {
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
#[doc = "Reader of field `CMPEO1`"]
pub type CMPEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEO1`"]
pub struct CMPEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEO1_W<'a> {
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
#[doc = "Reader of field `OVFEO`"]
pub type OVFEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVFEO`"]
pub struct OVFEO_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&self) -> PEREO0_R {
        PEREO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&self) -> PEREO1_R {
        PEREO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&self) -> PEREO2_R {
        PEREO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&self) -> PEREO3_R {
        PEREO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&self) -> PEREO4_R {
        PEREO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&self) -> PEREO5_R {
        PEREO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&self) -> PEREO6_R {
        PEREO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&self) -> PEREO7_R {
        PEREO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo0(&self) -> CMPEO0_R {
        CMPEO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo1(&self) -> CMPEO1_R {
        CMPEO1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&mut self) -> PEREO0_W {
        PEREO0_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&mut self) -> PEREO1_W {
        PEREO1_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&mut self) -> PEREO2_W {
        PEREO2_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&mut self) -> PEREO3_W {
        PEREO3_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&mut self) -> PEREO4_W {
        PEREO4_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&mut self) -> PEREO5_W {
        PEREO5_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&mut self) -> PEREO6_W {
        PEREO6_W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&mut self) -> PEREO7_W {
        PEREO7_W { w: self }
    }
    #[doc = "Bit 8 - Compare 0 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo0(&mut self) -> CMPEO0_W {
        CMPEO0_W { w: self }
    }
    #[doc = "Bit 9 - Compare 1 Event Output Enable"]
    #[inline(always)]
    pub fn cmpeo1(&mut self) -> CMPEO1_W {
        CMPEO1_W { w: self }
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
}

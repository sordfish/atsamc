#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u16, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u16, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPEO0`"]
pub type COMPEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO0`"]
pub struct COMPEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COMPEO1`"]
pub type COMPEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO1`"]
pub struct COMPEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `COMPEO2`"]
pub type COMPEO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO2`"]
pub struct COMPEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMPEO3`"]
pub type COMPEO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEO3`"]
pub struct COMPEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WINEO0`"]
pub type WINEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WINEO0`"]
pub struct WINEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> WINEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WINEO1`"]
pub type WINEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WINEO1`"]
pub struct WINEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> WINEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `COMPEI0`"]
pub type COMPEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI0`"]
pub struct COMPEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `COMPEI1`"]
pub type COMPEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI1`"]
pub struct COMPEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `COMPEI2`"]
pub type COMPEI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI2`"]
pub struct COMPEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `COMPEI3`"]
pub type COMPEI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPEI3`"]
pub struct COMPEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `INVEI0`"]
pub type INVEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI0`"]
pub struct INVEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `INVEI1`"]
pub type INVEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI1`"]
pub struct INVEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `INVEI2`"]
pub type INVEI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI2`"]
pub struct INVEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `INVEI3`"]
pub type INVEI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEI3`"]
pub struct INVEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> COMPEO0_R {
        COMPEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> COMPEO1_R {
        COMPEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    pub fn compeo2(&self) -> COMPEO2_R {
        COMPEO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    pub fn compeo3(&self) -> COMPEO3_R {
        COMPEO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> WINEO0_R {
        WINEO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    pub fn wineo1(&self) -> WINEO1_R {
        WINEO1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> COMPEI0_R {
        COMPEI0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> COMPEI1_R {
        COMPEI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    pub fn compei2(&self) -> COMPEI2_R {
        COMPEI2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    pub fn compei3(&self) -> COMPEI3_R {
        COMPEI3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei2(&self) -> INVEI2_R {
        INVEI2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei3(&self) -> INVEI3_R {
        INVEI3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&mut self) -> COMPEO0_W {
        COMPEO0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&mut self) -> COMPEO1_W {
        COMPEO1_W { w: self }
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    pub fn compeo2(&mut self) -> COMPEO2_W {
        COMPEO2_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    pub fn compeo3(&mut self) -> COMPEO3_W {
        COMPEO3_W { w: self }
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&mut self) -> WINEO0_W {
        WINEO0_W { w: self }
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    pub fn wineo1(&mut self) -> WINEO1_W {
        WINEO1_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&mut self) -> COMPEI0_W {
        COMPEI0_W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&mut self) -> COMPEI1_W {
        COMPEI1_W { w: self }
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    pub fn compei2(&mut self) -> COMPEI2_W {
        COMPEI2_W { w: self }
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    pub fn compei3(&mut self) -> COMPEI3_W {
        COMPEI3_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&mut self) -> INVEI0_W {
        INVEI0_W { w: self }
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&mut self) -> INVEI1_W {
        INVEI1_W { w: self }
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei2(&mut self) -> INVEI2_W {
        INVEI2_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei3(&mut self) -> INVEI3_W {
        INVEI3_W { w: self }
    }
}

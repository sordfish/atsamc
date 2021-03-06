#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u16, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u16, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER0`"]
pub type PER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER0`"]
pub struct PER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PER0_W<'a> {
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
#[doc = "Reader of field `PER1`"]
pub type PER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER1`"]
pub struct PER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PER1_W<'a> {
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
#[doc = "Reader of field `PER2`"]
pub type PER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER2`"]
pub struct PER2_W<'a> {
    w: &'a mut W,
}
impl<'a> PER2_W<'a> {
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
#[doc = "Reader of field `PER3`"]
pub type PER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER3`"]
pub struct PER3_W<'a> {
    w: &'a mut W,
}
impl<'a> PER3_W<'a> {
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
#[doc = "Reader of field `PER4`"]
pub type PER4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER4`"]
pub struct PER4_W<'a> {
    w: &'a mut W,
}
impl<'a> PER4_W<'a> {
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
#[doc = "Reader of field `PER5`"]
pub type PER5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER5`"]
pub struct PER5_W<'a> {
    w: &'a mut W,
}
impl<'a> PER5_W<'a> {
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
#[doc = "Reader of field `PER6`"]
pub type PER6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER6`"]
pub struct PER6_W<'a> {
    w: &'a mut W,
}
impl<'a> PER6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PER7`"]
pub type PER7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER7`"]
pub struct PER7_W<'a> {
    w: &'a mut W,
}
impl<'a> PER7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CMP0`"]
pub type CMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0`"]
pub struct CMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_W<'a> {
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
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    pub fn per2(&self) -> PER2_R {
        PER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    pub fn per3(&self) -> PER3_R {
        PER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    pub fn per4(&self) -> PER4_R {
        PER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    pub fn per5(&self) -> PER5_R {
        PER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    pub fn per6(&self) -> PER6_R {
        PER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    pub fn per7(&self) -> PER7_R {
        PER7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    pub fn per0(&mut self) -> PER0_W {
        PER0_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    pub fn per1(&mut self) -> PER1_W {
        PER1_W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    pub fn per2(&mut self) -> PER2_W {
        PER2_W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    pub fn per3(&mut self) -> PER3_W {
        PER3_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    pub fn per4(&mut self) -> PER4_W {
        PER4_W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    pub fn per5(&mut self) -> PER5_W {
        PER5_W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    pub fn per6(&mut self) -> PER6_W {
        PER6_W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    pub fn per7(&mut self) -> PER7_W {
        PER7_W { w: self }
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> CMP0_W {
        CMP0_W { w: self }
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
}

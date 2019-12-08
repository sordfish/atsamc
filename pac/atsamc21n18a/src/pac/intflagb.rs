#[doc = "Reader of register INTFLAGB"]
pub type R = crate::R<u32, super::INTFLAGB>;
#[doc = "Writer for register INTFLAGB"]
pub type W = crate::W<u32, super::INTFLAGB>;
#[doc = "Register INTFLAGB `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT_`"]
pub type PORT__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_`"]
pub struct PORT__W<'a> {
    w: &'a mut W,
}
impl<'a> PORT__W<'a> {
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
#[doc = "Reader of field `DSU_`"]
pub type DSU__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSU_`"]
pub struct DSU__W<'a> {
    w: &'a mut W,
}
impl<'a> DSU__W<'a> {
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
#[doc = "Reader of field `NVMCTRL_`"]
pub type NVMCTRL__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVMCTRL_`"]
pub struct NVMCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL__W<'a> {
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
#[doc = "Reader of field `DMAC_`"]
pub type DMAC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAC_`"]
pub struct DMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC__W<'a> {
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
#[doc = "Reader of field `MTB_`"]
pub type MTB__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTB_`"]
pub struct MTB__W<'a> {
    w: &'a mut W,
}
impl<'a> MTB__W<'a> {
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
#[doc = "Reader of field `HMATRIXHS_`"]
pub type HMATRIXHS__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HMATRIXHS_`"]
pub struct HMATRIXHS__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIXHS__W<'a> {
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
impl R {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&self) -> MTB__R {
        MTB__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> HMATRIXHS__R {
        HMATRIXHS__R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W { w: self }
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W { w: self }
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&mut self) -> MTB__W {
        MTB__W { w: self }
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&mut self) -> HMATRIXHS__W {
        HMATRIXHS__W { w: self }
    }
}

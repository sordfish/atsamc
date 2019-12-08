#[doc = "Reader of register INTFLAGAHB"]
pub type R = crate::R<u32, super::INTFLAGAHB>;
#[doc = "Writer for register INTFLAGAHB"]
pub type W = crate::W<u32, super::INTFLAGAHB>;
#[doc = "Register INTFLAGAHB `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAGAHB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_`"]
pub type FLASH__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_`"]
pub struct FLASH__W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH__W<'a> {
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
#[doc = "Reader of field `HSRAMCM0P_`"]
pub type HSRAMCM0P__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSRAMCM0P_`"]
pub struct HSRAMCM0P__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAMCM0P__W<'a> {
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
#[doc = "Reader of field `HSRAMDSU_`"]
pub type HSRAMDSU__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSRAMDSU_`"]
pub struct HSRAMDSU__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAMDSU__W<'a> {
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
#[doc = "Reader of field `HPB1_`"]
pub type HPB1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB1_`"]
pub struct HPB1__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB1__W<'a> {
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
#[doc = "Reader of field `HPB0_`"]
pub type HPB0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB0_`"]
pub struct HPB0__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB0__W<'a> {
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
#[doc = "Reader of field `HPB2_`"]
pub type HPB2__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB2_`"]
pub struct HPB2__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB2__W<'a> {
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
#[doc = "Reader of field `LPRAMDMAC_`"]
pub type LPRAMDMAC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPRAMDMAC_`"]
pub struct LPRAMDMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> LPRAMDMAC__W<'a> {
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
#[doc = "Reader of field `DIVAS_`"]
pub type DIVAS__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVAS_`"]
pub struct DIVAS__W<'a> {
    w: &'a mut W,
}
impl<'a> DIVAS__W<'a> {
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
#[doc = "Reader of field `HPB3_`"]
pub type HPB3__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB3_`"]
pub struct HPB3__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB3__W<'a> {
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
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> HSRAMCM0P__R {
        HSRAMCM0P__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> HSRAMDSU__R {
        HSRAMDSU__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&self) -> LPRAMDMAC__R {
        LPRAMDMAC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    pub fn divas_(&self) -> DIVAS__R {
        DIVAS__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&mut self) -> FLASH__W {
        FLASH__W { w: self }
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&mut self) -> HSRAMCM0P__W {
        HSRAMCM0P__W { w: self }
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&mut self) -> HSRAMDSU__W {
        HSRAMDSU__W { w: self }
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&mut self) -> HPB1__W {
        HPB1__W { w: self }
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&mut self) -> HPB0__W {
        HPB0__W { w: self }
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&mut self) -> HPB2__W {
        HPB2__W { w: self }
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&mut self) -> LPRAMDMAC__W {
        LPRAMDMAC__W { w: self }
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    pub fn divas_(&mut self) -> DIVAS__W {
        DIVAS__W { w: self }
    }
    #[doc = "Bit 8 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&mut self) -> HPB3__W {
        HPB3__W { w: self }
    }
}

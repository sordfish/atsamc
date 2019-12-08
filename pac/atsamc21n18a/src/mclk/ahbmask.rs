#[doc = "Reader of register AHBMASK"]
pub type R = crate::R<u32, super::AHBMASK>;
#[doc = "Writer for register AHBMASK"]
pub type W = crate::W<u32, super::AHBMASK>;
#[doc = "Register AHBMASK `reset()`'s with value 0x3cff"]
impl crate::ResetValue for super::AHBMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3cff
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HSRAM_`"]
pub type HSRAM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSRAM_`"]
pub struct HSRAM__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CAN0_`"]
pub type CAN0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0_`"]
pub struct CAN0__W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0__W<'a> {
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
#[doc = "Reader of field `CAN1_`"]
pub type CAN1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1_`"]
pub struct CAN1__W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1__W<'a> {
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
#[doc = "Reader of field `PAC_`"]
pub type PAC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAC_`"]
pub struct PAC__W<'a> {
    w: &'a mut W,
}
impl<'a> PAC__W<'a> {
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
#[doc = "Reader of field `NVMCTRL_PICACHU_`"]
pub type NVMCTRL_PICACHU__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVMCTRL_PICACHU_`"]
pub struct NVMCTRL_PICACHU__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL_PICACHU__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> HMATRIXHS__R {
        HMATRIXHS__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> HSRAM__R {
        HSRAM__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CAN0 AHB Clock Mask"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CAN1 AHB Clock Mask"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_picachu_(&self) -> NVMCTRL_PICACHU__R {
        NVMCTRL_PICACHU__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DIVAS AHB Clock Mask"]
    #[inline(always)]
    pub fn divas_(&self) -> DIVAS__R {
        DIVAS__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&mut self) -> HPB0__W {
        HPB0__W { w: self }
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&mut self) -> HPB1__W {
        HPB1__W { w: self }
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&mut self) -> HPB2__W {
        HPB2__W { w: self }
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrixhs_(&mut self) -> HMATRIXHS__W {
        HMATRIXHS__W { w: self }
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&mut self) -> HSRAM__W {
        HSRAM__W { w: self }
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W { w: self }
    }
    #[doc = "Bit 8 - CAN0 AHB Clock Mask"]
    #[inline(always)]
    pub fn can0_(&mut self) -> CAN0__W {
        CAN0__W { w: self }
    }
    #[doc = "Bit 9 - CAN1 AHB Clock Mask"]
    #[inline(always)]
    pub fn can1_(&mut self) -> CAN1__W {
        CAN1__W { w: self }
    }
    #[doc = "Bit 10 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W { w: self }
    }
    #[doc = "Bit 11 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_picachu_(&mut self) -> NVMCTRL_PICACHU__W {
        NVMCTRL_PICACHU__W { w: self }
    }
    #[doc = "Bit 12 - DIVAS AHB Clock Mask"]
    #[inline(always)]
    pub fn divas_(&mut self) -> DIVAS__W {
        DIVAS__W { w: self }
    }
    #[doc = "Bit 13 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&mut self) -> HPB3__W {
        HPB3__W { w: self }
    }
}

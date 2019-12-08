#[doc = "Reader of register STATUSB"]
pub type R = crate::R<u32, super::STATUSB>;
#[doc = "Reader of field `PORT_`"]
pub type PORT__R = crate::R<bool, bool>;
#[doc = "Reader of field `DSU_`"]
pub type DSU__R = crate::R<bool, bool>;
#[doc = "Reader of field `NVMCTRL_`"]
pub type NVMCTRL__R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAC_`"]
pub type DMAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `MTB_`"]
pub type MTB__R = crate::R<bool, bool>;
#[doc = "Reader of field `HMATRIXHS_`"]
pub type HMATRIXHS__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTB APB Protect Enable"]
    #[inline(always)]
    pub fn mtb_(&self) -> MTB__R {
        MTB__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS APB Protect Enable"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> HMATRIXHS__R {
        HMATRIXHS__R::new(((self.bits >> 5) & 0x01) != 0)
    }
}

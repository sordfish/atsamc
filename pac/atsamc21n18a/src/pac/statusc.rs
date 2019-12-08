#[doc = "Reader of register STATUSC"]
pub type R = crate::R<u32, super::STATUSC>;
#[doc = "Reader of field `EVSYS_`"]
pub type EVSYS__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM0_`"]
pub type SERCOM0__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM1_`"]
pub type SERCOM1__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM2_`"]
pub type SERCOM2__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM3_`"]
pub type SERCOM3__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM4_`"]
pub type SERCOM4__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM5_`"]
pub type SERCOM5__R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN0_`"]
pub type CAN0__R = crate::R<bool, bool>;
#[doc = "Reader of field `CAN1_`"]
pub type CAN1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC0_`"]
pub type TCC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC1_`"]
pub type TCC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC2_`"]
pub type TCC2__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC0_`"]
pub type TC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC1_`"]
pub type TC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC2_`"]
pub type TC2__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC3_`"]
pub type TC3__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC4_`"]
pub type TC4__R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0_`"]
pub type ADC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1_`"]
pub type ADC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `SDADC_`"]
pub type SDADC__R = crate::R<bool, bool>;
#[doc = "Reader of field `AC_`"]
pub type AC__R = crate::R<bool, bool>;
#[doc = "Reader of field `DAC_`"]
pub type DAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `PTC_`"]
pub type PTC__R = crate::R<bool, bool>;
#[doc = "Reader of field `CCL_`"]
pub type CCL__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN0 APB Protect Enable"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CAN1 APB Protect Enable"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SDADC APB Protect Enable"]
    #[inline(always)]
    pub fn sdadc_(&self) -> SDADC__R {
        SDADC__R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 23) & 0x01) != 0)
    }
}

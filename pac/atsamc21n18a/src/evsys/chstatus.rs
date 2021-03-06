#[doc = "Reader of register CHSTATUS"]
pub type R = crate::R<u32, super::CHSTATUS>;
#[doc = "Reader of field `USRRDY0`"]
pub type USRRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY1`"]
pub type USRRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY2`"]
pub type USRRDY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY3`"]
pub type USRRDY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY4`"]
pub type USRRDY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY5`"]
pub type USRRDY5_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY6`"]
pub type USRRDY6_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY7`"]
pub type USRRDY7_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY8`"]
pub type USRRDY8_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY9`"]
pub type USRRDY9_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY10`"]
pub type USRRDY10_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY11`"]
pub type USRRDY11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY0`"]
pub type CHBUSY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY1`"]
pub type CHBUSY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY2`"]
pub type CHBUSY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY3`"]
pub type CHBUSY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY4`"]
pub type CHBUSY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY5`"]
pub type CHBUSY5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY6`"]
pub type CHBUSY6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY7`"]
pub type CHBUSY7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY8`"]
pub type CHBUSY8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY9`"]
pub type CHBUSY9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY10`"]
pub type CHBUSY10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY11`"]
pub type CHBUSY11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 User Ready"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> USRRDY0_R {
        USRRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 User Ready"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> USRRDY1_R {
        USRRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 User Ready"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> USRRDY2_R {
        USRRDY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 User Ready"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> USRRDY3_R {
        USRRDY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 User Ready"]
    #[inline(always)]
    pub fn usrrdy4(&self) -> USRRDY4_R {
        USRRDY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 User Ready"]
    #[inline(always)]
    pub fn usrrdy5(&self) -> USRRDY5_R {
        USRRDY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 User Ready"]
    #[inline(always)]
    pub fn usrrdy6(&self) -> USRRDY6_R {
        USRRDY6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 User Ready"]
    #[inline(always)]
    pub fn usrrdy7(&self) -> USRRDY7_R {
        USRRDY7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 User Ready"]
    #[inline(always)]
    pub fn usrrdy8(&self) -> USRRDY8_R {
        USRRDY8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 User Ready"]
    #[inline(always)]
    pub fn usrrdy9(&self) -> USRRDY9_R {
        USRRDY9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 User Ready"]
    #[inline(always)]
    pub fn usrrdy10(&self) -> USRRDY10_R {
        USRRDY10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 User Ready"]
    #[inline(always)]
    pub fn usrrdy11(&self) -> USRRDY11_R {
        USRRDY11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> CHBUSY0_R {
        CHBUSY0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> CHBUSY1_R {
        CHBUSY1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> CHBUSY2_R {
        CHBUSY2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> CHBUSY3_R {
        CHBUSY3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 Busy"]
    #[inline(always)]
    pub fn chbusy4(&self) -> CHBUSY4_R {
        CHBUSY4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 Busy"]
    #[inline(always)]
    pub fn chbusy5(&self) -> CHBUSY5_R {
        CHBUSY5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 Busy"]
    #[inline(always)]
    pub fn chbusy6(&self) -> CHBUSY6_R {
        CHBUSY6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 Busy"]
    #[inline(always)]
    pub fn chbusy7(&self) -> CHBUSY7_R {
        CHBUSY7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Busy"]
    #[inline(always)]
    pub fn chbusy8(&self) -> CHBUSY8_R {
        CHBUSY8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Busy"]
    #[inline(always)]
    pub fn chbusy9(&self) -> CHBUSY9_R {
        CHBUSY9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Busy"]
    #[inline(always)]
    pub fn chbusy10(&self) -> CHBUSY10_R {
        CHBUSY10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Busy"]
    #[inline(always)]
    pub fn chbusy11(&self) -> CHBUSY11_R {
        CHBUSY11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}

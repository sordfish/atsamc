#[doc = "Reader of register SLEEPCFG"]
pub type R = crate::R<u8, super::SLEEPCFG>;
#[doc = "Writer for register SLEEPCFG"]
pub type W = crate::W<u8, super::SLEEPCFG>;
#[doc = "Register SLEEPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLEEPMODE_A {
    #[doc = "0: CPU clock is OFF"]
    IDLE0 = 0,
    #[doc = "1: AHB clock is OFF"]
    IDLE1 = 1,
    #[doc = "2: APB clock are OFF"]
    IDLE2 = 2,
    #[doc = "4: All Clocks are OFF"]
    STANDBY = 4,
    #[doc = "5: Only Backup domain is powered ON"]
    BACKUP = 5,
    #[doc = "6: All power domains are powered OFF"]
    OFF = 6,
}
impl From<SLEEPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLEEPMODE`"]
pub type SLEEPMODE_R = crate::R<u8, SLEEPMODE_A>;
impl SLEEPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLEEPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLEEPMODE_A::IDLE0),
            1 => Val(SLEEPMODE_A::IDLE1),
            2 => Val(SLEEPMODE_A::IDLE2),
            4 => Val(SLEEPMODE_A::STANDBY),
            5 => Val(SLEEPMODE_A::BACKUP),
            6 => Val(SLEEPMODE_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE0`"]
    #[inline(always)]
    pub fn is_idle0(&self) -> bool {
        *self == SLEEPMODE_A::IDLE0
    }
    #[doc = "Checks if the value of the field is `IDLE1`"]
    #[inline(always)]
    pub fn is_idle1(&self) -> bool {
        *self == SLEEPMODE_A::IDLE1
    }
    #[doc = "Checks if the value of the field is `IDLE2`"]
    #[inline(always)]
    pub fn is_idle2(&self) -> bool {
        *self == SLEEPMODE_A::IDLE2
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == SLEEPMODE_A::BACKUP
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODE_A::OFF
    }
}
#[doc = "Write proxy for field `SLEEPMODE`"]
pub struct SLEEPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CPU clock is OFF"]
    #[inline(always)]
    pub fn idle0(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE0)
    }
    #[doc = "AHB clock is OFF"]
    #[inline(always)]
    pub fn idle1(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE1)
    }
    #[doc = "APB clock are OFF"]
    #[inline(always)]
    pub fn idle2(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE2)
    }
    #[doc = "All Clocks are OFF"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::STANDBY)
    }
    #[doc = "Only Backup domain is powered ON"]
    #[inline(always)]
    pub fn backup(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::BACKUP)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&self) -> SLEEPMODE_R {
        SLEEPMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&mut self) -> SLEEPMODE_W {
        SLEEPMODE_W { w: self }
    }
}

#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Unaligned accesses generates a Hard Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRP_A {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    VALUE_0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    VALUE_1 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, UNALIGN_TRP_A>;
impl UNALIGN_TRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::VALUE_0,
            true => UNALIGN_TRP_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UNALIGN_TRP_A::VALUE_1
    }
}
#[doc = "Stack 8-byte aligned on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGN_A {
    #[doc = "0: 4-byte aligned"]
    VALUE_0 = 0,
    #[doc = "1: 8-byte aligned"]
    VALUE_1 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, STKALIGN_A>;
impl STKALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::VALUE_0,
            true => STKALIGN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == STKALIGN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == STKALIGN_A::VALUE_1
    }
}
impl R {
    #[doc = "Bit 3 - Unaligned accesses generates a Hard Fault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stack 8-byte aligned on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}

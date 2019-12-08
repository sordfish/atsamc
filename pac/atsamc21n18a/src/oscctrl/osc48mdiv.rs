#[doc = "Reader of register OSC48MDIV"]
pub type R = crate::R<u8, super::OSC48MDIV>;
#[doc = "Writer for register OSC48MDIV"]
pub type W = crate::W<u8, super::OSC48MDIV>;
#[doc = "Register OSC48MDIV `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::OSC48MDIV {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "OSC48M Division Factor\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: 48 MHz"]
    DIV1 = 0,
    #[doc = "1: 24 MHz"]
    DIV2 = 1,
    #[doc = "2: 16 MHz"]
    DIV3 = 2,
    #[doc = "3: 12 MHz"]
    DIV4 = 3,
    #[doc = "4: 9.6 MHz"]
    DIV5 = 4,
    #[doc = "5: 8 MHz"]
    DIV6 = 5,
    #[doc = "6: 6.86 MHz"]
    DIV7 = 6,
    #[doc = "7: 6 MHz"]
    DIV8 = 7,
    #[doc = "8: 5.33 MHz"]
    DIV9 = 8,
    #[doc = "9: 4.8 MHz"]
    DIV10 = 9,
    #[doc = "10: 4.36 MHz"]
    DIV11 = 10,
    #[doc = "11: 4 MHz"]
    DIV12 = 11,
    #[doc = "12: 3.69 MHz"]
    DIV13 = 12,
    #[doc = "13: 3.43 MHz"]
    DIV14 = 13,
    #[doc = "14: 3.2 MHz"]
    DIV15 = 14,
    #[doc = "15: 3 MHz"]
    DIV16 = 15,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, DIV_A>;
impl DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_A {
        match self.bits {
            0 => DIV_A::DIV1,
            1 => DIV_A::DIV2,
            2 => DIV_A::DIV3,
            3 => DIV_A::DIV4,
            4 => DIV_A::DIV5,
            5 => DIV_A::DIV6,
            6 => DIV_A::DIV7,
            7 => DIV_A::DIV8,
            8 => DIV_A::DIV9,
            9 => DIV_A::DIV10,
            10 => DIV_A::DIV11,
            11 => DIV_A::DIV12,
            12 => DIV_A::DIV13,
            13 => DIV_A::DIV14,
            14 => DIV_A::DIV15,
            15 => DIV_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == DIV_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == DIV_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == DIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == DIV_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == DIV_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == DIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == DIV_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == DIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == DIV_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == DIV_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == DIV_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIV_A::DIV16
    }
}
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIV_A::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIV_A::DIV2)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(DIV_A::DIV3)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIV_A::DIV4)
    }
    #[doc = "9.6 MHz"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(DIV_A::DIV5)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(DIV_A::DIV6)
    }
    #[doc = "6.86 MHz"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(DIV_A::DIV7)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIV_A::DIV8)
    }
    #[doc = "5.33 MHz"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(DIV_A::DIV9)
    }
    #[doc = "4.8 MHz"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(DIV_A::DIV10)
    }
    #[doc = "4.36 MHz"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(DIV_A::DIV11)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(DIV_A::DIV12)
    }
    #[doc = "3.69 MHz"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(DIV_A::DIV13)
    }
    #[doc = "3.43 MHz"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(DIV_A::DIV14)
    }
    #[doc = "3.2 MHz"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(DIV_A::DIV15)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIV_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - OSC48M Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - OSC48M Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}

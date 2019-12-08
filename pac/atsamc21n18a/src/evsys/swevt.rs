#[doc = "Writer for register SWEVT"]
pub type W = crate::W<u32, super::SWEVT>;
#[doc = "Register SWEVT `reset()`'s with value 0"]
impl crate::ResetValue for super::SWEVT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CHANNEL0`"]
pub struct CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL0_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL1`"]
pub struct CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL1_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL2`"]
pub struct CHANNEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL2_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL3`"]
pub struct CHANNEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL3_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL4`"]
pub struct CHANNEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL4_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL5`"]
pub struct CHANNEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL5_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL6`"]
pub struct CHANNEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL6_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL7`"]
pub struct CHANNEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL7_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL8`"]
pub struct CHANNEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL8_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL9`"]
pub struct CHANNEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL9_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL10`"]
pub struct CHANNEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL10_W<'a> {
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
#[doc = "Write proxy for field `CHANNEL11`"]
pub struct CHANNEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL11_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W {
        CHANNEL0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W {
        CHANNEL1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W {
        CHANNEL2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W {
        CHANNEL3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W {
        CHANNEL4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W {
        CHANNEL5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Selection"]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W {
        CHANNEL6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Selection"]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W {
        CHANNEL7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Selection"]
    #[inline(always)]
    pub fn channel8(&mut self) -> CHANNEL8_W {
        CHANNEL8_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Selection"]
    #[inline(always)]
    pub fn channel9(&mut self) -> CHANNEL9_W {
        CHANNEL9_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Selection"]
    #[inline(always)]
    pub fn channel10(&mut self) -> CHANNEL10_W {
        CHANNEL10_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Selection"]
    #[inline(always)]
    pub fn channel11(&mut self) -> CHANNEL11_W {
        CHANNEL11_W { w: self }
    }
}

#[doc = "Reader of register OSC48MSYNCBUSY"]
pub type R = crate::R<u32, super::OSC48MSYNCBUSY>;
#[doc = "Reader of field `OSC48MDIV`"]
pub type OSC48MDIV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - OSC48MDIV Synchronization Status"]
    #[inline(always)]
    pub fn osc48mdiv(&self) -> OSC48MDIV_R {
        OSC48MDIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}

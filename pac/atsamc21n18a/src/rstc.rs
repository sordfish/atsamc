#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](rcause) module"]
pub type RCAUSE = crate::Reg<u8, _RCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCAUSE;
#[doc = "`read()` method returns [rcause::R](rcause::R) reader structure"]
impl crate::Readable for RCAUSE {}
#[doc = "Reset Cause"]
pub mod rcause;

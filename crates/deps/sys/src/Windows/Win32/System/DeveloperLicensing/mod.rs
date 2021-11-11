#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DeveloperLicensing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AcquireDeveloperLicense();
    #[doc = "*Required features: `Win32_System_DeveloperLicensing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckDeveloperLicense();
    #[doc = "*Required features: `Win32_System_DeveloperLicensing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDeveloperLicense();
}

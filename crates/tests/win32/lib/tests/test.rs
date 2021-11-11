use test_win32_lib::*;

#[test]
fn linker() -> windows::core::Result<()> {
    unsafe {
        Windows::Win32::Graphics::Direct3D::Fxc::D3DCreateLinker()?;
        Ok(())
    }
}

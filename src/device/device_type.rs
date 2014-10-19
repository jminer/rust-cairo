#[repr(i32)]
pub enum DeviceType {
  Invalid = -1,
  DRM = 0,
  GL = 1,
  Script = 2,
  XCB = 3,
  XLib = 4,
  XML = 5,
  COGL = 6,
  Win32 = 7
}

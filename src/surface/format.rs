#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum Format {
  Invalid = -1,
  ARGB32 = 0,
  RGB24 = 1,
  A8 = 2,
  A1 = 3,
  RGB16_565 = 4,
  RGB30 = 5
}

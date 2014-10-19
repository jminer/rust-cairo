#[repr(i32)]
pub enum Filter {
  Fast = 0,
  Good = 1,
  Best = 2,
  Nearest = 3,
  Bilinear = 4,
  Gaussian = 5
}

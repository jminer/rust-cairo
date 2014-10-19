#[repr(i32)]
pub enum PatternType {
  Solid = 0,
  Surface = 1,
  Linear = 2,
  Radial = 3,
  Mesh = 4,
  RasterSource = 5
}

use std::mem;

#[repr(C)]
pub struct Matrix {
  xx: f64,
  yx: f64,
  xy: f64,
  yy: f64,
  x0: f64,
  y0: f64
}

impl Matrix {
  pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_init(&mut this, xx, yx, xy, yy, x0, y0);
      return this;
    }
  }

  pub fn identity() -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_init_identity(&mut this);
      return this;
    }
  }

  pub fn for_translation(x0: f64, y0: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_init_translate(&mut this, x0, y0);
      return this;
    }
  }

  pub fn for_scale(sx: f64, sy: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_init_scale(&mut this, sx, sy);
      return this;
    }
  }

  pub fn for_rotation(radians: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_init_rotate(&mut this, radians);
      return this;
    }
  }

  pub fn multiply(a: &mut Matrix, b: &mut Matrix) -> Matrix {
    unsafe {
      let mut this:Matrix = mem::zeroed();
      cairo_matrix_multiply(&mut this, a, b);
      return this;
    }
  }

  pub fn translate(&mut self, x0: f64, y0: f64) {
    unsafe {
      cairo_matrix_translate(self, x0, y0);
    }
  }

  pub fn scale(&mut self, sx: f64, sy: f64) {
    unsafe {
      cairo_matrix_scale(self, sx, sy);
    }
  }

  pub fn rotate(&mut self, radians: f64) {
    unsafe {
      cairo_matrix_rotate(self, radians);
    }
  }

  pub fn transform_distance(&mut self) -> (f64, f64) {
    unsafe {
      let mut dx:f64 = mem::zeroed();
      let mut dy:f64 = mem::zeroed();
      cairo_matrix_transform_distance(self, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  pub fn transform_point(&mut self) -> (f64, f64) {
    unsafe {
      let mut x:f64 = mem::zeroed();
      let mut y:f64 = mem::zeroed();
      cairo_matrix_transform_point(self, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn invert(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_matrix_invert(self);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_matrix_init(this: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
  fn cairo_matrix_init_identity(this: *mut Matrix);
  fn cairo_matrix_init_translate(this: *mut Matrix, x0: f64, y0: f64);
  fn cairo_matrix_init_scale(this: *mut Matrix, sx: f64, sy: f64);
  fn cairo_matrix_init_rotate(this: *mut Matrix, radians: f64);
  fn cairo_matrix_multiply(this: *mut Matrix, a: *mut Matrix, b: *mut Matrix);
  fn cairo_matrix_translate(self_arg: *mut Matrix, x0: f64, y0: f64);
  fn cairo_matrix_scale(self_arg: *mut Matrix, sx: f64, sy: f64);
  fn cairo_matrix_rotate(self_arg: *mut Matrix, radians: f64);
  fn cairo_matrix_transform_distance(self_arg: *mut Matrix, dx: *mut f64, dy: *mut f64);
  fn cairo_matrix_transform_point(self_arg: *mut Matrix, x: *mut f64, y: *mut f64);
  fn cairo_matrix_invert(self_arg: *mut Matrix) -> super::Status;
}

use libc;
use std;

pub struct Pattern {
  pub opaque: *mut libc::c_void
}

impl Pattern {
  pub fn add_color_stop_rgb(&mut self, offset: f64, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_pattern_add_color_stop_rgb(self.opaque, offset, red, green, blue);
    }
  }

  pub fn add_color_stop_rgba(&mut self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_pattern_add_color_stop_rgba(self.opaque, offset, red, green, blue, alpha);
    }
  }

  pub fn get_color_stop_count(&mut self) -> (super::Status, libc::c_int) {
    unsafe {
      let mut stop_count:libc::c_int = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_color_stop_count(self.opaque, &mut stop_count);
      return (foreign_result, stop_count);
    }
  }

  pub fn get_color_stop_rgba(&mut self, stop_count: libc::c_int) -> (super::Status, f64, f64, f64, f64, f64) {
    unsafe {
      let mut offset:f64 = std::intrinsics::init();
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_color_stop_rgba(self.opaque, stop_count, &mut offset, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, offset, red, green, blue, alpha);
    }
  }

  pub fn create_rgb(red: f64, green: f64, blue: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_rgb(red, green, blue);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn create_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_rgba(red, green, blue, alpha);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_rgba(&mut self) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_rgba(self.opaque, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, red, green, blue, alpha);
    }
  }

  pub fn create_for_surface(surface: &mut super::surface::Surface) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_for_surface(surface.opaque);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_surface(&mut self) -> (super::Status, super::surface::Surface) {
    unsafe {
      let mut surface:super::surface::Surface = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_surface(self.opaque, &mut surface);
      return (foreign_result, surface);
    }
  }

  pub fn create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_linear(x0, y0, x1, y1);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_linear_points(&mut self) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut x0:f64 = std::intrinsics::init();
      let mut y0:f64 = std::intrinsics::init();
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_linear_points(self.opaque, &mut x0, &mut y0, &mut x1, &mut y1);
      return (foreign_result, x0, y0, x1, y1);
    }
  }

  pub fn create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_radial(cx0, cy0, radius0, cx1, cy1, radius1);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_radial_circles(&mut self) -> (super::Status, f64, f64, f64, f64, f64, f64) {
    unsafe {
      let mut x0:f64 = std::intrinsics::init();
      let mut y0:f64 = std::intrinsics::init();
      let mut r0:f64 = std::intrinsics::init();
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut r1:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_radial_circles(self.opaque, &mut x0, &mut y0, &mut r0, &mut x1, &mut y1, &mut r1);
      return (foreign_result, x0, y0, r0, x1, y1, r1);
    }
  }

  pub fn create_mesh() -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_mesh();
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn begin_patch(&mut self) {
    unsafe {
      cairo_mesh_pattern_begin_patch(self.opaque);
    }
  }

  pub fn end_patch(&mut self) {
    unsafe {
      cairo_mesh_pattern_end_patch(self.opaque);
    }
  }

  pub fn move_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_move_to(self.opaque, x, y);
    }
  }

  pub fn line_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_line_to(self.opaque, x, y);
    }
  }

  pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe {
      cairo_mesh_pattern_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
    }
  }

  pub fn set_control_point(&mut self, point_num: libc::c_uint, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_set_control_point(self.opaque, point_num, x, y);
    }
  }

  pub fn set_corner_color_rgb(&mut self, corner_num: libc::c_uint, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_mesh_pattern_set_corner_color_rgb(self.opaque, corner_num, red, green, blue);
    }
  }

  pub fn set_corner_color_rgba(&mut self, corner_num: libc::c_uint, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_mesh_pattern_set_corner_color_rgba(self.opaque, corner_num, red, green, blue, alpha);
    }
  }

  pub fn get_patch_count(&mut self) -> (super::Status, libc::c_uint) {
    unsafe {
      let mut count:libc::c_uint = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_patch_count(self.opaque, &mut count);
      return (foreign_result, count);
    }
  }

  pub fn get_path(&mut self, patch_num: libc::c_uint) -> super::path::Path {
    unsafe {
      let foreign_result = cairo_mesh_pattern_get_path(self.opaque, patch_num);
      return super::path::Path { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_control_point(&mut self, patch_num: libc::c_uint, pointer_num: libc::c_uint) -> (super::Status, f64, f64) {
    unsafe {
      let mut x:f64 = std::intrinsics::init();
      let mut y:f64 = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_control_point(self.opaque, patch_num, pointer_num, &mut x, &mut y);
      return (foreign_result, x, y);
    }
  }

  pub fn get_corner_color_rgba(&mut self, patch_num: libc::c_uint, pointer_num: libc::c_uint) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_corner_color_rgba(self.opaque, patch_num, pointer_num, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, red, green, blue, alpha);
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_pattern_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_extend(&mut self, extend: extend::Extend) {
    unsafe {
      cairo_pattern_set_extend(self.opaque, extend);
    }
  }

  pub fn get_extend(&mut self) -> extend::Extend {
    unsafe {
      let foreign_result = cairo_pattern_get_extend(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_filter(&mut self, filter: filter::Filter) {
    unsafe {
      cairo_pattern_set_filter(self.opaque, filter);
    }
  }

  pub fn get_filter(&mut self) -> filter::Filter {
    unsafe {
      let foreign_result = cairo_pattern_get_filter(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_matrix(&mut self, matrix: &mut super::matrix::Matrix) {
    unsafe {
      cairo_pattern_set_matrix(self.opaque, matrix);
    }
  }

  pub fn get_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_pattern_get_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  pub fn get_type(&mut self) -> pattern_type::PatternType {
    unsafe {
      let foreign_result = cairo_pattern_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_pattern_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_pattern_add_color_stop_rgb(self_arg: *mut libc::c_void, offset: f64, red: f64, green: f64, blue: f64);
  fn cairo_pattern_add_color_stop_rgba(self_arg: *mut libc::c_void, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_pattern_get_color_stop_count(self_arg: *mut libc::c_void, stop_count: *mut libc::c_int) -> super::Status;
  fn cairo_pattern_get_color_stop_rgba(self_arg: *mut libc::c_void, stop_count: libc::c_int, offset: *mut f64, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_create_rgb(red: f64, green: f64, blue: f64) -> *mut libc::c_void;
  fn cairo_pattern_create_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> *mut libc::c_void;
  fn cairo_pattern_get_rgba(self_arg: *mut libc::c_void, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_create_for_surface(surface: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_pattern_get_surface(self_arg: *mut libc::c_void, surface: *mut super::surface::Surface) -> super::Status;
  fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> *mut libc::c_void;
  fn cairo_pattern_get_linear_points(self_arg: *mut libc::c_void, x0: *mut f64, y0: *mut f64, x1: *mut f64, y1: *mut f64) -> super::Status;
  fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> *mut libc::c_void;
  fn cairo_pattern_get_radial_circles(self_arg: *mut libc::c_void, x0: *mut f64, y0: *mut f64, r0: *mut f64, x1: *mut f64, y1: *mut f64, r1: *mut f64) -> super::Status;
  fn cairo_pattern_create_mesh() -> *mut libc::c_void;
  fn cairo_mesh_pattern_begin_patch(self_arg: *mut libc::c_void);
  fn cairo_mesh_pattern_end_patch(self_arg: *mut libc::c_void);
  fn cairo_mesh_pattern_move_to(self_arg: *mut libc::c_void, x: f64, y: f64);
  fn cairo_mesh_pattern_line_to(self_arg: *mut libc::c_void, x: f64, y: f64);
  fn cairo_mesh_pattern_curve_to(self_arg: *mut libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
  fn cairo_mesh_pattern_set_control_point(self_arg: *mut libc::c_void, point_num: libc::c_uint, x: f64, y: f64);
  fn cairo_mesh_pattern_set_corner_color_rgb(self_arg: *mut libc::c_void, corner_num: libc::c_uint, red: f64, green: f64, blue: f64);
  fn cairo_mesh_pattern_set_corner_color_rgba(self_arg: *mut libc::c_void, corner_num: libc::c_uint, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_mesh_pattern_get_patch_count(self_arg: *mut libc::c_void, count: *mut libc::c_uint) -> super::Status;
  fn cairo_mesh_pattern_get_path(self_arg: *mut libc::c_void, patch_num: libc::c_uint) -> *mut libc::c_void;
  fn cairo_mesh_pattern_get_control_point(self_arg: *mut libc::c_void, patch_num: libc::c_uint, pointer_num: libc::c_uint, x: *mut f64, y: *mut f64) -> super::Status;
  fn cairo_mesh_pattern_get_corner_color_rgba(self_arg: *mut libc::c_void, patch_num: libc::c_uint, pointer_num: libc::c_uint, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_pattern_set_extend(self_arg: *mut libc::c_void, extend: extend::Extend);
  fn cairo_pattern_get_extend(self_arg: *mut libc::c_void) -> extend::Extend;
  fn cairo_pattern_set_filter(self_arg: *mut libc::c_void, filter: filter::Filter);
  fn cairo_pattern_get_filter(self_arg: *mut libc::c_void) -> filter::Filter;
  fn cairo_pattern_set_matrix(self_arg: *mut libc::c_void, matrix: *mut super::matrix::Matrix);
  fn cairo_pattern_get_matrix(self_arg: *mut libc::c_void, matrix: *mut super::matrix::Matrix);
  fn cairo_pattern_get_type(self_arg: *mut libc::c_void) -> pattern_type::PatternType;
  fn cairo_pattern_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
}

impl std::clone::Clone for Pattern {
  fn clone(&self) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_reference(self.opaque);
      return Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_pattern_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for Pattern {
  fn drop(&mut self) {
    unsafe {
      cairo_pattern_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_pattern_destroy(self_arg: *mut libc::c_void);
}


pub mod extend {
  #[repr(i32)]
  pub enum Extend {
    None = 0,
    Repeat = 1,
    Reflect = 2,
    Pad = 3
  }
}

pub mod filter {
  #[repr(i32)]
  pub enum Filter {
    Fast = 0,
    Good = 1,
    Best = 2,
    Nearest = 3,
    Bilinear = 4,
    Gaussian = 5
  }
}

pub mod pattern_type {
  #[repr(i32)]
  pub enum PatternType {
    Solid = 0,
    Surface = 1,
    Linear = 2,
    Radial = 3,
    Mesh = 4,
    RasterSource = 5
  }
}

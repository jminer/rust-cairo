#![allow(missing_copy_implementations)]
#![feature(core)] 
#![feature(libc)] 
#![feature(std_misc)] 
extern crate num;
extern crate libc;
#[link(name = "cairo")] extern {}

pub mod antialias;
pub mod device;
pub mod fill_rule;
pub mod font;
pub mod line_cap;
pub mod line_join;
pub mod matrix;
pub mod operator;
pub mod path;
pub mod pattern;
pub mod region;
pub mod surface;

#[repr(i32)]
pub enum Status {
  Success = 0,
  NoMemory = 1,
  InvalidRestore = 2,
  InvalidPopGroup = 3,
  NoCurrentPoint = 4,
  InvalidMatrix = 5,
  InvalidStatus = 6,
  NullPointer = 7,
  InvalidString = 8,
  InvalidPathData = 9,
  ReadError = 10,
  WriteError = 11,
  SurfaceFinished = 12,
  SurfaceTypeMismatch = 13,
  PatternTypeMismatch = 14,
  InvalidContent = 15,
  InvalidFormat = 16,
  InvalidVisual = 17,
  FileNotFound = 18,
  InvalidDash = 19,
  InvalidDSCComment = 20,
  InvalidIndex = 21,
  ClipNotRepresentable = 22,
  TempFileError = 23,
  InvalidStride = 24,
  FontTypeMismatch = 25,
  UserFontImmutable = 26,
  UserFontError = 27,
  NegativeCount = 28,
  InvalidClusters = 29,
  InvalidSlant = 30,
  InvalidWeight = 31,
  InvalidSize = 32,
  UserFontNotImplemented = 33,
  DeviceTypeMismatch = 34,
  DeviceError = 35,
  InvalidMeshConstruction = 36,
  DeviceFinished = 37
}

pub struct Cairo {
  pub opaque: *mut libc::c_void
}

impl Cairo {
  pub fn create(surface: &mut surface::Surface) -> Cairo {
    unsafe {
      let foreign_result = cairo_create(surface.opaque);
      return Cairo { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn status(&mut self) -> Status {
    unsafe {
      let foreign_result = cairo_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn save(&mut self) {
    unsafe {
      cairo_save(self.opaque);
    }
  }

  pub fn restore(&mut self) {
    unsafe {
      cairo_restore(self.opaque);
    }
  }

  pub fn get_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_target(self.opaque);
      return surface::Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn push_group(&mut self) {
    unsafe {
      cairo_push_group(self.opaque);
    }
  }

  pub fn push_group_with_content(&mut self, content: surface::content::Content) {
    unsafe {
      cairo_push_group_with_content(self.opaque, content);
    }
  }

  pub fn pop_group(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_pop_group(self.opaque);
      return pattern::Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn pop_group_to_source(&mut self) {
    unsafe {
      cairo_pop_group_to_source(self.opaque);
    }
  }

  pub fn get_group_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_group_target(self.opaque);
      return surface::Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn set_source_rgb(&mut self, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_set_source_rgb(self.opaque, red, green, blue);
    }
  }

  pub fn set_source_rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_set_source_rgba(self.opaque, red, green, blue, alpha);
    }
  }

  pub fn set_source(&mut self, source: &mut pattern::Pattern) {
    unsafe {
      cairo_set_source(self.opaque, source.opaque);
    }
  }

  pub fn set_source_surface(&mut self, surface: &mut surface::Surface, x: f64, y: f64) {
    unsafe {
      cairo_set_source_surface(self.opaque, surface.opaque, x, y);
    }
  }

  pub fn get_source(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_get_source(self.opaque);
      return pattern::Pattern { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn set_antialias(&mut self, antialias: antialias::Antialias) {
    unsafe {
      cairo_set_antialias(self.opaque, antialias);
    }
  }

  pub fn get_antialias(&mut self) -> antialias::Antialias {
    unsafe {
      let foreign_result = cairo_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_dash(&mut self, dashes: &mut [f64], offset: f64) {
    unsafe {
      cairo_set_dash(self.opaque, dashes.as_mut_ptr(), dashes.len() as libc::c_int, offset);
    }
  }

  pub fn get_dash_count(&mut self) -> libc::c_int {
    unsafe {
      let foreign_result = cairo_get_dash_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_dash(&mut self) -> (Vec<f64>, f64) {
    unsafe {
      use std::intrinsics;
      use std::iter::repeat;
      use num::traits::Zero;
      let dashes_len = self.get_dash_count() as usize;
      let mut dashes:Vec<f64> = repeat(Zero::zero()).take(dashes_len).collect();
      let mut offset:f64 = intrinsics::init();
      cairo_get_dash(self.opaque, dashes.as_mut_ptr(), &mut offset);
      return (dashes, offset);
    }
  }

  pub fn set_fill_rule(&mut self, fill_rule: fill_rule::FillRule) {
    unsafe {
      cairo_set_fill_rule(self.opaque, fill_rule);
    }
  }

  pub fn get_fill_rule(&mut self) -> fill_rule::FillRule {
    unsafe {
      let foreign_result = cairo_get_fill_rule(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_cap(&mut self, line_cap: line_cap::LineCap) {
    unsafe {
      cairo_set_line_cap(self.opaque, line_cap);
    }
  }

  pub fn get_line_cap(&mut self) -> line_cap::LineCap {
    unsafe {
      let foreign_result = cairo_get_line_cap(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_join(&mut self, line_join: line_join::LineJoin) {
    unsafe {
      cairo_set_line_join(self.opaque, line_join);
    }
  }

  pub fn get_line_join(&mut self) -> line_join::LineJoin {
    unsafe {
      let foreign_result = cairo_get_line_join(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_width(&mut self, width: f64) {
    unsafe {
      cairo_set_line_width(self.opaque, width);
    }
  }

  pub fn get_line_width(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_line_width(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_miter_limit(&mut self, limit: f64) {
    unsafe {
      cairo_set_miter_limit(self.opaque, limit);
    }
  }

  pub fn get_miter_limit(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_miter_limit(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_operator(&mut self, operator: operator::Operator) {
    unsafe {
      cairo_set_operator(self.opaque, operator);
    }
  }

  pub fn get_operator(&mut self) -> operator::Operator {
    unsafe {
      let foreign_result = cairo_get_operator(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_tolerance(&mut self, tolerance: f64) {
    unsafe {
      cairo_set_tolerance(self.opaque, tolerance);
    }
  }

  pub fn get_tolerance(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_tolerance(self.opaque);
      return foreign_result;
    }
  }

  pub fn clip(&mut self) {
    unsafe {
      cairo_clip(self.opaque);
    }
  }

  pub fn clip_preserve(&mut self) {
    unsafe {
      cairo_clip_preserve(self.opaque);
    }
  }

  pub fn clip_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      use std::intrinsics;
      let mut x1:f64 = intrinsics::init();
      let mut y1:f64 = intrinsics::init();
      let mut x2:f64 = intrinsics::init();
      let mut y2:f64 = intrinsics::init();
      cairo_clip_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_clip(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_clip(self.opaque, x, y);
      return foreign_result;
    }
  }

  pub fn reset_clip(&mut self) {
    unsafe {
      cairo_reset_clip(self.opaque);
    }
  }

  pub fn fill(&mut self) {
    unsafe {
      cairo_fill(self.opaque);
    }
  }

  pub fn fill_preserve(&mut self) {
    unsafe {
      cairo_fill_preserve(self.opaque);
    }
  }

  pub fn fill_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      use std::intrinsics;
      let mut x1:f64 = intrinsics::init();
      let mut y1:f64 = intrinsics::init();
      let mut x2:f64 = intrinsics::init();
      let mut y2:f64 = intrinsics::init();
      cairo_fill_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_fill(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_fill(self.opaque, x, y);
      return foreign_result;
    }
  }

  pub fn mask(&mut self, pattern: &mut pattern::Pattern) {
    unsafe {
      cairo_mask(self.opaque, pattern.opaque);
    }
  }

  pub fn mask_surface(&mut self, surface: &mut surface::Surface, surface_x: f64, surface_y: f64) {
    unsafe {
      cairo_mask_surface(self.opaque, surface.opaque, surface_x, surface_y);
    }
  }

  pub fn paint(&mut self) {
    unsafe {
      cairo_paint(self.opaque);
    }
  }

  pub fn paint_with_alpha(&mut self, alpha: f64) {
    unsafe {
      cairo_paint_with_alpha(self.opaque, alpha);
    }
  }

  pub fn stroke(&mut self) {
    unsafe {
      cairo_stroke(self.opaque);
    }
  }

  pub fn stroke_preserve(&mut self) {
    unsafe {
      cairo_stroke_preserve(self.opaque);
    }
  }

  pub fn stroke_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      use std::intrinsics;
      let mut x1:f64 = intrinsics::init();
      let mut y1:f64 = intrinsics::init();
      let mut x2:f64 = intrinsics::init();
      let mut y2:f64 = intrinsics::init();
      cairo_stroke_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_stroke(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_stroke(self.opaque, x, y);
      return foreign_result;
    }
  }

  pub fn copy_page(&mut self) {
    unsafe {
      cairo_copy_page(self.opaque);
    }
  }

  pub fn show_page(&mut self) {
    unsafe {
      cairo_show_page(self.opaque);
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn copy_path(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path(self.opaque);
      return path::Path { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn copy_path_flat(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path_flat(self.opaque);
      return path::Path { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn append_path(&mut self, path: &path::Path) {
    unsafe {
      cairo_append_path(self.opaque, path.opaque);
    }
  }

  pub fn has_current_point(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_has_current_point(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_current_point(&mut self) -> (f64, f64) {
    unsafe {
      use std::intrinsics;
      let mut x:f64 = intrinsics::init();
      let mut y:f64 = intrinsics::init();
      cairo_get_current_point(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn new_path(&mut self) {
    unsafe {
      cairo_new_path(self.opaque);
    }
  }

  pub fn new_sub_path(&mut self) {
    unsafe {
      cairo_new_sub_path(self.opaque);
    }
  }

  pub fn close_path(&mut self) {
    unsafe {
      cairo_close_path(self.opaque);
    }
  }

  pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }

  pub fn arc_negative(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc_negative(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }

  pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe {
      cairo_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
    }
  }

  pub fn line_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_line_to(self.opaque, x, y);
    }
  }

  pub fn move_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_move_to(self.opaque, x, y);
    }
  }

  pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_rectangle(self.opaque, x, y, width, height);
    }
  }

  pub fn glyph_path(&mut self, glyphs: &mut [font::Glyph]) {
    unsafe {
      cairo_glyph_path(self.opaque, glyphs.as_mut_ptr(), glyphs.len() as libc::c_int);
    }
  }

  pub fn text_path(&mut self, text_path: &str) {
    unsafe {
      use std::ffi::CString;
      cairo_text_path(self.opaque, CString::from_slice(text_path.as_bytes()).as_ptr() as *mut i8);
    }
  }

  pub fn rel_curve_to(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
    unsafe {
      cairo_rel_curve_to(self.opaque, dx1, dy1, dx2, dy2, dx3, dy3);
    }
  }

  pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_line_to(self.opaque, dx, dy);
    }
  }

  pub fn rel_move_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_move_to(self.opaque, dx, dy);
    }
  }

  pub fn path_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      use std::intrinsics;
      let mut x1:f64 = intrinsics::init();
      let mut y1:f64 = intrinsics::init();
      let mut x2:f64 = intrinsics::init();
      let mut y2:f64 = intrinsics::init();
      cairo_path_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn translate(&mut self, tx: f64, ty: f64) {
    unsafe {
      cairo_translate(self.opaque, tx, ty);
    }
  }

  pub fn scale(&mut self, sx: f64, sy: f64) {
    unsafe {
      cairo_scale(self.opaque, sx, sy);
    }
  }

  pub fn rotate(&mut self, angle: f64) {
    unsafe {
      cairo_rotate(self.opaque, angle);
    }
  }

  pub fn transform(&mut self, matrix: &mut matrix::Matrix) {
    unsafe {
      cairo_transform(self.opaque, matrix);
    }
  }

  pub fn set_matrix(&mut self, matrix: &mut matrix::Matrix) {
    unsafe {
      cairo_set_matrix(self.opaque, matrix);
    }
  }

  pub fn get_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      use std::intrinsics;
      let mut matrix:matrix::Matrix = intrinsics::init();
      cairo_get_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  pub fn identity_matrix(&mut self) {
    unsafe {
      cairo_identity_matrix(self.opaque);
    }
  }

  pub fn user_to_device(&mut self, x: f64, y: f64) -> (f64, f64) {
    unsafe {
      let mut x:f64 = x;
      let mut y:f64 = y;
      cairo_user_to_device(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn user_to_device_distance(&mut self, dx: f64, dy: f64) -> (f64, f64) {
    unsafe {
      let mut dx:f64 = dx;
      let mut dy:f64 = dy;
      cairo_user_to_device_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  pub fn device_to_user(&mut self, x: f64, y: f64) -> (f64, f64) {
    unsafe {
      let mut x:f64 = x;
      let mut y:f64 = y;
      cairo_device_to_user(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn device_to_user_distance(&mut self, dx: f64, dy: f64) -> (f64, f64) {
    unsafe {
      let mut dx:f64 = dx;
      let mut dy:f64 = dy;
      cairo_device_to_user_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  pub fn select_font_face(&mut self, family: &str, slant: font::slant::Slant, weight: font::weight::Weight) {
    unsafe {
      use std::ffi::CString;
      cairo_select_font_face(self.opaque, CString::from_slice(family.as_bytes()).as_ptr() as *mut i8, slant, weight);
    }
  }

  pub fn set_font_size(&mut self, size: f64) {
    unsafe {
      cairo_set_font_size(self.opaque, size);
    }
  }

  pub fn set_font_matrix(&mut self, matrix: &mut matrix::Matrix) {
    unsafe {
      cairo_set_font_matrix(self.opaque, matrix);
    }
  }

  pub fn get_font_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      use std::intrinsics;
      let mut matrix:matrix::Matrix = intrinsics::init();
      cairo_get_font_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  pub fn set_font_options(&mut self, options: &mut font::Options) {
    unsafe {
      cairo_set_font_options(self.opaque, options.opaque);
    }
  }

  pub fn get_font_options(&mut self, options: &mut font::Options) {
    unsafe {
      cairo_get_font_options(self.opaque, options.opaque);
    }
  }

  pub fn set_font_face(&mut self, font_face: &mut font::FontFace) {
    unsafe {
      cairo_set_font_face(self.opaque, font_face.opaque);
    }
  }

  pub fn get_font_face(&mut self) -> font::FontFace {
    unsafe {
      let foreign_result = cairo_get_font_face(self.opaque);
      return font::FontFace { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn set_scaled_font(&mut self, scaled_font: &mut font::ScaledFont) {
    unsafe {
      cairo_set_scaled_font(self.opaque, scaled_font.opaque);
    }
  }

  pub fn get_scaled_font(&mut self) -> font::ScaledFont {
    unsafe {
      let foreign_result = cairo_get_scaled_font(self.opaque);
      return font::ScaledFont { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn show_text(&mut self, utf8: &str) {
    unsafe {
      use std::ffi::CString;
      cairo_show_text(self.opaque, CString::from_slice(utf8.as_bytes()).as_ptr() as *mut i8);
    }
  }

  pub fn show_glyphs(&mut self, glyphs: &[font::Glyph]) {
    unsafe {
      cairo_show_glyphs(self.opaque, glyphs.as_ptr() as *mut font::Glyph, glyphs.len() as libc::c_int);
    }
  }

  pub fn show_text_glyphs(&mut self, utf8: &str, glyphs: &mut [font::Glyph], clusters: &mut [font::Cluster], cluster_flags: font::cluster_flags::ClusterFlags) {
    unsafe {
      use std::ffi::CString;
      cairo_show_text_glyphs(self.opaque, CString::from_slice(utf8.as_bytes()).as_ptr() as *mut i8, -1, glyphs.as_mut_ptr(), glyphs.len() as libc::c_int, clusters.as_mut_ptr(), clusters.len() as libc::c_int, cluster_flags);
    }
  }

  pub fn font_extents(&mut self) -> font::FontExtents {
    unsafe {
      use std::intrinsics;
      let mut extents:font::FontExtents = intrinsics::init();
      cairo_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn text_extents(&mut self, utf8: &str) -> font::TextExtents {
    unsafe {
      use std::ffi::CString;
      use std::intrinsics;
      let mut extents:font::TextExtents = intrinsics::init();
      cairo_text_extents(self.opaque, CString::from_slice(utf8.as_bytes()).as_ptr() as *mut i8, &mut extents);
      return extents;
    }
  }

  pub fn glyph_extents(&mut self, glyphs: &[font::Glyph]) -> font::TextExtents {
    unsafe {
      use std::intrinsics;
      let mut extents:font::TextExtents = intrinsics::init();
      cairo_glyph_extents(self.opaque, glyphs.as_ptr() as *mut font::Glyph, glyphs.len() as libc::c_int, &mut extents);
      return extents;
    }
  }
}

extern {
  fn cairo_create(surface: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_status(self_arg: *mut libc::c_void) -> Status;
  fn cairo_save(self_arg: *mut libc::c_void);
  fn cairo_restore(self_arg: *mut libc::c_void);
  fn cairo_get_target(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_push_group(self_arg: *mut libc::c_void);
  fn cairo_push_group_with_content(self_arg: *mut libc::c_void, content: surface::content::Content);
  fn cairo_pop_group(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_pop_group_to_source(self_arg: *mut libc::c_void);
  fn cairo_get_group_target(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_set_source_rgb(self_arg: *mut libc::c_void, red: f64, green: f64, blue: f64);
  fn cairo_set_source_rgba(self_arg: *mut libc::c_void, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_set_source(self_arg: *mut libc::c_void, source: *mut libc::c_void);
  fn cairo_set_source_surface(self_arg: *mut libc::c_void, surface: *mut libc::c_void, x: f64, y: f64);
  fn cairo_get_source(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_set_antialias(self_arg: *mut libc::c_void, antialias: antialias::Antialias);
  fn cairo_get_antialias(self_arg: *mut libc::c_void) -> antialias::Antialias;
  fn cairo_set_dash(self_arg: *mut libc::c_void, dashes: *mut f64, dashes: libc::c_int, offset: f64);
  fn cairo_get_dash_count(self_arg: *mut libc::c_void) -> libc::c_int;
  fn cairo_get_dash(self_arg: *mut libc::c_void, dashes: *mut f64, offset: *mut f64);
  fn cairo_set_fill_rule(self_arg: *mut libc::c_void, fill_rule: fill_rule::FillRule);
  fn cairo_get_fill_rule(self_arg: *mut libc::c_void) -> fill_rule::FillRule;
  fn cairo_set_line_cap(self_arg: *mut libc::c_void, line_cap: line_cap::LineCap);
  fn cairo_get_line_cap(self_arg: *mut libc::c_void) -> line_cap::LineCap;
  fn cairo_set_line_join(self_arg: *mut libc::c_void, line_join: line_join::LineJoin);
  fn cairo_get_line_join(self_arg: *mut libc::c_void) -> line_join::LineJoin;
  fn cairo_set_line_width(self_arg: *mut libc::c_void, width: f64);
  fn cairo_get_line_width(self_arg: *mut libc::c_void) -> f64;
  fn cairo_set_miter_limit(self_arg: *mut libc::c_void, limit: f64);
  fn cairo_get_miter_limit(self_arg: *mut libc::c_void) -> f64;
  fn cairo_set_operator(self_arg: *mut libc::c_void, operator: operator::Operator);
  fn cairo_get_operator(self_arg: *mut libc::c_void) -> operator::Operator;
  fn cairo_set_tolerance(self_arg: *mut libc::c_void, tolerance: f64);
  fn cairo_get_tolerance(self_arg: *mut libc::c_void) -> f64;
  fn cairo_clip(self_arg: *mut libc::c_void);
  fn cairo_clip_preserve(self_arg: *mut libc::c_void);
  fn cairo_clip_extents(self_arg: *mut libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_clip(self_arg: *mut libc::c_void, x: f64, y: f64) -> bool;
  fn cairo_reset_clip(self_arg: *mut libc::c_void);
  fn cairo_fill(self_arg: *mut libc::c_void);
  fn cairo_fill_preserve(self_arg: *mut libc::c_void);
  fn cairo_fill_extents(self_arg: *mut libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_fill(self_arg: *mut libc::c_void, x: f64, y: f64) -> bool;
  fn cairo_mask(self_arg: *mut libc::c_void, pattern: *mut libc::c_void);
  fn cairo_mask_surface(self_arg: *mut libc::c_void, surface: *mut libc::c_void, surface_x: f64, surface_y: f64);
  fn cairo_paint(self_arg: *mut libc::c_void);
  fn cairo_paint_with_alpha(self_arg: *mut libc::c_void, alpha: f64);
  fn cairo_stroke(self_arg: *mut libc::c_void);
  fn cairo_stroke_preserve(self_arg: *mut libc::c_void);
  fn cairo_stroke_extents(self_arg: *mut libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_stroke(self_arg: *mut libc::c_void, x: f64, y: f64) -> bool;
  fn cairo_copy_page(self_arg: *mut libc::c_void);
  fn cairo_show_page(self_arg: *mut libc::c_void);
  fn cairo_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
  fn cairo_copy_path(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_copy_path_flat(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_append_path(self_arg: *mut libc::c_void, path: *mut libc::c_void);
  fn cairo_has_current_point(self_arg: *mut libc::c_void) -> bool;
  fn cairo_get_current_point(self_arg: *mut libc::c_void, x: *mut f64, y: *mut f64);
  fn cairo_new_path(self_arg: *mut libc::c_void);
  fn cairo_new_sub_path(self_arg: *mut libc::c_void);
  fn cairo_close_path(self_arg: *mut libc::c_void);
  fn cairo_arc(self_arg: *mut libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_arc_negative(self_arg: *mut libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_curve_to(self_arg: *mut libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
  fn cairo_line_to(self_arg: *mut libc::c_void, x: f64, y: f64);
  fn cairo_move_to(self_arg: *mut libc::c_void, x: f64, y: f64);
  fn cairo_rectangle(self_arg: *mut libc::c_void, x: f64, y: f64, width: f64, height: f64);
  fn cairo_glyph_path(self_arg: *mut libc::c_void, glyphs: *mut font::Glyph, glyphs: libc::c_int);
  fn cairo_text_path(self_arg: *mut libc::c_void, text_path: *mut i8);
  fn cairo_rel_curve_to(self_arg: *mut libc::c_void, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64);
  fn cairo_rel_line_to(self_arg: *mut libc::c_void, dx: f64, dy: f64);
  fn cairo_rel_move_to(self_arg: *mut libc::c_void, dx: f64, dy: f64);
  fn cairo_path_extents(self_arg: *mut libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_translate(self_arg: *mut libc::c_void, tx: f64, ty: f64);
  fn cairo_scale(self_arg: *mut libc::c_void, sx: f64, sy: f64);
  fn cairo_rotate(self_arg: *mut libc::c_void, angle: f64);
  fn cairo_transform(self_arg: *mut libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_set_matrix(self_arg: *mut libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_get_matrix(self_arg: *mut libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_identity_matrix(self_arg: *mut libc::c_void);
  fn cairo_user_to_device(self_arg: *mut libc::c_void, x: *mut f64, y: *mut f64);
  fn cairo_user_to_device_distance(self_arg: *mut libc::c_void, dx: *mut f64, dy: *mut f64);
  fn cairo_device_to_user(self_arg: *mut libc::c_void, x: *mut f64, y: *mut f64);
  fn cairo_device_to_user_distance(self_arg: *mut libc::c_void, dx: *mut f64, dy: *mut f64);
  fn cairo_select_font_face(self_arg: *mut libc::c_void, family: *mut i8, slant: font::slant::Slant, weight: font::weight::Weight);
  fn cairo_set_font_size(self_arg: *mut libc::c_void, size: f64);
  fn cairo_set_font_matrix(self_arg: *mut libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_get_font_matrix(self_arg: *mut libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_set_font_options(self_arg: *mut libc::c_void, options: *mut libc::c_void);
  fn cairo_get_font_options(self_arg: *mut libc::c_void, options: *mut libc::c_void);
  fn cairo_set_font_face(self_arg: *mut libc::c_void, font_face: *mut libc::c_void);
  fn cairo_get_font_face(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_set_scaled_font(self_arg: *mut libc::c_void, scaled_font: *mut libc::c_void);
  fn cairo_get_scaled_font(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_show_text(self_arg: *mut libc::c_void, utf8: *mut i8);
  fn cairo_show_glyphs(self_arg: *mut libc::c_void, glyphs: *mut font::Glyph, glyphs: libc::c_int);
  fn cairo_show_text_glyphs(self_arg: *mut libc::c_void, utf8: *mut i8, utf8_len: libc::c_int, glyphs: *mut font::Glyph, glyphs: libc::c_int, clusters: *mut font::Cluster, clusters: libc::c_int, cluster_flags: font::cluster_flags::ClusterFlags);
  fn cairo_font_extents(self_arg: *mut libc::c_void, extents: *mut font::FontExtents);
  fn cairo_text_extents(self_arg: *mut libc::c_void, utf8: *mut i8, extents: *mut font::TextExtents);
  fn cairo_glyph_extents(self_arg: *mut libc::c_void, glyphs: *mut font::Glyph, glyphs: libc::c_int, extents: *mut font::TextExtents);
}

impl Clone for Cairo {
  fn clone(&self) -> Cairo {
    unsafe {
      let foreign_result = cairo_reference(self.opaque);
      return Cairo { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl Drop for Cairo {
  fn drop(&mut self) {
    unsafe {
      cairo_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_destroy(self_arg: *mut libc::c_void);
}

use libc;
use std;

pub mod content;
pub mod surface_type;
pub mod format;

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum SVGVersion {
  SVGVersion_1_1 = 0,
  SVGVersion_1_2 = 1
}

#[repr(C)]
pub struct Surface {
  pub opaque: *mut libc::c_void
}

impl Surface {
  pub fn create_similar_image(format: format::Format, width: libc::c_int, height: libc::c_int) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_similar_image(format, width, height);
      return Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn create_for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_for_rectangle(x, y, width, height);
      return Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_surface_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      cairo_surface_finish(self.opaque);
    }
  }

  pub fn flush(&mut self) {
    unsafe {
      cairo_surface_flush(self.opaque);
    }
  }

  pub fn get_device(&mut self) -> super::device::Device {
    unsafe {
      let foreign_result = cairo_surface_get_device(self.opaque);
      return super::device::Device { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_font_options(&mut self, options: &mut super::font::Options) {
    unsafe {
      cairo_surface_get_font_options(self.opaque, options.opaque);
    }
  }

  pub fn get_content(&mut self) -> content::Content {
    unsafe {
      let foreign_result = cairo_surface_get_content(self.opaque);
      return foreign_result;
    }
  }

  pub fn mark_dirty(&mut self) {
    unsafe {
      cairo_surface_mark_dirty(self.opaque);
    }
  }

  pub fn mark_dirty_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_surface_mark_dirty_rectangle(self.opaque, x, y, width, height);
    }
  }

  pub fn set_device_offset(&mut self, x_offset: f64, y_offset: f64) {
    unsafe {
      cairo_surface_set_device_offset(self.opaque, x_offset, y_offset);
    }
  }

  pub fn get_device_offset(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_offset:f64 = std::intrinsics::init();
      let mut y_offset:f64 = std::intrinsics::init();
      cairo_surface_get_device_offset(self.opaque, &mut x_offset, &mut y_offset);
      return (x_offset, y_offset);
    }
  }

  pub fn set_fallback_resolution(&mut self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
    unsafe {
      cairo_surface_set_fallback_resolution(self.opaque, x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  pub fn get_fallback_resolution(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_pixels_per_inch:f64 = std::intrinsics::init();
      let mut y_pixels_per_inch:f64 = std::intrinsics::init();
      cairo_surface_get_fallback_resolution(self.opaque, &mut x_pixels_per_inch, &mut y_pixels_per_inch);
      return (x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  pub fn get_type(&mut self) -> surface_type::SurfaceType {
    unsafe {
      let foreign_result = cairo_surface_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_surface_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn copy_page(&mut self) {
    unsafe {
      cairo_surface_copy_page(self.opaque);
    }
  }

  pub fn show_page(&mut self) {
    unsafe {
      cairo_surface_show_page(self.opaque);
    }
  }

  pub fn create_image(format: format::Format, width: libc::c_int, height: libc::c_int) -> Surface {
    unsafe {
      let foreign_result = cairo_image_surface_create(format, width, height);
      return Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn get_format(&mut self) -> format::Format {
    unsafe {
      let foreign_result = cairo_image_surface_get_format(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_width(&mut self) -> libc::c_int {
    unsafe {
      let foreign_result = cairo_image_surface_get_width(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_height(&mut self) -> libc::c_int {
    unsafe {
      let foreign_result = cairo_image_surface_get_height(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_stride(&mut self) -> libc::c_int {
    unsafe {
      let foreign_result = cairo_image_surface_get_stride(self.opaque);
      return foreign_result;
    }
  }

  pub fn create_from_png(filename: &str) -> Surface {
    use std::ffi::CString;
    let cstr_filename = CString::new(filename.as_bytes()).unwrap(); // TODO
    // Fails if the filename has any internal null bytes.
    unsafe {
      let foreign_result = cairo_image_surface_create_from_png(cstr_filename.as_ptr());
      return Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn write_to_png(&mut self, filename: &str) -> super::Status {
    use std::ffi::CString;
    let cstr_filename = CString::new(filename.as_bytes()).unwrap(); // TODO
    unsafe {
      let foreign_result = cairo_surface_write_to_png(self.opaque, cstr_filename.as_ptr());
      return foreign_result;
    }
  }

  pub fn create_svg(&mut self, filename: &str, width: f64, height: f64) {
    use std::ffi::CString;
    let cstr_filename = CString::new(filename.as_bytes()).unwrap(); // TODO
    unsafe {
      cairo_svg_surface_create(self.opaque, cstr_filename.as_ptr(), width, height);
    }
  }

  pub fn restrict_to_svg_version(&mut self, version: SVGVersion) {
    unsafe {
      cairo_svg_surface_restrict_to_version(self.opaque, version);
    }
  }

  pub fn svg_version_to_string(version: SVGVersion) -> std::string::String {
    unsafe {
      let foreign_result = cairo_svg_version_to_string(version);
      return std::string::String::from_utf8_lossy(std::ffi::CStr::from_ptr(foreign_result).to_bytes()).into_owned()
    }
  }
}

extern {
  fn cairo_surface_create_similar_image(format: format::Format, width: libc::c_int, height: libc::c_int) -> *mut libc::c_void;
  fn cairo_surface_create_for_rectangle(x: f64, y: f64, width: f64, height: f64) -> *mut libc::c_void;
  fn cairo_surface_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_surface_finish(self_arg: *mut libc::c_void);
  fn cairo_surface_flush(self_arg: *mut libc::c_void);
  fn cairo_surface_get_device(self_arg: *mut libc::c_void) -> *mut libc::c_void;
  fn cairo_surface_get_font_options(self_arg: *mut libc::c_void, options: *mut libc::c_void);
  fn cairo_surface_get_content(self_arg: *mut libc::c_void) -> content::Content;
  fn cairo_surface_mark_dirty(self_arg: *mut libc::c_void);
  fn cairo_surface_mark_dirty_rectangle(self_arg: *mut libc::c_void, x: f64, y: f64, width: f64, height: f64);
  fn cairo_surface_set_device_offset(self_arg: *mut libc::c_void, x_offset: f64, y_offset: f64);
  fn cairo_surface_get_device_offset(self_arg: *mut libc::c_void, x_offset: *mut f64, y_offset: *mut f64);
  fn cairo_surface_set_fallback_resolution(self_arg: *mut libc::c_void, x_pixels_per_inch: f64, y_pixels_per_inch: f64);
  fn cairo_surface_get_fallback_resolution(self_arg: *mut libc::c_void, x_pixels_per_inch: *mut f64, y_pixels_per_inch: *mut f64);
  fn cairo_surface_get_type(self_arg: *mut libc::c_void) -> surface_type::SurfaceType;
  fn cairo_surface_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
  fn cairo_surface_copy_page(self_arg: *mut libc::c_void);
  fn cairo_surface_show_page(self_arg: *mut libc::c_void);
  fn cairo_image_surface_create(format: format::Format, width: libc::c_int, height: libc::c_int) -> *mut libc::c_void;
  fn cairo_image_surface_get_format(self_arg: *mut libc::c_void) -> format::Format;
  fn cairo_image_surface_get_width(self_arg: *mut libc::c_void) -> libc::c_int;
  fn cairo_image_surface_get_height(self_arg: *mut libc::c_void) -> libc::c_int;
  fn cairo_image_surface_get_stride(self_arg: *mut libc::c_void) -> libc::c_int;
  fn cairo_image_surface_create_from_png(filename: *const libc::c_char) -> *mut libc::c_void;
  fn cairo_surface_write_to_png(self_arg: *mut libc::c_void, filename: *const libc::c_char) -> super::Status;
  fn cairo_svg_surface_create(self_arg: *mut libc::c_void, filename: *const libc::c_char, width: f64, height: f64);
  fn cairo_svg_surface_restrict_to_version(self_arg: *mut libc::c_void, version: SVGVersion);
  fn cairo_svg_version_to_string(version: SVGVersion) -> *const libc::c_char;
}

impl std::clone::Clone for Surface {
  fn clone(&self) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_reference(self.opaque);
      return Surface { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_surface_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for Surface {
  fn drop(&mut self) {
    unsafe {
      cairo_surface_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_surface_destroy(self_arg: *mut libc::c_void);
}

use libc;
use std;

pub struct Path {
  pub opaque: *mut libc::c_void
}

impl std::ops::Drop for Path {
  fn drop(&mut self) {
    unsafe {
      cairo_path_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_path_destroy(self_arg: *mut libc::c_void);
}

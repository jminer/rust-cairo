use libc;
use std;

pub mod device_type;

pub struct Device {
  pub opaque: *mut libc::c_void
}

impl Device {
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      cairo_device_finish(self.opaque);
    }
  }

  pub fn flush(&mut self) {
    unsafe {
      cairo_device_flush(self.opaque);
    }
  }

  pub fn get_type(&mut self) -> device_type::DeviceType {
    unsafe {
      let foreign_result = cairo_device_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> libc::c_uint {
    unsafe {
      let foreign_result = cairo_device_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn acquire(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_acquire(self.opaque);
      return foreign_result;
    }
  }

  pub fn release(&mut self) {
    unsafe {
      cairo_device_release(self.opaque);
    }
  }
}

extern {
  fn cairo_device_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_device_finish(self_arg: *mut libc::c_void);
  fn cairo_device_flush(self_arg: *mut libc::c_void);
  fn cairo_device_get_type(self_arg: *mut libc::c_void) -> device_type::DeviceType;
  fn cairo_device_get_reference_count(self_arg: *mut libc::c_void) -> libc::c_uint;
  fn cairo_device_acquire(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_device_release(self_arg: *mut libc::c_void);
}

impl std::clone::Clone for Device {
  fn clone(&self) -> Device {
    unsafe {
      let foreign_result = cairo_device_reference(self.opaque);
      return Device { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_device_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for Device {
  fn drop(&mut self) {
    unsafe {
      cairo_device_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_device_destroy(self_arg: *mut libc::c_void);
}

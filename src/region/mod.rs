use libc;
use std;

pub mod overlap;

pub struct Region {
  pub opaque: *mut libc::c_void
}

impl Region {
  pub fn create() -> Region {
    unsafe {
      let foreign_result = cairo_region_create();
      return Region { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn create_rectangle(rectangle: &mut Rectangle) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangle(rectangle);
      return Region { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn create_rectangles(rectangles: &[Rectangle]) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangles(rectangles.as_ptr() as *mut Rectangle, rectangles.len() as libc::c_int);
      return Region { opaque: foreign_result as *mut libc::c_void };
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_extents(&mut self) -> Rectangle {
    unsafe {
      let mut extents:Rectangle = std::intrinsics::init();
      cairo_region_get_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn num_rectangles(&mut self) -> libc::c_int {
    unsafe {
      let foreign_result = cairo_region_num_rectangles(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_rectangle(&mut self, nth: libc::c_int) -> Rectangle {
    unsafe {
      let mut rectangle:Rectangle = std::intrinsics::init();
      cairo_region_get_rectangle(self.opaque, nth, &mut rectangle);
      return rectangle;
    }
  }

  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_region_is_empty(self.opaque);
      return foreign_result;
    }
  }

  pub fn contains_point(&mut self, x: libc::c_int, y: libc::c_int) -> bool {
    unsafe {
      let foreign_result = cairo_region_contains_point(self.opaque, x, y);
      return foreign_result;
    }
  }

  pub fn contains_rectangle(&mut self, rectangle: &mut Rectangle) -> overlap::Overlap {
    unsafe {
      let foreign_result = cairo_region_contains_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn equal(&mut self, other: &Region) -> bool {
    unsafe {
      let foreign_result = cairo_region_equal(self.opaque, other.opaque);
      return foreign_result;
    }
  }

  pub fn translate(&mut self, dx: libc::c_int, dy: libc::c_int) {
    unsafe {
      cairo_region_translate(self.opaque, dx, dy);
    }
  }

  pub fn intersect_rectangle(&mut self, rectangle: &mut Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_intersect_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn subtract(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract(self.opaque, region.opaque);
      return foreign_result;
    }
  }

  pub fn subtract_rectangle(&mut self, rectangle: &mut Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn union(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union(self.opaque, region.opaque);
      return foreign_result;
    }
  }

  pub fn union_rectangle(&mut self, rectangle: &mut Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn xor(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_xor(self.opaque, region.opaque);
      return foreign_result;
    }
  }

  pub fn xor_rectangle(&mut self, rectangle: &mut Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_xor_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_region_create() -> *mut libc::c_void;
  fn cairo_region_create_rectangle(rectangle: *mut Rectangle) -> *mut libc::c_void;
  fn cairo_region_create_rectangles(rectangles: *mut Rectangle, rectangles: libc::c_int) -> *mut libc::c_void;
  fn cairo_region_status(self_arg: *mut libc::c_void) -> super::Status;
  fn cairo_region_get_extents(self_arg: *mut libc::c_void, extents: *mut Rectangle);
  fn cairo_region_num_rectangles(self_arg: *mut libc::c_void) -> libc::c_int;
  fn cairo_region_get_rectangle(self_arg: *mut libc::c_void, nth: libc::c_int, rectangle: *mut Rectangle);
  fn cairo_region_is_empty(self_arg: *mut libc::c_void) -> bool;
  fn cairo_region_contains_point(self_arg: *mut libc::c_void, x: libc::c_int, y: libc::c_int) -> bool;
  fn cairo_region_contains_rectangle(self_arg: *mut libc::c_void, rectangle: *mut Rectangle) -> overlap::Overlap;
  fn cairo_region_equal(self_arg: *mut libc::c_void, other: *mut libc::c_void) -> bool;
  fn cairo_region_translate(self_arg: *mut libc::c_void, dx: libc::c_int, dy: libc::c_int);
  fn cairo_region_intersect_rectangle(self_arg: *mut libc::c_void, rectangle: *mut Rectangle) -> super::Status;
  fn cairo_region_subtract(self_arg: *mut libc::c_void, region: *mut libc::c_void) -> super::Status;
  fn cairo_region_subtract_rectangle(self_arg: *mut libc::c_void, rectangle: *mut Rectangle) -> super::Status;
  fn cairo_region_union(self_arg: *mut libc::c_void, region: *mut libc::c_void) -> super::Status;
  fn cairo_region_union_rectangle(self_arg: *mut libc::c_void, rectangle: *mut Rectangle) -> super::Status;
  fn cairo_region_xor(self_arg: *mut libc::c_void, region: *mut libc::c_void) -> super::Status;
  fn cairo_region_xor_rectangle(self_arg: *mut libc::c_void, rectangle: *mut Rectangle) -> super::Status;
}

impl std::clone::Clone for Region {
  fn clone(&self) -> Region {
    unsafe {
      let foreign_result = cairo_region_reference(self.opaque);
      return Region { opaque: foreign_result as *mut libc::c_void };
    }
  }
}

extern {
  fn cairo_region_reference(self_arg: *mut libc::c_void) -> *mut libc::c_void;
}

impl std::ops::Drop for Region {
  fn drop(&mut self) {
    unsafe {
      cairo_region_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_region_destroy(self_arg: *mut libc::c_void);
}


pub struct Rectangle {
  x: libc::c_int,
  y: libc::c_int,
  width: libc::c_int,
  height: libc::c_int
}

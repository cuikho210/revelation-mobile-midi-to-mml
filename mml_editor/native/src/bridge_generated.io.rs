use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_parse_midi(
    port_: i64,
    bytes: *mut wire_uint_8_list,
    is_auto_split: bool,
    to_merge: *mut wire_list___record__usize_usize,
) {
    wire_parse_midi_impl(port_, bytes, is_auto_split, to_merge)
}

#[no_mangle]
pub extern "C" fn wire_get_track_length(port_: i64, bytes: *mut wire_uint_8_list) {
    wire_get_track_length_impl(port_, bytes)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_list___record__usize_usize_0(
    len: i32,
) -> *mut wire_list___record__usize_usize {
    let wrap = wire_list___record__usize_usize {
        ptr: support::new_leak_vec_ptr(<wire___record__usize_usize>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<(usize, usize)> for wire___record__usize_usize {
    fn wire2api(self) -> (usize, usize) {
        (self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<Vec<(usize, usize)>> for *mut wire_list___record__usize_usize {
    fn wire2api(self) -> Vec<(usize, usize)> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire___record__usize_usize {
    field0: usize,
    field1: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list___record__usize_usize {
    ptr: *mut wire___record__usize_usize,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire___record__usize_usize {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire___record__usize_usize {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}

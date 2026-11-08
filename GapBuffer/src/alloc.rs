pub fn alloc_char(count: usize) -> *mut char {
    let layout = std::alloc::Layout::array::<char>(count).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    ptr as *mut char
}

pub fn dealloc_char(ptr: *mut char, count: usize) {
    let layout = std::alloc::Layout::array::<char>(count).unwrap();
    unsafe {std::alloc::dealloc(ptr as *mut u8, layout)}
}
use std::alloc::alloc;

struct GapBuffer {
    left: usize,
    right: usize,
    capacity: usize,
    data: *mut char, 
}

fn initialize_buffer(size: usize) -> GapBuffer {
    GapBuffer {
        left: 0,
        right: size,
        capacity: size,
        data: alloc_char(size),
    }
}

fn debug(gb: &GapBuffer) {
    // TODO
}

fn type_char(gb: &mut GapBuffer, c: char) {
    // TODO - must be O(1) amortized
}

fn insert_text(gb: &mut GapBuffer, text: &str) {
    for c in text.chars() {
        type_char(gb, c);
    }
}

fn move_left(gb: &mut GapBuffer) {
    if gb.left == 0 {
        return;
    }
    gb.left -= 1;
    gb.right -= 1;
    unsafe {
        *gb.data.add(gb.right) = *gb.data.add(gb.left);
    }
}

fn move_right(gb: &mut GapBuffer) {
    if gb.right == gb.capacity {
        return;
    }
    gb.left += 1;
    gb.right += 1;
    unsafe {
        *gb.data.add(gb.left) = *gb.data.add(gb.right);
    }
}

fn backspace(gb: &mut GapBuffer) {
    if gb.left == 0{
        return;
    }
    unsafe{
        *gb.data.add(gb.left) = '\0';
    }
    
}

fn main() {
    let mut gb = initialize_buffer(10);
    insert_text(&mut gb, "This!");
    debug(&gb);
    move_left(&mut gb);
    debug(&gb);
    insert_text(&mut gb, " is a ni");
    debug(&gb);
    insert_text(&mut gb, "ce Rap buffer");
    debug(&gb);
    for _ in 0..9 {
        move_left(&mut gb);
        debug(&gb);
    }
    backspace(&mut gb);
    type_char(&mut gb, 'G');
    for _ in 0..10 {
        move_right(&mut gb);
        debug(&gb);
    }
    insert_text(&mut gb, " ;-) ");
    debug(&gb);
}

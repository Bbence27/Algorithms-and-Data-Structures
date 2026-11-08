mod alloc;

struct GapBuffer {
    left: usize,
    right: usize,
    capacity: usize,
    data: *mut char,
}

impl GapBuffer {
    fn resize(&mut self) {
        let new_data = alloc::alloc_char(self.capacity*2);
        for i in 0..self.left {
            unsafe {*new_data.add(i) = *self.data.add(i);}
        }
        for i in self.right..self.capacity {
            unsafe {*new_data.add(i + self.capacity) = *self.data.add(i);}
        }
        alloc::dealloc_char(self.data, self.capacity);
        self.data = new_data;
        self.right += self.capacity;
        self.capacity *= 2;
    }
}


fn initialize_buffer(size: usize) -> GapBuffer {
    GapBuffer {
            left: 0,
            right: size,
            capacity: size,
            data: alloc::alloc_char(size),
        }
    }

fn debug(gb: &GapBuffer) {
    for i in 0..gb.left {
        unsafe {
            print!("{}", *gb.data.add(i));
        }
    }
    for _i in gb.left..gb.right {
        print!("_");
    }
    for i in gb.right..gb.capacity {
        unsafe {
            print!("{}", *gb.data.add(i));
        }
    }
    println!();
}

fn type_char(gb: &mut GapBuffer, c: char) {
    if gb.left == gb.right {
        gb.resize();
    }
    unsafe {
        *gb.data.add(gb.left) = c;
    }
    gb.left += 1;
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
    unsafe {
        gb.left -= 1;
        gb.right -= 1;
        *gb.data.add(gb.right) = *gb.data.add(gb.left);
    }
}

fn move_right(gb: &mut GapBuffer) {
    if gb.right == gb.capacity {
        return;
    }
    unsafe {
        *gb.data.add(gb.left) = *gb.data.add(gb.right);
        gb.left += 1;
    }
    gb.right += 1;
}

fn backspace(gb: &mut GapBuffer) {
    if gb.left == 0 {
        return;
    }
    unsafe {
        gb.left -= 1;
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

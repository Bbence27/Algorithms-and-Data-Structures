use std::{print};

mod alloc;

struct GapBuffer {
    left: usize,
    right: usize,
    capacity: usize,
    data: *mut char, 
}

impl GapBuffer {
    fn resize(&mut self){
        let new_data_place = alloc::alloc_char(self.capacity*2);
        for i in 0..self.left{
            unsafe {
                *new_data_place.add(i) = *self.data.add(i);
            }
        }
        for c in self.right..self.capacity{
            unsafe {
                *new_data_place.add(c + self.capacity) = *self.data.add(c);
            }
        }
        alloc::dealloc_char(self.data, self.capacity);
        self.data = new_data_place;
        self.right += self.capacity;
        self.capacity = self.capacity * 2;
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
    let mut left_string = String::new();
    let middle_string = "_".repeat(gb.right-gb.left);
    let mut right_string = String::new();

    for i in 0..gb.left{
        unsafe {
            left_string = [left_string, (*gb.data.add(i)).to_string()].concat();
        }
    }
    
    for c in gb.right..gb.capacity{
        unsafe {
            right_string = [right_string, (*gb.data.add(c)).to_string()].concat();
        }
    }
    
    let output_string = [left_string,middle_string,right_string].concat();
    print!("{:?}",output_string);

    println!("")
}

fn type_char(gb: &mut GapBuffer, c: char) {
    if gb.left == gb.right{
        gb.resize();
    }
    unsafe{
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
    
    unsafe {
        *gb.data.add(gb.left) = *gb.data.add(gb.right);
    }
    gb.left += 1;
    gb.right += 1;
}

fn backspace(gb: &mut GapBuffer) {
    if gb.left == 0{
        return;
    }
    gb.left -= 1
    
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

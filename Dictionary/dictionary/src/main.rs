use std::fmt;

struct Dict {
    data: Vec<Option<(i32, i32)>>
}

#[inline]
fn parent(idx: usize) -> usize { (idx - 1) / 2 }
#[inline]
fn left(idx: usize) -> usize { 2 * idx + 1 }
#[inline]
fn right(idx: usize) -> usize { 2 * idx + 2 }

impl Dict {
    fn new() -> Dict {
        Dict { data: vec![] }
    }

    fn insert(&mut self, key: i32, value: i32) {
        let mut idx = 0;
        loop {
            if idx >= self.data.len() {
                self.data.resize(idx + 1, None);
            }
            match self.data[idx] {
                None => {
                    self.data[idx] = Some((key, value));
                    return;
                }
                Some((k, _)) if key == k => {
                    self.data[idx] = Some((key, value)); // update existing key
                    return;
                }
                Some((k, _)) if key < k => idx = left(idx),
                Some(_) => idx = right(idx),
            }
        }
    }

    fn get(&self, key: i32) -> Option<i32> {
        let mut idx = 0;
        while idx < self.data.len() {
            match self.data[idx] {
                None => return None,
                Some((k, v)) if key == k => return Some(v),
                Some((k, _)) if key < k => idx = left(idx),
                Some(_) => idx = right(idx),
            }
        }
        None
    }

    fn remove(&mut self, key: i32) -> Option<i32> {
        let mut idx = 0;
        while idx < self.data.len() {
            match self.data[idx] {
                None => return None,
                Some((k, _)) if key < k => idx = left(idx),
                Some((k, _)) if key > k => idx = right(idx),
                Some((_, v)) => {
                    self.remove_at(idx);
                    return Some(v);
                }
            }
        }
        None
    }

    fn has_node(&self, idx: usize) -> bool {
        idx < self.data.len() && self.data[idx].is_some()
    }

    fn shift_up(&mut self, from: usize, to: usize) {
        if !self.has_node(from) {
            if to < self.data.len() { self.data[to] = None; }
            return;
        }
        self.data[to] = self.data[from].take();
        self.shift_up(left(from), left(to));
        self.shift_up(right(from), right(to));
    }

    fn remove_at(&mut self, idx: usize) {
        let has_l = self.has_node(left(idx));
        let has_r = self.has_node(right(idx));

        if !has_l && !has_r {
            self.data[idx] = None;
        } else if has_l && !has_r {
            self.shift_up(left(idx), idx);
        } else if !has_l && has_r {
            self.shift_up(right(idx), idx);
        } else {
            let mut succ_idx = right(idx);
            while self.has_node(left(succ_idx)) {
                succ_idx = left(succ_idx);
            }
            self.data[idx] = self.data[succ_idx];
            self.shift_up(right(succ_idx), succ_idx);
        }
    }
}

impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut new_layer_idx = 1;
        for i in 0..self.data.len() {
            if i == new_layer_idx {
                write!(f, "|")?;
                new_layer_idx = left(new_layer_idx);
            }
            match self.data[i] {
                Some((k, v)) => write!(f, " {}:{} ", k, v)?,
                None => write!(f, " _ ")?,
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let mut d: Dict = Dict::new();

    for (k, v) in [(42, 1), (23, 2), (84, 3), (7, 4), (35, 5), (58, 6), (91, 7)] {
        d.insert(k, v);
    }

    println!("{}", d);

    for k in [42, 35, 99] {
        println!("get({}): {:?}", k, d.get(k));
    }

    println!("remove(42): {:?}", d.remove(42));
    println!("{}", d);
}
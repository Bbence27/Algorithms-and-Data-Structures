struct Node {
    data: u32,
    next: *mut Node,
    previous: *mut Node
}

impl Node {
    fn new(data:u32, next: *mut Node, previous: *mut Node) -> *mut Node {
        let layout = std::alloc::Layout::new::<Node>();
        unsafe {
            let ptr : *mut Node = std::alloc::alloc(layout) as *mut Node;
            (*ptr).data = data;
            (*ptr).next = next;
            (*ptr).previous = previous;
            ptr as *mut Node
        }
    }
}

struct DLL {
    head: *mut Node,
    tail: *mut Node
}

impl DLL {
    pub fn new() -> Self {
        DLL { 
            head : std::ptr::null_mut(),
            tail : std::ptr::null_mut()
        }
    }

    pub fn size(&self) -> usize {
        let mut current = self.head;
        let mut count = 0;
        while !current.is_null() {
            count += 1;
            unsafe { current = (*current).next; }
        }
        count
    }

    pub fn push_front(&mut self, value: u32) {
        let new_node = Node::new(value, self.head, std::ptr::null_mut());
    
        if self.head.is_null() {
            self.tail = new_node;
        } else {
            unsafe { 
            (*self.head).previous = new_node; 
            }
        }
        self.head = new_node;
    }

    pub fn push_back(&mut self, value: u32) {
        let new_node = Node::new(value, std::ptr::null_mut(), self.tail);
    
        if self.tail.is_null() {
            self.head = new_node;
        } else {
            unsafe { 
            (*self.tail).next = new_node; 
            }
        }
        self.tail = new_node;
    }

    pub fn pop_front(&mut self) -> Option<u32>{
        if self.head.is_null() {
            return None;
        }

        let layout = std::alloc::Layout::new::<Node>();
        let old_head = self.head;
        let data;

        unsafe {
            data = (*old_head).data;
            self.head = (*old_head).next;

            if self.head.is_null() {
                self.tail = std::ptr::null_mut();
            } else {
                (*self.head).previous = std::ptr::null_mut();
            }

            std::alloc::dealloc(old_head as *mut u8, layout);
        }
        Some(data)
    }

    pub fn pop_back(&mut self) -> Option<u32>{
        if self.tail.is_null() {
            return None;
        }

        let layout = std::alloc::Layout::new::<Node>();
        let old_tail = self.tail;
        let data;

        unsafe {
            data = (*old_tail).data;
            self.tail = (*old_tail).previous;

            if self.tail.is_null() {
                self.head = std::ptr::null_mut();
            } else {
                (*self.tail).next = std::ptr::null_mut();
            }

            std::alloc::dealloc(old_tail as *mut u8, layout);
        }
        Some(data)
    }
    /* 
    pub fn print_backwards(&self) {
        let mut current = self.tail;
        print!("TAIL({:p}) -> ", self.tail);
        unsafe {
            while !current.is_null() {
                print!("[{}, {:p}] -> ", (*current).data, (*current).previous);
                current = (*current).previous;
            }
        }
        println!("NULL");
    }
    */
}

impl Drop for DLL {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::new::<Node>();
        while !self.head.is_null() {
            let tmp = self.head;
            unsafe {
                self.head = (*self.head).next; 
                std::alloc::dealloc(tmp as *mut u8, layout)
            }
        }
    }
}

impl std::fmt::Display for DLL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head;        
        write!(f, "HEAD({:p}) -> ", self.head)?;
        unsafe {
            while !current.is_null() {
                write!(f, "[{}, {:p}] -> ", (*current).data, (*current).next)?;
                current = (*current).next;
            }
        } 
        write!(f, "NULL")
    }
}


fn main() {
    let mut dll1 = DLL::new();
    dll1.push_front(1);
    dll1.push_front(22);
    dll1.push_front(333);
    dll1.push_back(4444);
    println!("{}", dll1.size());
    println!("{}", dll1);
    dll1.pop_front();
    println!("{}", dll1.size());
    println!("{}", dll1);
    dll1.pop_back();
    println!("{}", dll1.size());
    println!("{}", dll1);
}
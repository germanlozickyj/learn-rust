struct MyArray<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyArray<T> {
    fn new(capacity: usize) -> Self {
        unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::array::<T>(capacity).unwrap());
            MyArray {
                data: ptr as *mut T,
                len: 0,
                capacity,
            }
        }
    }

    fn push(&mut self, value: T) {
        unsafe {
            if self.len >= self.capacity {
                let new_capacity = self.capacity * 2;
                let new_ptr = std::alloc::alloc(std::alloc::Layout::array::<T>(new_capacity).unwrap());
                std::ptr::copy_nonoverlapping(self.data, new_ptr as *mut T, self.len);
                std::alloc::dealloc(self.data as *mut u8, std::alloc::Layout::array::<T>(self.capacity).unwrap());
                self.data = new_ptr as *mut T;
                self.capacity = new_capacity;
            }

            std::ptr::write(self.data.add(self.len), value);
            self.len += 1;
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }

    fn free(self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, std::alloc::Layout::array::<T>(self.capacity).unwrap());
        }
    }
}

fn main() {
    let mut arr = MyArray::new(2);
    arr.push(1);
    arr.push(2);
    arr.push(3);
    
    if let Some(first) = arr.get(0) {
        println!("Primer valor: {}", first);
    }
    
    arr.free();
}

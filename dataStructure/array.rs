struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    fn get(&self, key: usize) -> Option<&T> {
        if key >= self.data.len() {
            return None;
        }
        Some(&self.data[key])
    }
    
    fn push(mut self, value: Option<&T>) {
        
    }
}

fn main() {
    let arr: Array<i32> = Array { data: Vec::new() };

    println!("{:?}", arr.get(1));
}

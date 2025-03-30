pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn push(&mut self, value: T) {
        self.stack.push(value)
    }
    fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.stack.pop()
        } else {
            None
        }
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Stack<T> {
    // add code here
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .stack
            .iter()
            .map(|x| format!("{}", x))
            .fold(String::new(), |acc, x| {
                if acc.is_empty() {
                    x
                } else {
                    acc + ", " + &x
                }
            });
        write!(f, "[{}]", elements)
    }
}

fn main() {
    let mut a = Stack::<i32>::new();
    a.push(12);
    a.push(1200);
    a.push(12000);
    a.pop();
    println!("{}", a)
}

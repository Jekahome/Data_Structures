
/// cargo run --example stack
/// Стек - это линейная структура данных, которая следует принципу Last In First Out (LIFO) .
/// Это означает, что первым удаляется последний элемент, вставленный в стек.
/// Вы можете думать о структуре данных стека как о стопке тарелок.

/// Здесь вы можете:
/// - сверху ставим новую тарелку;
/// - снимите верхнюю пластину;
/// И, если вы хотите, чтобы тарелка была внизу, вы должны сначала удалить все тарелки сверху.

///Основные операции стека
///Есть несколько основных операций, которые позволяют нам выполнять различные действия со стеком.
///
/// Push : добавить элемент в верхнюю часть стека.
/// Pop : удалить элемент из вершины стека
/// IsEmpty : проверьте, пуст ли стек
/// IsFull : проверьте, заполнен ли стек
/// Peek : получить значение верхнего элемента, не удаляя его.

#[derive(Debug)]
struct Stack<T>{
    data:Vec<T>
}

impl <T>Stack<T>{
    fn new()->Self{
        Self{data:Vec::<T>::new()}
    }
    fn push(&mut self,item:T){
        self.data.push(item);
    }
    fn pop(&mut self)->Option<T>{
        if !self.data.is_empty(){
            return Some(self.data.remove(0));
        }
        return None;
    }
    fn peek(&self)->Option<&T>{
        if !self.data.is_empty(){
            return Some(self.data.first().unwrap());
        }
        return None;
    }
    fn isEmpty(&self)->bool{
        self.data.is_empty()
    }
}

fn main() {
    let mut stack = Stack::<i32>::new();
    stack.push(8);
    stack.push(6);
    stack.push(3);
    let first = stack.pop();
    println!("first:{:?} \nstack:{:?}",first,stack);

    let d:[i32;4] = [1,2,3,4];
    let a:&[i32] = &d[..];

}

/// $ cargo test  --examples
/// Or you can
/// $ cargo test  --example stack
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.pop();
        assert_eq!(Some(8),first);
    }

    #[test]
    fn test_empty(){
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.pop();
        let first = stack.pop();
        assert!(first.is_none());
        assert!(stack.isEmpty());
    }

    #[test]
    fn test_peek(){
        let mut stack = Stack::<i32>::new();
        stack.push(8);
        let first = stack.peek();

        assert_eq!(first,Some(&8));
    }
}

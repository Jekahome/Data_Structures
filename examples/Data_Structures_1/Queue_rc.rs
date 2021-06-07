
/// cargo run --example queue-rc
///
/// Очередь следует правилу «первым пришел - первым ушел» (FIFO): элемент, который идет первым,
/// является элементом, который выходит первым.
///
/// Основные операции с очередью:
/// - добавить элемент в конец очереди;
/// - удалить элемент из начала очереди;
/// - проверьте, пуста ли очередь;
/// - проверьте, заполнена ли очередь;
/// - получить значение передней части очереди, не удаляя ее;


/// https://www.kirillvasiltsov.com/writing/how-to-write-a-queue-in-rust/
/// Это необходимо для решения низкоуровневых задач, таких как планирование заданий ЦП,
/// а также для моделирования реальной очереди - например, запросов на техническую
/// поддерку, которые необходимо обрабатывать последовательно.

use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T>{
    data:T,
    next:Option<Rc<Node<T>>>
}

impl <T> Node<T> where T:Copy+Debug{
    fn new(data:T)->Self{
        Node{data,next:None}
    }
    fn set_next(&mut self,next:Option<Rc<Node<T>>>)->bool{
           if self.next.is_none(){
               self.next = next;
               return true;
           }else{
               if let Some(e) = &mut self.next{
                   (*Rc::<Node<T>>::get_mut(e).unwrap()).set_next(next);
                   return true;
               }
           }
        return false;
    }
    fn peek_all<'b>(&'b self,buf:&mut Vec<&'b T>){
        buf.push(&self.data);
        if self.next.is_some(){
            self.next.as_ref().unwrap().peek_all(buf);
        }
    }
}


#[derive(Debug)]
struct Queue<T>{
    head:Option<Rc<Node<T>>>,
    count:usize
}
impl <T>Queue<T> where T:Copy+Debug{
    fn new(head:Option<Rc<Node<T>>>)->Self{
        Self{head,count:1}
    }
    fn enqueue(&mut self,next:Option<Rc<Node<T>>>){
        if self.head.is_some(){
            if let Some(e) = Rc::<Node<T>>::get_mut(&mut self.head.as_mut().unwrap()){
                e.set_next(next);
                self.count+=1;
            }
        }
    }
    fn dequeue(&mut self)->Option<T>{
        if self.head.is_some(){
            let head = self.head.as_mut().unwrap();
            let data = head.data.clone();
            if let Some(e) = &head.next{
                self.head = Some(Rc::clone(&e));
                self.count-=1;
            }else{
                self.head = None;
                self.count=0;
            }
           return Some(data);
        }
        return None;
    }
    fn peek(&self) -> Option<&T> {
        if self.head.is_some(){
            return Some(&self.head.as_ref().unwrap().data);
        }
        return None;
    }
    fn peek_all<'b>(&'b self,buf:&mut Vec<&'b T>){
        if self.head.is_some(){
            self.head.as_ref().unwrap().peek_all(buf);
        }
    }
    fn length(&self) -> usize {
        self.count
    }
    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<T> Iterator for Queue<T> where T: Copy+Debug {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.dequeue()
    }
}

fn main() {
    let mut node1 = Node::<i32>::new(1);
    let mut node2 = Node::<i32>::new(2);
    let mut node3 = Node::<i32>::new(3);

    let mut q = Queue::new(Some(Rc::new(node1)));
    q.enqueue(Some(Rc::new(node2)));
    q.enqueue(Some(Rc::new(node3)));

    /*for (i, x) in q.into_iter().enumerate() {
        println!("iter: {}, {}", i, x);
    }*/

    let item1 = q.dequeue();
    assert_eq!(item1,Some(1));

    let mut node4 = Node::<i32>::new(4);
    q.enqueue(Some(Rc::new(node4)));

    let item2 = q.dequeue();
    assert_eq!(item2,Some(2));

    let item3 = q.dequeue();
    assert_eq!(item3,Some(3));

    let item4 = q.dequeue();
    assert_eq!(item4,Some(4));

    let item5 = q.dequeue();
    assert_eq!(item5,None);

    println!("{:?}",&q);
}

/// $ cargo test  --example queue-rc
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut node1 = Node::<i32>::new(1);
        let mut queue = Queue::new(Some(Rc::new(node1)));

        let mut node2 = Node::<i32>::new(2);
        queue.enqueue(Some(Rc::new(node2)));

        assert_eq!(queue.peek(),Some(&1));
        assert_eq!(queue.length(),2);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let mut node1 = Node::<i32>::new(1);
        let mut queue = Queue::new(Some(Rc::new(node1)));

        let mut node2 = Node::<i32>::new(2);
        queue.enqueue(Some(Rc::new(node2)));

        let item = queue.dequeue();
        assert_eq!(item,Some(1));
        assert_eq!(queue.peek(),Some(&2));
        assert_eq!(queue.length(),1);
        assert!(!queue.is_empty());

        let item = queue.dequeue();
        assert_eq!(item,Some(2));
        assert_eq!(queue.peek(),None);
        assert_eq!(queue.length(),0);
        assert!(queue.is_empty());

    }

    #[test]
    fn test_peek_all() {
        let mut node1 = Node::<i32>::new(1);
        let mut node2 = Node::<i32>::new(2);
        let mut node3 = Node::<i32>::new(3);

        let mut q = Queue::new(Some(Rc::new(node1)));
        q.enqueue(Some(Rc::new(node2)));
        q.enqueue(Some(Rc::new(node3)));

        let mut buf:Vec<&i32> = vec![];
        &q.peek_all(&mut buf);
        assert_eq!(buf,vec![&1,&2,&3]);
    }

}


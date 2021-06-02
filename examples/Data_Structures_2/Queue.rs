
/// cargo run --example queue
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

#[derive(Debug)]
struct Queue<T>{
    queue:Vec<T>
}
impl <T> Queue<T> where T:Copy{
    fn new()->Self{
        Queue{queue:Vec::new()}
    }
    fn enqueue(&mut self,item :T){
        self.queue.push(item);
    }
    fn dequeue(&mut self)->T{
        self.queue.remove(0)
    }
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
    fn length(&self) -> usize {
        self.queue.len()
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}


/// $ cargo test  --example queue
fn main() {
    let mut queue = Queue::<i32>::new();
    queue.enqueue(1);
    let item = queue.dequeue();
    assert_eq!(item, 1);
    assert_eq!(queue.is_empty(), true);
    //println!("{:?}",queue);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_peek() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        assert_eq!(*queue.peek().unwrap(), 1);
    }

    #[test]
    fn queue_dequeue() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        let item = queue.dequeue();
        assert_eq!(item, 1);
        assert_eq!(queue.is_empty(), true);
    }
}
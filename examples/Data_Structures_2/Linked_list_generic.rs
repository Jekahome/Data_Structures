

/// cargo run --example linked-list-generic
///
/// Сила связного списка заключается в возможности разорвать цепочку и снова присоединиться к ней.
///  Например, если вы хотите поместить элемент 4 между 1 и 2, шаги будут следующими:
///
/// Создайте новый структурный узел и выделите ему память.
/// - Добавьте его значение данных как 4
/// - Укажите его следующий указатель на узел структуры, содержащий 2 в качестве значения данных
/// - Измените следующий указатель «1» на только что созданный узел.
/// - Выполнение чего-то подобного в массиве потребовало бы сдвига позиций всех последующих элементов.
use std::fmt;
use std::fmt::Display;
use std::cmp::PartialEq;

#[derive(Debug,Clone)]
struct Node<T>{
    data:T,
    next:Box<Option<Node<T>>>
}


impl <T>Node<T> where T: Copy + PartialEq + Display{

    fn new(data: T, next: Option<Node<T>>)->Self{
        Self{data,next:Box::new(next)}
    }
    fn set_next(&mut self,node_next:Node<T>){
        if self.next.is_none(){
            self.next = Box::new(Some(node_next));
        }else{
            if let Some(node) = &mut (*self.next).as_mut(){
                node.set_next(node_next);
            }
        }
    }
    fn update_node(&mut self,search_data:T,node_next: Node<T>){
        if self.data == search_data{

            let mut node_clone = std::mem::replace(&mut self.next, Box::new(Some(node_next)));
            //let node_clone = *self.next.clone();
            //self.next = Box::new(Some(node_next));
            if let Some(node) = &mut (*self.next).as_mut(){
               if let Some(n) = *node_clone{
                   node.set_next(n);
               }
            }
        }
        else if self.next.is_some(){
            if let Some(node) = &mut (*self.next).as_mut(){
                node.update_node(search_data,node_next);
            }
        }
    }
    fn search(&self,data:T)->Option<&Node<T>>{
        if self.data == data{
            return Some(self);
        }else if self.next.is_some(){
            return (*self.next).as_ref().unwrap().search(data);
        }else{
            return None;
        }
    }
    fn recursion(&self){
        println!("{}",self.data);
        if let Some(next) = &*self.next{
            next.recursion();
        }
    }
}

#[derive(Debug,Clone)]
struct LinkedList<T>{
 head:   Box<Option<Node<T>>>
}
impl  <T>LinkedList<T> where T: Copy + PartialEq + Display{
    fn new()->Self{
        Self{head:Box::new(None)}
    }
    fn add(&mut self,node_next: Node<T>){
        if self.head.is_none(){
            self.head = Box::new(Some(node_next));
        }else{
            if let Some(node) =  (*self.head).as_mut(){
                node.set_next(node_next);
            }
        }
    }
    fn insert(&mut self,search_data:T,node_next: Node<T>){
        if self.head.is_some(){
            if let Some(node) = &mut (*self.head).as_mut(){
                node.update_node(search_data,node_next);
            }
        }
    }
    fn search(&self,data:T)->Option<&Node<T>>{
        if self.head.is_none(){
            return None;
        }
        if let Some(node) = &(*self.head).as_ref(){
            return node.search(data);
        }
        return None;

    }
    fn show_data(&self){

        if let Some(head) = &*self.head{
            head.recursion();
        }
    }
}


fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    for i in 1..=4_i32{
        if i == 3 {continue;}
        let node = Node::new(i,None);
        list.add(node);
    }


    let node = Node::new(3,None);
    list.insert(2,node);
    //println!("Search 4: {:?}",list.search(4));

    list.show_data();
}

/// $ cargo test  --examples
/// Or you can
/// $ cargo test  --example linked-list-generic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() -> Result<(), String> {
        let mut list:LinkedList<i32>  = LinkedList::new();

        for i in 1..=4{
            let node = Node::new(i,None);
            list.add(node);
        }

        assert_eq!(4,list.search(4).unwrap().data);
        Ok(())
    }

    #[test]
    fn test_insert(){
        let mut list:LinkedList<i32> = LinkedList::new();

        for i in 1..=4{
            if i == 3 {continue;}
            let node = Node::new(i,None);
            list.add(node);
        }

        let node = Node::new(3,None);
        list.insert(2,node);

        assert_eq!(3,list.search(3).unwrap().data);
    }

}
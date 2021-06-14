use std::rc::Rc;

/// cargo run --example binary-tree
/// Типы двоичного дерева:
/// 1. Полное двоичное дерево
/// 2. Идеальное двоичное дерево
/// 3. Полное двоичное дерево
/// 4. Дегенеративное или патологическое дерево
/// 5. Перекошенное двоичное дерево
/// 6. Сбалансированное двоичное дерево
///
///
/// Идеальное двоичное дерево
/// Совершенное двоичное дерево - это тип двоичного дерева, в котором каждый внутренний узел имеет
/// ровно два дочерних узла, а все конечные узлы находятся на одном уровне.
///
/// Сбалансированное двоичное дерево
/// Это тип двоичного дерева, в котором разница между высотой левого и правого поддерева для каждого
/// узла равна 0 или 1.
///
/// Применение:
/// Приложения двоичного дерева
/// Для легкого и быстрого доступа к данным
/// В алгоритмах маршрутизатора
/// Для реализации структуры данных кучи
/// Синтаксическое дерево

#[derive(Debug)]
struct Data{
    index:i32
}

#[derive(Debug)]
struct Node{
    data:Data,
    left:Option<Rc<Node>>,
    right:Option<Rc<Node>>
}
impl Node{
    fn new(data:Data,left:Option<Rc<Node>>,right:Option<Rc<Node>>)->Self{
        Self{data,left,right}
    }
    fn add(&mut self,node:Node){
        if node.data.index < self.data.index {
            if let Some(n) = &mut self.left{
                (*Rc::<Node>::get_mut(n).unwrap()).add(node);
            }else{
                self.left = Some(Rc::new(node));
            }
        }else{
            if let Some(n) = &mut self.right{
                (*Rc::<Node>::get_mut(n).unwrap()).add(node);
            }else{
                self.right = Some(Rc::new(node));
            }
        }
    }
}

fn main(){
    let mut root = Node::new(Data{index:2},None,None);
    {
        let new_node = Node::new(Data{index:4},None,None);
        root.add(new_node);

        let new_node = Node::new(Data{index:1},None,None);
        root.add(new_node);

        let new_node = Node::new(Data{index:3},None,None);
        root.add(new_node);
    }

    //println!("{:?}",root.right.unwrap().data.index);
      assert_eq!(root.right.as_ref().unwrap().data.index,4);
      assert_eq!(root.left.unwrap().data.index,1);
      assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().data.index,3);
}


/// $ cargo test  --example binary-tree
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut root = Node::new(Data{index:2},None,None);
        let new_node = Node::new(Data{index:4},None,None);
        root.add(new_node);

        let new_node = Node::new(Data{index:1},None,None);
        root.add(new_node);

        let new_node = Node::new(Data{index:3},None,None);
        root.add(new_node);
        assert_eq!(root.right.as_ref().unwrap().data.index,4);
        assert_eq!(root.left.unwrap().data.index,1);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().data.index,3);
    }
}

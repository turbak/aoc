use std::{ops::Index, process::id, collections::{HashSet, VecDeque}, rc::Rc, cell::RefCell, borrow::Borrow, fmt::{Display, Debug}};

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

struct ListNode<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>
}

struct CyclicDoubleLinkedList<T> {
    lenght: usize,
    head: Link<T>,
}

impl <T> CyclicDoubleLinkedList<T> {
    fn new(mut list: Vec<T>) -> Self {
        let mut linked_list = CyclicDoubleLinkedList { lenght: list.len(), head: Some(Rc::new(RefCell::new(ListNode {
            elem: list.pop().unwrap(),
            prev: None,
            next: None,
        }))) };

        let mut current = linked_list.head.clone().unwrap();
        for el in list {
            let next = Rc::new(RefCell::new(ListNode {
                elem: el,
                prev: Some(current.clone()),
                next: None,
            }));
            current.borrow_mut().next = Some(next.clone());
            current = next;
        }

        let head = linked_list.head.unwrap();

        head.borrow_mut().prev = Some(current.clone());
        current.borrow_mut().next = Some(head.clone());

        linked_list
    }

    fn insert_nth_back_after(&self, pos: usize, after: usize, link: Link<T>) {
        let mut current = self.head.as_ref();
        for _ in 0..after {
            current = current.unwrap().borrow_mut().next.as_ref();
        }
        
        for _ in 0..pos {
            current = current.unwrap().borrow_mut().prev.as_ref();
        }

        let prev = current.unwrap().borrow_mut().prev.as_ref();
        let next = current;

        let unwrapped_link = link.unwrap();
        unwrapped_link.borrow_mut().next = Some(next.unwrap().clone());
        unwrapped_link.borrow_mut().prev = Some(prev.unwrap().clone());

        prev.unwrap().borrow_mut().next = link;
        next.unwrap().borrow_mut().prev = link;
    }

    fn insert_nth_after(&self, pos: usize, after: usize, link: Link<T>) {
        let mut current = self.head;
        for _ in 0..after {
            let mut current = current.unwrap().get_mut().next;
        }
        
        for _ in 0..pos {
            current = current.unwrap().get_mut().next;
        }

        let prev = current.unwrap().get_mut().prev;
        let next = current;

        link.unwrap().get_mut().next = next;
        link.unwrap().get_mut().prev = prev;

        prev.unwrap().get_mut().next = link;
        next.unwrap().get_mut().prev = link;
    }

    fn set_head(&mut self, link: Link<T>) {
        self.head = link
    }

    fn remove<P>(&self, f: P) -> Link<T>
    where P: Fn(&T) -> bool {
        let mut current = self.head;
        while let Some(item) = current {
            if f(&item.get_mut().elem) {
                let prev = item.get_mut().prev;
                let next = item.get_mut().next;

                prev.unwrap().get_mut().next = next;
                next.unwrap().get_mut().prev = prev;

                return Some(item);
            }
        }
        None
    }
}

impl <T: std::cmp::PartialEq + Debug> Display for CyclicDoubleLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.unwrap().get_mut().next;
        write!(f, "[{:?}", self.head.unwrap().get_mut().elem);
        while current.unwrap().get_mut().elem != self.head.unwrap().get_mut().elem {
            write!(f, " ,{:?}", current.unwrap().get_mut().elem);
        }
        write!(f, "]")
    }
}

fn main() {
    #[cfg(debug_assertions)]
    let input = include_str!("../inputs/d20_test");

    #[cfg(not(debug_assertions))]
    let input = include_str!("../inputs/d20");

    let coords: Vec<(usize, isize)> = input
        .lines()
        .map(|l| l.parse::<isize>().expect("should have parsed int"))
        .enumerate()
        .collect();

    let mut coord_list = CyclicDoubleLinkedList::new(coords.clone());
    for (pos, num) in coords.iter() {
        let target_link = coord_list.remove(|item| item.0 == *pos).unwrap();
        let abs_num = num.abs() as usize;

        if *num > 0 {
            coord_list.insert_nth_after(abs_num, *pos, Some(target_link))
        } else if *num < 0 {
            coord_list.insert_nth_back_after(abs_num, *pos, Some(target_link))
        }
        println!("{}", coord_list);
    }

    // println!("zero pos: {}", zero_pos);
    // let mut res = 0;
    // for coord in groove_coords {
    //     let nth_num = coord_list.decrypted_list[(coord + zero_pos) % coord_list.list.len()];
    //     println!(
    //         "{}th number is {}",
    //         coord,
    //         nth_num
    //     );
    //     res += nth_num;
    // }

    //println!("groove coords sum: {}", res);
}

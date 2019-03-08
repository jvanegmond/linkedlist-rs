use std::string::String;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
{
    pub fn new() -> Self {
        List::<T> {
            head: Link::Empty
        }
    }

    pub fn add(&mut self, value: T) {
        let next = &mut &self.head;
        loop {
            match next {
                Link::Empty => {
                    let newnode = Node::<T> {
                            elem: value,
                            next: Link::Empty
                        };
                    *next = &mut Link::<T>::More(Box::new(newnode));
                    break;
                },
                Link::More(v) => {
                    println!("Maandag :( ");
                    let node = v.as_ref();
                    *next = &node.next;
                }
            }
        }
    }
}

fn main() {
    let mut list = List::<i32>::new();
    
    list.add(1);
    list.add(2);
    list.add(3);

    print(&list, ',');

    println!("Done");
}

pub fn print<T : ToString>(list : &List<T>, separator: char) {
    let mut print : String = String::new();
    let next = &mut &list.head;
    loop {
        match next {
            Link::Empty => {
                println!("{}", print);
                return;
            },
            Link::More(v) => {
                let node = v.as_ref();
                print.push_str(&node.elem.to_string());
                print.push(separator);
                *next = &node.next;
            }
        }
    }
}
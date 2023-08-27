use core::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        let mut iter = data.into_iter();
        let mut lst = Self::new();
        while let Some(elem) = iter.next_back() {
            lst = lst.prepend(elem);
        }
        lst
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Option<Self> {
        self.head.as_ref().map(|node| List {
            head: node.next.clone(),
        })
    }

    pub fn prepend(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // Data can only be moved out of the RC if there are no other owners
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            }
            // If there are other owners, leave the rest of the list alone
            else {
                break;
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

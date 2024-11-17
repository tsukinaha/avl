use crate::avl::AvlTree::{
    self,
    *,
};

pub struct AvlLNRIter<'a, T> {
    stack: Vec<&'a AvlTree<T>>,
    current: &'a AvlTree<T>,
}

impl<T> AvlTree<T>
where
    T: Ord,
{
    pub fn iter_lnr(&self) -> AvlLNRIter<T> {
        AvlLNRIter {
            stack: Vec::new(),
            current: self,
        }
    }
}

impl<'a, T> Iterator for AvlLNRIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current {
                Null => match self.stack.pop() {
                    Some(Tree(node)) => {
                        self.current = &node.right;
                        return Some(&node.val);
                    }
                    _ => return None,
                },
                Tree(node) => {
                    self.stack.push(self.current);
                    self.current = &node.left;
                }
            }
        }
    }
}

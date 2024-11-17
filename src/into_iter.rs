use crate::avl::AvlTree::{
    self,
    *,
};

pub struct AvlNRLIntoIter<T> {
    stack: Vec<AvlTree<T>>,
}

pub struct AvlRLNIntoIter<T> {
    stack: Vec<AvlTree<T>>,
}

pub struct AvlLNRIntoIter<T> {
    stack: Vec<AvlTree<T>>,
    current: AvlTree<T>,
}

impl<T> AvlTree<T>
where
    T: Ord,
{
    pub fn into_iter_nrl(self) -> AvlNRLIntoIter<T> {
        AvlNRLIntoIter { stack: vec![self] }
    }

    pub fn into_iter_rln(self) -> AvlRLNIntoIter<T> {
        AvlRLNIntoIter { stack: vec![self] }
    }

    pub fn into_iter_lnr(self) -> AvlLNRIntoIter<T> {
        AvlLNRIntoIter {
            stack: Vec::new(),
            current: self,
        }
    }
}

impl<T> Iterator for AvlNRLIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tree) = self.stack.pop() {
            match tree {
                Null => continue,
                Tree(node) => {
                    match node.left {
                        Null => (),
                        Tree(node) => self.stack.push(Tree(node)),
                    }
                    match node.right {
                        Null => (),
                        Tree(node) => self.stack.push(Tree(node)),
                    }
                    return Some(node.val);
                }
            }
        }
        None
    }
}

impl<T> Iterator for AvlRLNIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tree) = self.stack.pop() {
            match tree {
                Null => continue,
                Tree(node) => {
                    match node.right {
                        Null => (),
                        Tree(node) => self.stack.push(Tree(node)),
                    }
                    match node.left {
                        Null => (),
                        Tree(node) => self.stack.push(Tree(node)),
                    }
                    return Some(node.val);
                }
            }
        }
        None
    }
}

impl<T> Iterator for AvlLNRIntoIter<T>
where
    T: Ord,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current.take() {
                Null => match self.stack.pop() {
                    Some(Tree(mut node)) => {
                        self.current = node.right.take();
                        return Some(node.val);
                    }
                    _ => return None,
                },
                Tree(mut node) => {
                    self.current = node.left.take();
                    self.stack.push(Tree(node));
                }
            }
        }
    }
}

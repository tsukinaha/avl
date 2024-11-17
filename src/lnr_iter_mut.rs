use crate::avl::AvlTree;

pub struct AvlLNRIterMut<'a, T> {
    current: *mut AvlTree<T>,
    stack: Vec<*mut AvlTree<T>>,
    _phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for AvlLNRIterMut<'a, T>
where
    T: Ord,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: self.current is a valid pointer to a node in the tree
        while let AvlTree::Tree(node) = unsafe { &mut *self.current } {
            self.stack.push(self.current);
            self.current = &mut node.left;
        }

        match self.stack.pop() {
            Some(node_ptr) => {
                // SAFETY: node_ptr is a valid pointer to a node in the tree
                let node = unsafe { &mut *node_ptr };
                if let AvlTree::Tree(node) = node {
                    self.current = &mut node.right;
                    Some(&mut node.val)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<T> AvlTree<T>
where
    T: Ord,
{
    pub fn iter_lnr_mut(&mut self) -> AvlLNRIterMut<T> {
        AvlLNRIterMut {
            current: self,
            stack: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

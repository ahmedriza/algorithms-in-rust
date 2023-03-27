//! Binary Search Tree
use std::{cmp::Ordering, fmt::Debug};

use super::{item::Item, symboltable::SymbolTable};

type Link<I> = Option<Box<Node<I>>>;

#[derive(Debug)]
struct Node<I: Item> {
    item: I,
    left: Link<I>,
    right: Link<I>,
}

impl<I> PartialEq for Node<I>
where
    I: Item + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.left == other.left && self.right == other.right
    }
}

impl<I: Item> Node<I> {
    pub fn new(item: I) -> Self {
        Self {
            item,
            left: None,
            right: None,
        }
    }
}

/// A symbol table implementation using binary search trees.
#[derive(Default)]
pub struct BinarySearchTree<I: Item> {
    head: Link<I>,
    count: usize,
}

impl<I> BinarySearchTree<I>
where
    I: Item + Default + Clone + PartialEq + Debug,
{
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    // Recursive implementation of insert
    fn insert_r(link: &mut Link<I>, item: I) {
        match link {
            Some(node) => {
                if item.key() < node.item.key() {
                    BinarySearchTree::insert_r(&mut node.left, item);
                } else {
                    BinarySearchTree::insert_r(&mut node.right, item);
                }
            }
            None => {
                link.replace(Box::new(Node::new(item)));
            }
        }
    }

    // Recursive implementation of search
    fn search_r(link: &Link<I>, key: I::Key) -> Option<&I> {
        match link {
            Some(node) => match key.cmp(&node.item.key()) {
                Ordering::Less => BinarySearchTree::search_r(&node.left, key),
                Ordering::Equal => Some(&node.item),
                Ordering::Greater => BinarySearchTree::search_r(&node.right, key),
            },
            None => None,
        }
    }

    fn show_r<'a>(
        link: &'a Link<I>,
        acc: &mut Vec<&'a dyn Item<Key = I::Key>>,
    ) -> Vec<&'a dyn Item<Key = I::Key>> {
        match link {
            Some(node) => {
                BinarySearchTree::show_r(&node.left, acc);
                acc.push(node.item.show());
                BinarySearchTree::show_r(&node.right, acc);
            }
            None => {}
        }
        acc.to_vec()
    }

    /// Right rotation. In a right rotation, the left child of the root becomes the new root.
    /// For example, given the following tree where the root is at S:
    ///
    /// ```text
    ///           S   
    ///          / \
    ///         E   X
    ///        / \    
    ///       C   R
    /// ```
    /// a right rotation will result in:
    /// ```text
    ///
    ///            E
    ///           / \
    ///          C   S
    ///             / \
    ///            R   X
    ///
    /// ```
    fn rotate_right(root: Link<I>) -> Link<I> {
        if let Some(mut s) = root {
            let e = s.left;
            if let Some(mut e_node) = e {
                s.left = e_node.right;
                e_node.right = Some(s);
                return Some(e_node);
            }
        }
        None
    }

    /// Left rotation. In a left rotation, the right child of the root becomes the new root.
    /// For example, given the following tree where the root is at A:
    ///
    /// ```text
    ///            A   
    ///           / \
    ///              E
    ///             / \
    ///            C   S
    ///                 
    /// ````
    /// a left rotation will result in:
    /// ```text
    ///               E
    ///              / \
    ///             A   S
    ///            / \   
    ///               C
    /// ```
    fn rotate_left(root: Link<I>) -> Link<I> {
        if let Some(mut a) = root {
            let e = a.right;
            if let Some(mut e_node) = e {
                a.right = e_node.left;
                e_node.left = Some(a);
                return Some(e_node);
            }
        }
        None
    }

    fn insert_at_root(link: Link<I>, item: I) -> Link<I> {
        match link {
            Some(node) => {
                if item.key() < node.item.key() {
                    let l = BinarySearchTree::insert_at_root(node.left, item);
                    // BinarySearchTree::rotate_right(l)
                    todo!()
                } else {
                    let r = BinarySearchTree::insert_at_root(node.right, item);
                    // BinarySearchTree::rotate_left(r)
                    todo!()
                }
            }
            None => Some(Box::new(Node::new(item))),
        }
    }
}

impl<I> SymbolTable<I, I::Key> for BinarySearchTree<I>
where
    I: Item + Default + Clone + PartialEq + Debug,
{
    fn count(&self) -> usize {
        self.count
    }

    fn search(&self, key: I::Key) -> Option<&I> {
        BinarySearchTree::search_r(&self.head, key)
    }

    fn insert(&mut self, item: I) {
        BinarySearchTree::insert_r(&mut self.head, item);
        self.count += 1;
    }

    fn remove(&mut self, _item: I) {
        todo!()
    }

    fn select(&self, _k: usize) -> I {
        todo!()
    }

    fn show(&self) -> Vec<&dyn Item<Key = I::Key>> {
        let mut acc = vec![];
        BinarySearchTree::show_r(&self.head, &mut acc)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::{rc::Rc, cell::RefCell};

    use crate::symboltables::{
        binarysearchtree::Node,
        item::{DoubleItem, Item},
        symboltable::SymbolTable,
    };

    use super::BinarySearchTree;

    #[test]
    fn test_binary_search_tree() {
        let mut bst = BinarySearchTree::<DoubleItem>::default();

        let i1 = DoubleItem::with_key(10);
        let i2 = DoubleItem::with_key(9);
        let i3 = DoubleItem::with_key(15);
        let i4 = DoubleItem::with_key(8);

        bst.insert(i1);
        bst.insert(i2);
        bst.insert(i3);
        bst.insert(i4);

        assert_eq!(bst.count(), 4);

        let expected_result: Vec<&dyn Item<Key = usize>> = vec![&i4, &i2, &i1, &i3];
        let result = bst.show();
        assert_eq!(result, expected_result);

        assert_eq!(bst.search(15), Some(&DoubleItem::with_key(15)));
        assert_eq!(bst.search(9), Some(&DoubleItem::with_key(9)));

        // non-existent item
        assert_eq!(bst.search(150), None);
    }

    #[test]
    fn test_rotate_right() {
        let mut bst = BinarySearchTree::<DoubleItem>::default();

        let i_7 = DoubleItem::with_key(7);
        let i_8 = DoubleItem::with_key(8);
        let i_9 = DoubleItem::with_key(9);
        let i_11 = DoubleItem::with_key(11);
        let i_15 = DoubleItem::with_key(15);

        //         11
        //        / \
        //       8   15
        //      / \
        //     7   9

        bst.insert(i_11);
        bst.insert(i_15);
        bst.insert(i_8);
        bst.insert(i_9);
        bst.insert(i_7);

        let h = BinarySearchTree::rotate_right(bst.head);

        //        8
        //       / \
        //      7   11
        //         /  \
        //        9    15

        assert_eq!(h.as_ref().unwrap().item, i_8);

        let left_subtree = Box::new(Node::new(i_7));

        let mut right_subtree = Box::new(Node::new(i_11));
        right_subtree.left = Some(Box::new(Node::new(i_9)));
        right_subtree.right = Some(Box::new(Node::new(i_15)));

        assert_eq!(h.as_ref().unwrap().left, Some(left_subtree));
        assert_eq!(h.as_ref().unwrap().right, Some(right_subtree));
    }

    #[test]
    fn test_rotate_left() {
        let mut bst = BinarySearchTree::<DoubleItem>::default();

        let i_7 = DoubleItem::with_key(7);
        let i_8 = DoubleItem::with_key(8);
        let i_9 = DoubleItem::with_key(9);
        let i_11 = DoubleItem::with_key(11);

        //         7
        //        / \
        //           9
        //          / \
        //         8  11

        bst.insert(i_7);
        bst.insert(i_9);
        bst.insert(i_8);
        bst.insert(i_11);

        let h = BinarySearchTree::rotate_left(bst.head);

        //         9
        //        / \
        //       7   11
        //      / \
        //         8
        //

        assert_eq!(h.as_ref().unwrap().item, i_9);

        let right_subtree = Box::new(Node::new(i_11));

        let mut left_subtree = Box::new(Node::new(i_7));
        left_subtree.right = Some(Box::new(Node::new(i_8)));

        assert_eq!(h.as_ref().unwrap().right, Some(right_subtree));
        assert_eq!(h.as_ref().unwrap().left, Some(left_subtree));
    }

    #[test]
    fn test_insert_at_root() {
        let mut bst = BinarySearchTree::<DoubleItem>::default();

        let i_7 = DoubleItem::with_key(7);
        let i_8 = DoubleItem::with_key(8);
        let i_9 = DoubleItem::with_key(9);
        let i_11 = DoubleItem::with_key(11);

        //         7
        //        / \
        //           9
        //            \
        //            11

        bst.insert(i_7);
        bst.insert(i_9);
        bst.insert(i_11);

        println!("bst: {:#?}", bst.head);

        // let _t = BinarySearchTree::insert_at_root(bst.head, i_8);
    }

    #[test]
    fn test_tree() {

        type NodePtr<I> = Option<Rc<RefCell<Node<I>>>>;

        #[derive(Debug)]
        struct Node<I: Item> {
            item: I,
            left: NodePtr<I>,
            right: NodePtr<I>,
        }

        impl <I: Item> Node<I> {
            pub fn new(item: I) -> Self {
                Self {
                    item,
                    left: None,
                    right: None,
                }
            }
        }

        #[derive(Default)]
        struct Tree<I: Item> {
            head: NodePtr<I>
        }

        impl <I: Item> Tree<I> {
            pub fn insert_r(root: &mut NodePtr<I>, item: I) {
                match root {
                    Some(node) => {
                        if item.key() < node.borrow().item.key() {
                            Tree::insert_r(&mut node.borrow_mut().left, item)
                        } else {
                            Tree::insert_r(&mut node.borrow_mut().right, item)
                        }
                    },
                    None => {
                        root.replace(Rc::new(RefCell::new(Node::new(item))));
                    }
                }
            }

            pub fn insert(&mut self, item: I) {
                Tree::insert_r(&mut self.head, item);
            }
        }

        fn rotate_right<I: Item>(root: &mut NodePtr<I>) {
            let _t = do_rotate_right(root);
            *root = _t;
        }

        fn do_rotate_right<I: Item>(root: &mut NodePtr<I>) -> NodePtr<I> {
            if let Some(s_node) = root {
                let mut s = s_node.borrow_mut();
                let e = s.left.clone();
                if let Some(e_node) = e {
                    s.left = e_node.borrow_mut().right.take();
                    e_node.borrow_mut().right = Some(s_node.clone());
                    return Some(e_node)
                }
            }
            None
        }

        let i_7 = DoubleItem::with_key(7);
        let i_8 = DoubleItem::with_key(8);
        let i_9 = DoubleItem::with_key(9);
        let i_11 = DoubleItem::with_key(11);
        let i_15 = DoubleItem::with_key(15);

        //         11
        //        / \
        //       8   15
        //      / \
        //     7   9

        let mut bst = Tree::<DoubleItem>::default();

        bst.insert(i_11);
        bst.insert(i_15);
        bst.insert(i_8);
        bst.insert(i_9);
        bst.insert(i_7);

        println!("bst: {:#?}", bst.head);

        rotate_right(&mut bst.head);

        //        8
        //       / \
        //      7   11
        //         /  \
        //        9    15

        println!("bst: {:#?}", bst.head);
    }
}

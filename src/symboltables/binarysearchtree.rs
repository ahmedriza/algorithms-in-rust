//! Binary Search Tree
use std::{cell::RefCell, cmp::Ordering, fmt::Debug, rc::Rc};

use super::{item::Item, symboltable::SymbolTable};

type NodePtr<I> = Rc<RefCell<Node<I>>>;

type Link<I> = Option<NodePtr<I>>;

/// Nodes of the binary tree
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
    pub fn new(item: I) -> NodePtr<I> {
        let node = Self {
            item,
            left: None,
            right: None,
        };
        Rc::new(RefCell::new(node))
    }
}

/// A symbol table implementation using binary search trees.
#[derive(Default)]
pub struct BinarySearchTree<I: Item> {
    head: Link<I>,
    count: usize,
}

impl<I> SymbolTable<I, I::Key> for BinarySearchTree<I>
where
    I: Item + Default + Clone + PartialEq + Debug,
{
    fn count(&self) -> usize {
        self.count
    }

    fn search(&self, key: I::Key) -> Option<I> {
        BinarySearchTree::search_r(self.head.clone(), key)
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

    fn show(&self) -> Vec<I> {
        let mut acc = vec![];
        BinarySearchTree::show_r(self.head.clone(), &mut acc)
    }
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

    /// Insert the `item` at the root of the tree. This will do the necessary rotations to
    /// ensure that the `item` ends up at the root of the tree.
    pub fn insert_at_root(&mut self, item: I) {
        BinarySearchTree::insert_at_root_r(&mut self.head, item);
    }

    // recursively insert `item` so that it ends up at the root of the whole tree
    fn insert_at_root_r(root: &mut Link<I>, item: I) {
        match root {
            Some(node) => {
                if item.key() < node.borrow().item.key() {
                    BinarySearchTree::insert_at_root_r(&mut node.borrow_mut().left, item);
                    BinarySearchTree::rotate_right(root);
                } else {
                    BinarySearchTree::insert_at_root_r(&mut node.borrow_mut().right, item);
                    BinarySearchTree::rotate_left(root);
                }
            }
            None => {
                root.replace(Node::new(item));
            }
        }
    }

    // Recursive implementation of insert
    fn insert_r(root: &mut Link<I>, item: I) {
        match root {
            Some(node) => {
                if item.key() < node.borrow().item.key() {
                    BinarySearchTree::insert_r(&mut node.borrow_mut().left, item)
                } else {
                    BinarySearchTree::insert_r(&mut node.borrow_mut().right, item)
                }
            }
            None => {
                root.replace(Node::new(item));
            }
        }
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
    fn rotate_right(root: &mut Link<I>) {
        *root = BinarySearchTree::do_rotate_right(root);
    }

    fn do_rotate_right(root: &mut Link<I>) -> Link<I> {
        if let Some(s_node) = root {
            let mut s = s_node.borrow_mut();
            let e = s.left.clone();
            if let Some(e_node) = e {
                s.left = e_node.borrow_mut().right.take();
                e_node.borrow_mut().right = Some(s_node.clone());
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
    fn rotate_left(root: &mut Link<I>) {
        *root = BinarySearchTree::do_rotate_left(root);
    }

    fn do_rotate_left(root: &mut Link<I>) -> Link<I> {
        if let Some(a_node) = root {
            let mut a = a_node.borrow_mut();
            let e = a.right.clone();
            if let Some(e_node) = e {
                a.right = e_node.borrow_mut().left.take();
                e_node.borrow_mut().left = Some(a_node.clone());
                return Some(e_node);
            }
        }
        None
    }

    // Recursive implementation of search
    fn search_r(root: Link<I>, key: I::Key) -> Option<I> {
        match root {
            Some(node) => match key.cmp(&node.borrow().item.key()) {
                Ordering::Less => BinarySearchTree::search_r(node.borrow().left.clone(), key),
                Ordering::Equal => Some(node.borrow().item.clone()),
                Ordering::Greater => BinarySearchTree::search_r(node.borrow().right.clone(), key),
            },
            None => None,
        }
    }

    // traverse the tree in-order and collect the nodes
    fn show_r(root: Link<I>, acc: &mut Vec<I>) -> Vec<I> {
        if let Some(node) = root {
            BinarySearchTree::show_r(node.borrow().left.clone(), acc);
            acc.push(node.borrow().item.clone());
            BinarySearchTree::show_r(node.borrow().right.clone(), acc);
        }
        acc.to_vec()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::symboltables::{
        binarysearchtree::Node,
        item::{DoubleItem, GenericItem, Item},
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

        let expected_result = vec![i4, i2, i1, i3];
        let result = bst.show();
        assert_eq!(result, expected_result);

        assert_eq!(bst.search(15), Some(DoubleItem::with_key(15)));
        assert_eq!(bst.search(9), Some(DoubleItem::with_key(9)));

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

        BinarySearchTree::rotate_right(&mut bst.head);

        //        8
        //       / \
        //      7   11
        //         /  \
        //        9    15

        assert_eq!(bst.head.as_ref().unwrap().borrow().item, i_8);

        let left_subtree = Node::new(i_7);

        let right_subtree = Node::new(i_11);
        right_subtree.borrow_mut().left = Some(Node::new(i_9));
        right_subtree.borrow_mut().right = Some(Node::new(i_15));

        assert_eq!(bst.head.as_ref().unwrap().borrow().left, Some(left_subtree));
        assert_eq!(
            bst.head.as_ref().unwrap().borrow().right,
            Some(right_subtree)
        );
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

        BinarySearchTree::rotate_left(&mut bst.head);

        //         9
        //        / \
        //       7   11
        //      / \
        //         8
        //

        assert_eq!(bst.head.as_ref().unwrap().borrow().item, i_9);

        let right_subtree = Node::new(i_11);
        let left_subtree = Node::new(i_7);
        left_subtree.borrow_mut().right = Some(Node::new(i_8));

        assert_eq!(
            bst.head.as_ref().unwrap().borrow().right,
            Some(right_subtree)
        );
        assert_eq!(bst.head.as_ref().unwrap().borrow().left, Some(left_subtree));
    }

    #[test]
    fn test_insert_at_root_one() {
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

        bst.insert_at_root(i_8);

        //        8
        //       / \
        //      7   9
        //           \
        //           11

        let left_subtree = Node::new(i_7);
        let right_subtree = Node::new(i_9);
        right_subtree.borrow_mut().right = Some(Node::new(i_11));

        assert_eq!(bst.head.as_ref().unwrap().borrow().left, Some(left_subtree));
        assert_eq!(
            bst.head.as_ref().unwrap().borrow().right,
            Some(right_subtree)
        );
    }

    #[test]
    fn test_insert_at_root_two() {
        let mut bst = BinarySearchTree::<GenericItem<String, f64>>::default();

        let i_a = GenericItem::<String, f64>::new("A".to_string());
        let i_c = GenericItem::<String, f64>::new("C".to_string());
        let i_e = GenericItem::<String, f64>::new("E".to_string());
        let i_s = GenericItem::<String, f64>::new("S".to_string());
        let i_x = GenericItem::<String, f64>::new("X".to_string());
        let i_r = GenericItem::<String, f64>::new("R".to_string());
        let i_h = GenericItem::<String, f64>::new("H".to_string());
        let i_g = GenericItem::<String, f64>::new("G".to_string());

        //       A
        //      / \
        //         S
        //        / \
        //       E   X
        //      / \
        //     C   R
        //        / \
        //       H
        //
        bst.insert(i_a.clone());
        bst.insert(i_s.clone());
        bst.insert(i_x.clone());
        bst.insert(i_e.clone());
        bst.insert(i_c.clone());
        bst.insert(i_r.clone());
        bst.insert(i_h.clone());

        // insert 'G' at root
        bst.insert_at_root(i_g);

        //         G
        //       /   \
        //      A     S
        //     / \    /\
        //        E  R  X
        //       /  /
        //      C  H

        let node_a = Node::new(i_a);
        let node_e = Node::new(i_e);
        node_e.borrow_mut().left = Some(Node::new(i_c));
        node_a.borrow_mut().right = Some(node_e);
        let left_subtree = Some(node_a);

        let node_r = Node::new(i_r);
        node_r.borrow_mut().left = Some(Node::new(i_h));
        let node_s = Node::new(i_s);
        node_s.borrow_mut().right = Some(Node::new(i_x));
        node_s.borrow_mut().left = Some(node_r);
        let right_subtree = Some(node_s);

        assert_eq!(bst.head.as_ref().unwrap().borrow().left, left_subtree);
        assert_eq!(bst.head.as_ref().unwrap().borrow().right, right_subtree);
    }

    #[allow(unused)]
    fn test_tree() {
        type NodePtr<I> = Option<Rc<RefCell<Node<I>>>>;

        #[derive(Debug)]
        struct Node<I: Item> {
            item: I,
            left: NodePtr<I>,
            right: NodePtr<I>,
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

        #[derive(Default)]
        struct Tree<I: Item> {
            head: NodePtr<I>,
        }

        impl<I: Item> Tree<I> {
            pub fn insert(&mut self, item: I) {
                Tree::insert_r(&mut self.head, item);
            }

            pub fn insert_r(root: &mut NodePtr<I>, item: I) {
                match root {
                    Some(node) => {
                        if item.key() < node.borrow().item.key() {
                            Tree::insert_r(&mut node.borrow_mut().left, item)
                        } else {
                            Tree::insert_r(&mut node.borrow_mut().right, item)
                        }
                    }
                    None => {
                        root.replace(Rc::new(RefCell::new(Node::new(item))));
                    }
                }
            }

            pub fn insert_at_root(&mut self, item: I) {
                Tree::insert_at_root_r(&mut self.head, item);
            }

            pub fn insert_at_root_r(root: &mut NodePtr<I>, item: I) {
                match root {
                    Some(node) => {
                        if item.key() < node.borrow().item.key() {
                            Tree::insert_at_root_r(&mut node.borrow_mut().left, item);
                            Tree::rotate_right(root);
                        } else {
                            Tree::insert_at_root_r(&mut node.borrow_mut().right, item);
                            Tree::rotate_left(root);
                        }
                    }
                    None => {
                        root.replace(Rc::new(RefCell::new(Node::new(item))));
                    }
                }
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
            fn rotate_right(root: &mut NodePtr<I>) {
                let _t = Tree::do_rotate_right(root);
                *root = _t;
            }

            fn do_rotate_right(root: &mut NodePtr<I>) -> NodePtr<I> {
                if let Some(s_node) = root {
                    let mut s = s_node.borrow_mut();
                    let e = s.left.clone();
                    if let Some(e_node) = e {
                        s.left = e_node.borrow_mut().right.take();
                        e_node.borrow_mut().right = Some(s_node.clone());
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
            fn rotate_left(root: &mut NodePtr<I>) {
                let _t = Tree::do_rotate_left(root);
                *root = _t;
            }

            fn do_rotate_left(root: &mut NodePtr<I>) -> NodePtr<I> {
                if let Some(a_node) = root {
                    let mut a = a_node.borrow_mut();
                    let e = a.right.clone();
                    if let Some(e_node) = e {
                        a.right = e_node.borrow_mut().left.take();
                        e_node.borrow_mut().left = Some(a_node.clone());
                        return Some(e_node);
                    }
                }
                None
            }
        }
    }
}

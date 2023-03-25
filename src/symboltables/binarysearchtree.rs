//! Binary Search Tree
//!
//! A symbol table implementation using binary search trees.
use std::{cmp::Ordering, fmt::Debug};

use super::{item::Item, symboltable::SymbolTable};

type Link<I> = Option<Box<Node<I>>>;

#[derive(Debug)]
pub struct Node<I: Item> {
    item: I,
    left: Link<I>,
    right: Link<I>,
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
pub struct BinarySearchTree<I: Item> {
    head: Link<I>,
    count: usize,
}

impl<I> BinarySearchTree<I>
where
    I: Item + Default + Clone + Copy + PartialEq + Debug,
{
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    // Recursive implementation of insert
    pub fn insert_r(link: &mut Link<I>, item: I) {
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
    pub fn search_r(link: &Link<I>, key: I::Key) -> I {
        match link {
            Some(node) => match key.cmp(&node.item.key()) {
                Ordering::Less => BinarySearchTree::search_r(&node.left, key),
                Ordering::Equal => node.item,
                Ordering::Greater => BinarySearchTree::search_r(&node.right, key),
            },
            None => I::default(),
        }
    }

    pub fn show_r<'a>(
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
}

impl<I> SymbolTable<I, I::Key> for BinarySearchTree<I>
where
    I: Item + Default + Clone + Copy + PartialEq + Debug,
{
    fn count(&self) -> usize {
        todo!()
    }

    fn search(&self, key: I::Key) -> I {
        BinarySearchTree::search_r(&self.head, key)
    }

    fn insert(&mut self, item: I) {
        BinarySearchTree::insert_r(&mut self.head, item);
        self.count += 1;
    }

    //     9
    //    / \
    //   8   11
    //  / \
    // 7
    fn remove(&mut self, _item: I) {
        // find the item in the tree
        // we will also need to know the parent node of the node we are removing
        self.count -= 1;
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
    use crate::symboltables::{
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

        let expected_result: Vec<&dyn Item<Key = usize>> = vec![&i4, &i2, &i1, &i3];
        let result = bst.show();
        assert_eq!(result, expected_result);

        assert_eq!(bst.search(15), DoubleItem::with_key(15));
        assert_eq!(bst.search(9), DoubleItem::with_key(9));

        // non-existent item
        assert_eq!(bst.search(150), DoubleItem::default());
    }
}

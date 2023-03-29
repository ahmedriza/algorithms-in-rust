#![allow(unused)]

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

type NodePtr<K, V> = Rc<RefCell<Node<K, V>>>;
type Link<K, V> = Option<NodePtr<K, V>>;

struct Node<K, V> {
    key: K,
    value: V,
    n: usize, // nodes in subtree rooted here
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, n: usize) -> NodePtr<K, V> {
        let node = Self {
            key,
            value,
            n,
            left: None,
            right: None,
        };
        Rc::new(RefCell::new(node))
    }
}

pub struct BalancedTree<K, V> {
    root: Link<K, V>, // root of the tree
}

impl<K, V> BalancedTree<K, V>
where
    K: Ord,
    V: Clone,
{
    /// Return the smallest key greater than or equal to the given key
    pub fn ceiling(&self, key: K) -> K {
        todo!()
    }

    /// Returns whether there's a value paired with the given key in the table
    pub fn contains(&self, key: K) -> bool {
        todo!()
    }

    /// Delete the key (and value) from the table
    pub fn delete(&self, key: K) {
        todo!()
    }

    /// Delete the largest key (and value) from the table
    pub fn delete_man(&self) {
        todo!()
    }

    /// Delete the smallest key (and value) from the table
    pub fn delete_min(&self) {
        todo!()
    }

    /// Return the largest key less than or equal to the given key
    pub fn floor(&self, key: K) -> K {
        todo!()
    }

    /// Return the value that corresponds to the given key
    pub fn get(&self, key: K) -> Option<V> {
        BalancedTree::get_r(&self.root, key)
    }

    fn get_r(link: &Link<K, V>, key: K) -> Option<V> {
        match link {
            Some(node) => match key.cmp(&node.borrow().key) {
                Ordering::Less => BalancedTree::get_r(&node.borrow().left, key),
                Ordering::Equal => Some(node.borrow().value.clone()),
                Ordering::Greater => BalancedTree::get_r(&node.borrow().right, key),
            },
            None => None,
        }
    }

    /// Is the table empty or not?
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Return all keys in the table in sorted order
    pub fn keys(&self) -> Vec<K> {
        todo!()
    }

    /// Return keys in [lo..hi] in sorted order
    pub fn keys_in_range(&self, lo: K, hi: K) -> Vec<K> {
        todo!()
    }

    /// Return the largest key
    pub fn max(&self) -> K {
        todo!()
    }

    /// Return the smallest key
    pub fn min(&self) -> K {
        todo!()
    }

    /// Put the key, value pair into the table
    pub fn put(&self, key: K, value: V) {
        todo!()
    }

    /// Number of keys less than the given key
    pub fn rank(&self, key: K) -> usize {
        todo!()
    }

    /// Return the key of rank k
    pub fn select(&self, k: usize) -> K {
        todo!()
    }

    /// Return the number of keys in [lo..hi]
    pub fn size_in_range(&self, lo: K, hi: K) -> usize {
        todo!()
    }

    /// Return the number of key, value pairs in the table
    pub fn size(&self) -> usize {
        BalancedTree::_size(&self.root)
    }

    fn _size(link: &Link<K, V>) -> usize {
        match link {
            Some(node) => node.borrow().n,
            None => 0,
        }
    }
}

#![allow(unused)]

use std::{cell::RefCell, cmp::Ordering, fmt::Debug, rc::Rc};

type NodePtr<K, V> = Rc<RefCell<Node<K, V>>>;
type Link<K, V> = Option<NodePtr<K, V>>;

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct SymbolTableStatistics {
    average_put_cost: f64,
}

impl SymbolTableStatistics {
    pub fn new(compares_put: usize, total_puts: usize) -> Self {
        // The average cost of a put operation is:
        // 1 + the total number of comparisons done during puts divided by the total number of
        // put operations.
        //
        // The theoretical value is ~ 1.39 lg N
        // See: Algorithms, 4th edition, Robert Sedgewick, Kevin Wayne, Addition-Wesley, 2011
        //
        let average_put_cost = 1.0 + compares_put as f64 / total_puts as f64;
        Self { average_put_cost }
    }
}

#[derive(Default, Debug)]
pub struct BalancedTree<K, V> {
    root: Link<K, V>, // root of the tree
    // Number of compares for the put operation
    compares_put: usize,
}

impl<K, V> BalancedTree<K, V>
where
    K: Clone + Debug + Ord,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            root: None,
            compares_put: 0,
        }
    }

    /// Return the smallest key greater than or equal to the given key
    pub fn ceiling(&self, key: K) -> K {
        todo!()
    }

    /// Returns whether there's a value paired with the given key in the table
    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
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
        self.size() == 0
    }

    /// Return all keys in the table in sorted order
    pub fn keys(&self) -> Vec<K> {
        let mut result = vec![];
        BalancedTree::keys_r(&self.root, &mut result);
        result
    }

    fn keys_r(link: &Link<K, V>, acc: &mut Vec<K>) {
        match link {
            Some(node) => {
                BalancedTree::keys_r(&node.borrow().left, acc);
                acc.push(node.borrow().key.clone());
                BalancedTree::keys_r(&node.borrow().right, acc);
            }
            None => {}
        }
    }

    /// Return keys in [lo..hi] in sorted order
    pub fn keys_in_range(&self, lo: K, hi: K) -> Vec<K> {
        todo!()
    }

    /// Return the largest key.
    ///
    /// If the right link of the root is null, the largest key is the key at the root.
    /// If the right link is not null, the largest key is the largest key in the subtree rooted
    /// at the node referenced by the right link.
    pub fn max(&self) -> K {
        BalancedTree::max_r(&self.root)
    }

    fn max_r(link: &Link<K, V>) -> K {
        match link {
            Some(node) => match node.borrow().right {
                Some(_) => {
                    return BalancedTree::max_r(&node.borrow().right);
                }
                None => {
                    return node.borrow().key.clone();
                }
            },
            None => {
                panic!("Empty tree");
            }
        }
    }

    /// Return the smallest key
    ///
    /// If the left link of the root is null, the smallest key is the key at the root.
    /// If the left link is not null, the smallest key is the smallest key in the subtree rooted
    /// at the node referenced by the left link.     
    pub fn min(&self) -> K {
        BalancedTree::min_r(&self.root)
    }

    fn min_r(link: &Link<K, V>) -> K {
        match link {
            Some(node) => match node.borrow().left {
                Some(_) => {
                    return BalancedTree::min_r(&node.borrow().left);
                }
                None => {
                    return node.borrow().key.clone();
                }
            },
            None => {
                panic!("Empty tree");
            }
        }
    }

    /// Put the key, value pair into the table. Update the value if found, if not add the
    /// new key value pair.
    pub fn put(&mut self, key: K, value: V) {
        BalancedTree::put_r(&mut self.root, key, value, &mut self.compares_put);
    }

    fn put_r(link: &mut Link<K, V>, key: K, value: V, compares_put: &mut usize) {
        match link {
            Some(node) => {
                // store the ordering in a temporary to avoid overlapping borrows.
                let ordering = key.cmp(&node.borrow().key);
                *compares_put += 1;
                match ordering {
                    Ordering::Less => {
                        BalancedTree::put_r(&mut node.borrow_mut().left, key, value, compares_put);
                    }
                    Ordering::Equal => {
                        node.borrow_mut().value = value;
                    }
                    Ordering::Greater => {
                        BalancedTree::put_r(&mut node.borrow_mut().right, key, value, compares_put);
                    }
                }
                let left_size = BalancedTree::_size(&node.borrow().left);
                let right_size = BalancedTree::_size(&node.borrow().right);
                node.borrow_mut().n = left_size + right_size + 1;
            }
            None => {
                link.replace(Node::new(key, value, 1));
            }
        }
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

    /// Get the collected statistics
    pub fn statistics(&self, total_puts: usize) -> SymbolTableStatistics {
        SymbolTableStatistics::new(self.compares_put, total_puts)
    }

    fn _size(link: &Link<K, V>) -> usize {
        match link {
            Some(node) => node.borrow().n,
            None => 0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::BalancedTree;

    #[test]
    fn test_put() {
        let mut tree = make_tree();

        // update the value of node C
        tree.put("C".into(), 42);

        println!("{:#?}", tree);

        assert_eq!(tree.root.as_ref().unwrap().borrow().n, 7);
    }

    #[test]
    fn test_min_max() {
        let tree = make_tree();
        let min = tree.min();
        assert_eq!("A", min);

        let max = tree.max();
        assert_eq!("X", max);
    }

    fn make_tree() -> BalancedTree<String, u32> {
        let mut tree = BalancedTree::<String, u32>::new();

        // The numbers in brackets indicate the number of nodes in the subtree
        //
        //       A (7)
        //      / \
        //         S (6)
        //        / \
        //   (4) E   X (1)
        //      / \
        // (1) C   R (2)
        //        / \
        //   (1) H
        //

        tree.put("A".into(), 0);
        tree.put("S".into(), 0);
        tree.put("X".into(), 0);
        tree.put("E".into(), 0);
        tree.put("C".into(), 0);
        tree.put("R".into(), 0);
        tree.put("H".into(), 0);
        tree
    }
}

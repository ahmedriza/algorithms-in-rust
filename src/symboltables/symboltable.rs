use std::{fmt::Debug, rc::Rc};

use super::item::Item;

/// Symbol Table
///
/// The interface of a symbol table
pub trait SymbolTable<I: Item + PartialEq, K> {
    /// Return the item count
    fn count(&self) -> usize;

    /// Find an item with the given key
    fn search(&self, key: K) -> Option<&I>;

    /// Insert an item
    fn insert(&mut self, item: I);

    /// Remove an item
    fn remove(&mut self, item: I);

    /// Select k_th smallest item
    fn select(&self, k: usize) -> I;

    /// Display the items
    fn show(&self) -> Vec<&dyn Item<Key = K>>;
}

// -------------------------------------------------------------------------------------------------

/// Key indexed symbol table.
///
/// Key values are positive integers less than a sentinel value `m` and uses them as indices into
/// an array.
pub struct KeyIndexedSymbolTable<I: Item<Key = usize>> {
    items: Vec<I>,
    m: usize,
}

impl<I> KeyIndexedSymbolTable<I>
where
    I: Item<Key = usize> + Default + Clone + PartialEq,
{
    pub fn new(m: usize) -> Self {
        let items = vec![I::default(); m];
        Self { items, m }
    }
}

impl<I> SymbolTable<I, usize> for KeyIndexedSymbolTable<I>
where
    I: Item<Key = usize> + Default + Clone + PartialEq,
{
    fn count(&self) -> usize {
        let mut n = 0;
        for i in 0..self.m {
            if !self.items[i].null() {
                n += 1;
            }
        }
        n
    }

    fn search(&self, key: usize) -> Option<&I> {
        Some(&self.items[key])
    }

    fn insert(&mut self, item: I) {
        self.items[item.key()] = item.clone();
    }

    fn remove(&mut self, item: I) {
        self.items[item.key()] = I::default();
    }

    fn select(&self, k: usize) -> I {
        let mut k = k;
        for i in 0..self.m {
            if !self.items[i].null() {
                if k == 0 {
                    return self.items[i].clone();
                }
                k -= 1;
            }
        }
        I::default()
    }

    fn show(&self) -> Vec<&dyn Item<Key = usize>> {
        let mut result = vec![];
        for i in 0..self.m {
            if !self.items[i].null() {
                let item = self.items[i].show();
                result.push(item);
            }
        }
        result
    }
}

// -------------------------------------------------------------------------------------------------

/// Array Symbol Table
///
/// Array based symbol table where the items are kept in the order of the keys
pub struct ArraySymbolTable<I: Item> {
    items: Vec<I>,
    count: usize,
}

impl<I> ArraySymbolTable<I>
where
    I: Item + Default + Clone + Debug + PartialEq,
{
    pub fn new(m: usize) -> Self {
        let items = vec![I::default(); m];
        let count = 0;
        Self { items, count }
    }

    /// Find the index of the given item if it exists
    pub fn find_index(&self, item: I) -> Option<usize> {
        (0..self.count).find(|&i| self.items[i] == item)
    }
}

impl<I> SymbolTable<I, I::Key> for ArraySymbolTable<I>
where
    I: Item + Default + Clone + Debug + PartialEq,
{
    fn count(&self) -> usize {
        self.count
    }

    fn search(&self, key: I::Key) -> Option<&I> {
        let mut k = 0;
        for i in 0..self.count {
            if self.items[i].key() >= key {
                break;
            }
            k += 1;
        }
        if key == self.items[k].key() {
            return Some(&self.items[k]);
        }
        None
    }

    // Keep the array in order when inserting a new item by moving larger items to make room,
    // in the same manner as insertion sort.
    fn insert(&mut self, item: I) {
        let mut i = self.count;

        while i > 0 && item.key() < self.items[i - 1].key() {
            self.items[i] = self.items[i - 1].clone();
            i -= 1;
        }
        self.items[i] = item;
        self.count += 1;
    }

    fn remove(&mut self, item: I) {
        // find the index of the item in the array
        if let Some(i) = self.find_index(item) {
            // shift the elements from higher indices so the current element is overwritten
            let mut j = i;
            while j < self.count {
                self.items[j] = self.items[j + 1].clone();
                j += 1;
            }
            self.items[j - 1] = I::default();
        }
    }

    fn select(&self, k: usize) -> I {
        self.items[k].clone()
    }

    fn show(&self) -> Vec<&dyn Item<Key = I::Key>> {
        let mut result = vec![];
        let mut i = 0;
        while i < self.count {
            let item = self.items[i].show();
            if !item.null() {
                result.push(item);
            }
            i += 1;
        }
        result
    }
}

// -------------------------------------------------------------------------------------------------

type Link<I> = Option<Rc<Node<I>>>;

pub struct Node<I: Item> {
    item: I,
    next: Link<I>,
}

impl<I: Item> Node<I> {
    pub fn new(item: I, next: Link<I>) -> Self {
        Self { item, next }
    }
}

/// Lined Symbol Table
///
/// Linked list based (un-ordered) symbol table
#[derive(Default)]
pub struct LinkedSymbolTable<I: Item> {
    head: Link<I>,
    count: usize,
}

impl<I> LinkedSymbolTable<I>
where
    I: Item + Default + Clone + PartialEq,
{
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    // recursive implementation of search.
    pub fn search_r(link: &Link<I>, key: I::Key) -> Option<&I> {
        match link {
            Some(t) => {
                if t.item.key() == key {
                    Some(&t.item)
                } else {
                    LinkedSymbolTable::search_r(&t.next, key)
                }
            }
            None => None,
        }
    }
}

impl<I> SymbolTable<I, I::Key> for LinkedSymbolTable<I>
where
    I: Item + Default + Clone + PartialEq,
{
    fn count(&self) -> usize {
        self.count
    }

    fn search(&self, key: I::Key) -> Option<&I> {
        LinkedSymbolTable::search_r(&self.head, key)
    }

    fn insert(&mut self, item: I) {
        self.head = Some(Rc::new(Node::new(item, self.head.clone())));
    }

    fn remove(&mut self, _item: I) {
        todo!()
    }

    // Since the list is not in order this is not implemented
    fn select(&self, _k: usize) -> I {
        todo!()
    }

    // The list is not in order. `show` should return items in order for a correct implementation.
    fn show(&self) -> Vec<&dyn Item<Key = I::Key>> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::symboltables::item::{DoubleItem, Item};

    use super::{ArraySymbolTable, KeyIndexedSymbolTable, LinkedSymbolTable, SymbolTable};

    #[test]
    fn test_key_indexed_symbol_table() {
        let mut st = KeyIndexedSymbolTable::new(10);
        for i in 0..10 {
            let item = DoubleItem::with_key(i);
            st.insert(item);
        }

        // select 5th smallest item
        let _k = st.select(5);
        assert_eq!(_k, DoubleItem::with_key(5));
    }

    #[test]
    fn test_array_symbol_table() {
        let mut st = ArraySymbolTable::new(10);
        let i1 = DoubleItem::with_key(10);
        let i2 = DoubleItem::with_key(20);
        let i3 = DoubleItem::with_key(15);
        st.insert(i1);
        st.insert(i2);
        st.insert(i3);

        // an item that exists
        assert_eq!(st.search(15), Some(&DoubleItem::with_key(15)));
        // non-existent item
        assert_eq!(st.search(150), None);

        assert_eq!(st.select(1), DoubleItem::with_key(15));

        let expected: Vec<&dyn Item<Key = usize>> = vec![&i1, &i3, &i2];
        assert_eq!(st.show(), expected);

        // remove the item with key 15
        st.remove(i3);

        let expected: Vec<&dyn Item<Key = usize>> = vec![&i1, &i2];
        assert_eq!(st.show(), expected);
    }

    #[test]
    fn test_linked_symbol_table() {
        let mut st = LinkedSymbolTable::default();
        let i1 = DoubleItem::with_key(10);
        let i2 = DoubleItem::with_key(20);
        let i3 = DoubleItem::with_key(15);
        st.insert(i1);
        st.insert(i2);
        st.insert(i3);

        assert_eq!(st.search(15), Some(&DoubleItem::with_key(15)));

        // non-existent item
        assert_eq!(st.search(150), None);
    }
}

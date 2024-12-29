use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct BTreeNode {
    keys: Vec<Option<u32>>,
    children: Vec<Option<BTreeNode>>,
    is_leaf: bool,
}

#[derive(Debug)]
struct BTree {
    degree: usize,
    key_size: usize,
    children_size: usize,
    root: BTreeNode,
}

impl BTree {
    pub fn create_tree(degree: usize) -> Self {
        let root = BTreeNode {
            keys: vec![None; 2*degree - 1],
            children: vec![None; 2*degree],
            is_leaf: true,
        };
        let key_size = 2*degree - 1;
        let children_size = 2*degree;
        BTree { degree, root, key_size, children_size }
    }

    pub fn search(node: &BTreeNode, key: u32) -> Option<(&BTreeNode, usize)> {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i].unwrap() {
            i += 1;
        }
        if i < node.keys.len() && node.keys[i].is_some() && key == node.keys[i].unwrap() {
            return Some((node, i))
        }
        if node.is_leaf {
            return None
        }
        Self::search(node.children[i].as_ref().unwrap(), key)
    }

    pub fn insert(&mut self, key: u32) {
        let root_vector_size = Self::count_current_vector(&self.root);
        if root_vector_size == self.root.keys.capacity() {
            Self::split_root(self);
        }
        Self::insert_not_full(&mut self.root, key, self.degree)
    }

    pub fn remove(key: u32) {
        todo!()
    }

    fn merge_children() {
        todo!()
    }

    fn insert_not_full(node: &mut BTreeNode, key: u32, degree: usize) {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i].unwrap() {
            i += 1;
        }
        if node.is_leaf {
            // TODO refactor shift of elements
            for index in (i+1..node.keys.len()).rev() {
                node.keys[index] = node.keys[index-1].take();
            }
            node.keys[i] = Some(key);
        } else {
            let child_vector_size = Self::count_current_vector(node.children[i].as_ref().unwrap());
            if child_vector_size == node.children[i].as_ref().unwrap().keys.capacity() {
                Self::split_child(node, i, degree);
                if key > node.keys[i].unwrap() {
                    i += 1;
                }
            }
            Self::insert_not_full(node.children[i].as_mut().unwrap(), key, degree);
        }
    }

    fn split_root(&mut self) {
        let mut new_root = Self::new_node(self.degree, false);
        let old_root = std::mem::replace(&mut self.root, Self::new_node(self.degree, false));
        new_root.children[0] = Some(old_root);
        self.root = new_root;
        Self::split_child(&mut self.root, 0, self.degree);
    }

    fn split_child(parent: &mut BTreeNode, index: usize, degree: usize) {
        let mut full_node = parent.children[index].take().unwrap();
        let mut new_node = Self::new_node(degree, full_node.is_leaf);
        let new_node_keys: Vec<Option<u32>> = full_node.keys.iter_mut().skip(degree).map(Option::take).collect();
        for (i, key) in new_node_keys.into_iter().enumerate() {
            new_node.keys[i] = key;
        }
        if !full_node.is_leaf {
            let new_node_children: Vec<Option<BTreeNode>> = full_node.children.iter_mut().skip(degree).map(Option::take).collect();
            for (i, child) in new_node_children.into_iter().enumerate() {
                new_node.children[i] = child;
            }
        }

        // TODO refactor shift of keys
        for i in (index+2..2*degree).rev() {
            parent.children[i] = parent.children[i-1].take();
        }
        parent.children[index+1] = Some(new_node);

        // TODO refactor shift of children
        for i in (index+1..2*degree-1).rev() {
            parent.keys[i] = parent.keys[i-1].take();
        }
        parent.keys[index] = full_node.keys[degree-1].take();
        parent.children[index] = Some(full_node);
    }

    fn new_node(degree: usize, leaf: bool) -> BTreeNode {
        BTreeNode {
            keys: vec![None; 2*degree - 1],
            children: vec![None; 2*degree],
            is_leaf: leaf
        }
    }

    fn count_current_vector(node: &BTreeNode) -> usize {
        node.keys.iter().filter(|el| el.is_some()).count()
    }

    fn shift_elements() {
        todo!()
    }

    fn iter(&self) -> BTreeIter {
        let mut iter = BTreeIter { unvisited: VecDeque::new() };
        iter.initialize_stack(self);
        iter
    }
}

struct BTreeIter<'a> {
    unvisited: VecDeque<&'a BTreeNode>
}

impl<'a> BTreeIter<'a> {
    fn initialize_stack(&mut self, btree: &'a BTree) {
        self.unvisited.push_back(&btree.root);
    }
}

impl<'a> IntoIterator for &'a BTree {
    type Item = &'a Vec<Option<u32>>;
    type IntoIter = BTreeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for BTreeIter<'a> {
    type Item = &'a Vec<Option<u32>>;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop_front()?;
        let mut children_iter = node.children.iter();
        while let Some(child) = children_iter.next() {
            if child.is_some() {
                self.unvisited.push_back(child.as_ref().unwrap());
            }
        }
        Some(&node.keys)
    }
}


fn main() {
    let t = 2;
    let mut btree = BTree::create_tree(t);
    for i in 1..=10 {
        btree.insert(i);
    }
    
}

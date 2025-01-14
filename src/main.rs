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
        let root = Self::new_node(degree, true);
        let key_size = 2*degree - 1;
        let children_size = 2*degree;
        BTree { degree, root, key_size, children_size }
    }

    pub fn search(node: &BTreeNode, key: u32) -> Option<(&BTreeNode, usize)> {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i]? {
            i += 1;
        }
        if i < node.keys.len() && node.keys[i].is_some() && key == node.keys[i]? {
            return Some((node, i))
        }
        if node.is_leaf {
            return None
        }
        Self::search(node.children[i].as_ref()?, key)
    }

    pub fn insert(&mut self, key: u32) {
        let root_vector_size = Self::count_current_vector(&self.root);
        if root_vector_size == self.root.keys.capacity() {
            Self::split_root(self);
        }
        Self::insert_not_full(&mut self.root, key, self.degree)
    }

    pub fn remove(&mut self, key: u32) -> Option<u32> {
        let result = Self::remove_from_node(&mut self.root, key, self.degree);
        if Self::count_current_vector(&self.root) == 0 && !self.root.is_leaf {
            self.root = self.root.children[0].take().unwrap();
        }
        result
    }

    fn remove_from_node(node: &mut BTreeNode, key: u32, degree: usize) -> Option<u32> {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i]? {
            i += 1;
        }
        if node.is_leaf {
            return if i < node.keys.len() && node.keys[i].is_some() && key == node.keys[i]? {
                let result = node.keys[i].take();
                Self::shift_keys(&mut node.keys, i, 2*degree-2, false);
                result
            } else {
                None
            }
        }
        if i < node.keys.len() && node.keys[i].is_some() && key == node.keys[i]? {
            if Self::count_current_vector(node.children[i].as_ref()?) >= degree {
                let predecessor_key_index = node.children[i].as_ref().unwrap().keys
                    .iter()
                    .enumerate()
                    .filter(|(_, el)| el.is_some())
                    .max_by_key(|(_, &val)| val)
                    .map(|(idx, _)| idx)?;
                let predecessor_key = node.children[i].as_mut().unwrap().keys[predecessor_key_index].take()?;
                Self::shift_keys(&mut node.children[i].as_mut()?.keys, predecessor_key_index, 2*degree-2, false);
                let result = node.keys[i].take();
                node.keys[i] = Some(predecessor_key);
                result
            } else if Self::count_current_vector(node.children[i+1].as_ref()?) >= degree {
                let successor_key_index = node.children[i+1].as_ref().unwrap().keys
                    .iter()
                    .enumerate()
                    .filter(|(_, el)| el.is_some())
                    .min_by_key(|(_, &val)| val)
                    .map(|(idx, _)| idx)?;
                let successor_key = node.children[i+1].as_mut().unwrap().keys[successor_key_index].take()?;
                Self::shift_keys(&mut node.children[i+1].as_mut()?.keys, successor_key_index, 2*degree-2, false);
                let result = node.keys[i].take();
                node.keys[i] = Some(successor_key);
                result
            } else {
                Self::merge_children(node, i, degree);
                Self::remove_from_node(node.children[i].as_mut()?, key, degree)
            }
        } else {
            if Self::count_current_vector(node.children[i].as_ref()?) >= degree {
                Self::remove_from_node(node.children[i].as_mut()?, key, degree)
            } else if Self::siblings_with_t_keys(node, i, degree) {
                let j = Self::index_of_sibling_with_t_keys(node, i, degree)?;
                if j == i + 1 {
                    let first_none_node = node.children[i].as_ref()?.keys
                        .iter()
                        .position(|key| key.is_none())?;
                    node.children[i].as_mut()?.keys[first_none_node] = node.keys[i].take();
                    node.keys[i] = node.children[j].as_mut()?.keys[0].take();
                    Self::shift_keys(&mut node.children[j].as_mut()?.keys, 0, 2*degree-2, false);
                    if !node.children[j].as_ref()?.is_leaf {
                        let first_child = node.children[j].as_mut()?.children[0].take();
                        let first_none_child = node.children[i].as_ref()?.children
                            .iter()
                            .position(|child| child.is_none())?;
                        node.children[i].as_mut()?.children[first_none_child] = first_child;
                        Self::shift_children(&mut node.children[j].as_mut()?.children, 0, 2*degree-1, false);
                    }
                } else {
                    // for move_index in (1..2*degree-2).rev() {
                    //     node.children[i].as_mut()?.keys[move_index] = node.children[i].as_mut()?.keys[move_index-1].take();
                    // }
                    Self::shift_keys(&mut node.children[i].as_mut()?.keys, 1, 2*degree-2, true);
                    node.children[i].as_mut()?.keys[0] = node.keys[j].take();
                    let pop_key = node.children[j].as_ref()?.keys
                        .iter()
                        .position(|key| key.is_none())? - 1;
                    node.keys[j] = node.children[j].as_mut()?.keys[pop_key].take();
                    if !node.children[j].as_ref()?.is_leaf {
                        let pop_child = node.children[j].as_ref()?.children
                            .iter()
                            .position(|child| child.is_none())? - 1;
                        // for move_index in (1..2*degree-1).rev() {
                        //     node.children[i].as_mut()?.children[move_index] = node.children[i].as_mut()?.children[move_index-1].take();
                        // }
                        Self::shift_children(&mut node.children[i].as_mut()?.children, 1, 2*degree-1, true);
                        node.children[i].as_mut()?.children[0] = node.children[j].as_mut()?.children[pop_child].take();
                    }
                }
                Self::remove_from_node(node.children[i].as_mut()?, key, degree)
            } else {
                if i > 0 {
                    Self::merge_children(node, i - 1, degree);
                    i = i - 1;
                } else {
                    Self::merge_children(node, i, degree);
                }
                Self::remove_from_node(node.children[i].as_mut()?, key, degree)
            }
        }
    }

    fn merge_children(node: &mut BTreeNode, index: usize, degree: usize) {
        let median_key = node.keys[index].take();
        node.children[index].as_mut().unwrap().keys[degree-1] = median_key;
        Self::shift_keys(&mut node.keys, index, 2*degree-2, false);
        let sibling_keys: Vec<Option<u32>> = node.children[index+1].as_mut().unwrap().keys.iter_mut().take(degree-1).map(Option::take).collect();
        for (i, key) in sibling_keys.into_iter().enumerate() {
            node.children[index].as_mut().unwrap().keys[degree+i] = key;
        }
        if !node.children[index+1].as_ref().unwrap().is_leaf {
            // Same logic as for keys but for children
            let sibling_children: Vec<Option<BTreeNode>> = node.children[index+1].as_mut().unwrap().children.iter_mut().take(degree-1).map(Option::take).collect();
            for (i, key) in sibling_children.into_iter().enumerate() {
                node.children[index].as_mut().unwrap().children[degree+i+1] = key;
            }
        }
        // Shift children making sure the index+1 node gets deleted
        for i in index+1..node.children.capacity()-1 {
            node.children[i] = node.children[i+1].take();
        }
    }

    fn insert_not_full(node: &mut BTreeNode, key: u32, degree: usize) {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i].unwrap() {
            i += 1;
        }
        if node.is_leaf {
            Self::shift_keys(&mut node.keys, i+1, 2*degree-2, true);
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
        Self::shift_children(&mut parent.children.as_mut(), index+2, 2*degree-1, true);
        parent.children[index+1] = Some(new_node);
        Self::shift_keys(&mut parent.keys, index+1, 2*degree-2, true);
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

    fn siblings_with_t_keys(node: &BTreeNode, index: usize, t: usize) -> bool {
        let left_sibling = index > 0 &&
            Self::count_current_vector(node.children[index-1].as_ref().unwrap()) >= t;
        let right_sibling = index < node.children.capacity() - 1 &&
            Self::count_current_vector(node.children[index+1].as_ref().unwrap()) >= t;
        left_sibling || right_sibling
    }

    fn index_of_sibling_with_t_keys(node: &BTreeNode, index: usize, t: usize) -> Option<usize> {
        if index > 0 &&
            Self::count_current_vector(node.children[index-1].as_ref().unwrap()) >= t {
            return Some(index - 1)
        }
        if index < node.children.capacity() - 1 &&
            Self::count_current_vector(node.children[index+1].as_ref().unwrap()) >= t {
            return Some(index + 1)
        }
        None
    }

    // These functions are until more generic functionality is added
    fn shift_keys(vector: &mut [Option<u32>], start: usize, end: usize, reversed: bool) {
        if reversed {
            // shift to the right
            vector.copy_within((start-1)..end, start);
            vector[start - 1] = None;
        } else {
            // shift to the left
            vector.copy_within((start+1)..=end, start);
            vector[end] = None;
        }
    }

    fn shift_children(vector: &mut [Option<BTreeNode>], start: usize, end: usize, reversed: bool) {
        if reversed {
            // shift to the right
            for i in (start..end).rev() {
                vector[i] = vector[i-1].take();
            }
        } else {
            // shift to the left
            for i in start..end {
                vector[i] = vector[i+1].take();
            }
        }
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
    let mut btree = BTree::create_tree(2);

    for i in 0..=6 {
        btree.insert(i);
    }

    println!("Original tree");
    for i in &btree {
        println!("{:?}", i);
    }

    let mut deleted = btree.remove(1);
    println!("Delete key is {:?}", deleted);

    println!("Tree after removing {:?}", deleted);
    for i in &btree {
        println!("{:?}", i);
    }

    deleted = btree.remove(0);
    println!("Delete key is {:?}", deleted);

    println!("Tree after removing {:?}", deleted);
    for i in &btree {
        println!("{:?}", i);
    }

    btree.insert(7);
    println!("Tree after inserting {:?}", 7);
    for i in &btree {
        println!("{:?}", i);
    }
}

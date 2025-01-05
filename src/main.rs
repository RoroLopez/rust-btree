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

    pub fn remove(&mut self, node: &mut BTreeNode, key: u32) -> Option<u32> {
        let mut i = 0;
        while i < node.keys.len() && node.keys[i].is_some() && key > node.keys[i]? {
            i += 1;
        }
        if node.is_leaf {
            return if i < node.keys.len() && key == node.keys[i]? {
                node.keys[i]
            } else {
                None
            }
        }
        if i < node.keys.len() && key == node.keys[i]? {
            if Self::count_current_vector(node.children[i].as_ref()?) >= self.degree {
                let predecessor_key_index = node.children[i].as_ref().unwrap().keys
                    .iter()
                    .enumerate()
                    .filter(|(id, el)| el.is_some())
                    .max_by_key(|(_idx, &val)| val)
                    .map(|(idx, value)| idx)?;
                let predecessor_key = node.children[i].as_mut().unwrap().keys[predecessor_key_index].take()?;
                for i in predecessor_key_index..self.key_size {
                    node.children[i].as_mut().unwrap().keys[i] = node.children[i].as_mut().unwrap().keys[i+1];
                }
                let result = Self::remove(self, node.children[i].as_mut()?, predecessor_key);
                node.keys[i] = Some(predecessor_key);
                result
            }
            else if Self::count_current_vector(node.children[i+1].as_ref()?) >= self.degree {
                let successor_key_index = node.children[i+1].as_ref().unwrap().keys
                    .iter()
                    .enumerate()
                    .filter(|(id, el)| el.is_some())
                    .max_by_key(|(_idx, &val)| val)
                    .map(|(idx, value)| idx)?;
                let successor_key = node.children[i+1].as_mut().unwrap().keys[successor_key_index].take()?;
                for i in successor_key_index..self.key_size {
                    node.children[i+1].as_mut().unwrap().keys[i] = node.children[i+1].as_mut().unwrap().keys[i+1];
                }
                let result = Self::remove(self, node.children[i+1].as_mut()?, successor_key);
                node.keys[i] = Some(successor_key);
                result
            } else {
                Self::merge_children(node, i, self.degree);
                if Self::count_current_vector(&self.root) == 0 {
                    let child = self.root.children[0].take();
                    self.root = child?;
                }
                Self::remove(self, node.children[i].as_mut()?, key)
            }
        } else {
            if Self::count_current_vector(node.children[i].as_ref()?) >= self.degree {
                return Self::remove(self, node.children[i].as_mut()?, key)
            } else if Self::siblings_with_t_keys(node, i, self.degree) {
                let j = Self::index_of_sibling_with_t_keys(node, i, self.degree)?;
                if j == i + 1 {
                    let first_none = node.children[i].as_ref()?.keys
                        .iter()
                        .position(|val| val.is_none())?;
                    node.children[i].as_mut()?.keys[first_none] = node.keys[i].take();
                    node.keys[i] = node.children[j].as_mut()?.keys[0].take();
                    for i in first_none..node.keys.capacity() {
                        node.children[j].as_mut()?.keys[i] = node.children[j].as_mut()?.keys[i+1];
                    }
                    if !node.children[j].as_ref()?.is_leaf {
                        let first_child = node.children[j].take();
                        let first_none = node.children[i].as_ref()?.children
                            .iter()
                            .position(|child| child.is_none())?;
                        node.children[i].as_mut()?.children[first_none] = first_child;
                        for i in first_none..node.children.capacity() {
                            node.children[i].as_mut()?.children[i] = node.children[i].as_mut()?.children[i+1].take();
                        }
                    }
                }
            } else {

            }
            None
        }
    }

    fn merge_children(node: &mut BTreeNode, index: usize, degree: usize) {
        let median_key = node.keys[index].take();
        node.children[index].as_mut().unwrap().keys[degree-1] = median_key;
        // TODO refactor this shift
        // Shifting all elements so no gaps are in between
        for i in index..node.keys.capacity()-1 {
            node.keys[i] = node.keys[i+1];
        }
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
    println!("Testing");
}

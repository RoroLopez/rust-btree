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
    pub fn create_tree(&self, degree: usize) -> Self {
        let root = BTreeNode {
            keys: vec![None; 2*degree - 1],
            children: vec![None; 2*degree],
            is_leaf: true,
        };
        let key_size = 2*degree - 1;
        let children_size = 2*degree;
        BTree { degree, root, key_size, children_size }
    }

    pub fn search() {
        todo!()
    }

    pub fn insert(&mut self, key: u32) {
        let root_vector_size = self.root
            .keys
            .iter()
            .filter(|el| el.is_some())
            .count();
    }

    pub fn remove(key: u32) {
        todo!()
    }

    fn merge_node() {
        todo!()
    }

    fn insert_not_full(&self, node: &mut BTreeNode, key: u32, degree: usize) {
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
            // TODO refactor count of keys
            let child_vector_size = node.children[i]
                .as_ref()
                .unwrap()
                .keys
                .iter()
                .filter(|el| el.is_some())
                .count();
            if child_vector_size == node.children[i].as_ref().unwrap().keys.capacity() {
                Self::split_child(node, i, degree);
                if key > node.keys[i].unwrap() {
                    i += 1;
                }
            }
            Self::insert_not_full(self, node.children[i].as_mut().unwrap(), key, degree);
        }
    }

    fn new_node(degree: usize, leaf: bool) -> BTreeNode {
        BTreeNode {
            keys: vec![None; 2*degree - 1],
            children: vec![None; 2*degree],
            is_leaf: leaf
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
}

fn main() {
    let t = 2;
    let mut root = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: false,
    };
    root.keys[0] = Some(4);

    let mut child1 = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: false,
    };

    child1.keys[0] = Some(6);

    let mut child2 = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: true,
    };

    child2.keys[0] = Some(5);

    let mut child3 = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: true,
    };

    child3.keys[0] = Some(7);
    child3.keys[1] = Some(8);
    child3.keys[2] = Some(9);

    child1.children[0] = Some(child2);
    child1.children[1] = Some(child3);
    root.children[1] = Some(child1);

    let mut tree = BTree {
        degree: 2,
        key_size: 2*2 - 1,
        children_size: 2*2,
        root,
    };

    let empty_node = BTreeNode {
        keys: vec![None; 2*2-1],
        children: vec![None; 2*2],
        is_leaf: true,
    };

    let mut old_root = std::mem::replace(&mut tree.root, empty_node);
    
    tree.insert_not_full(&mut old_root, 10, 2);
    println!("{:?}", old_root);
}

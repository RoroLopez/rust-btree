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

    pub fn search() {
        todo!()
    }

    pub fn insert(key: u32) {
        todo!()
    }

    pub fn remove(key: u32) {
        todo!()
    }

    // fn split_child(&mut self, parent: &mut BTreeNode, index: usize) --> original signature
    fn split_child(&mut self, index: usize) {
        let t = self.degree;
        let mut parent = &mut self.root;
        let mut full_node = parent.children[index].take().unwrap();
        let mut new_node: BTreeNode = BTreeNode {
            keys: vec![None; self.key_size],
            children: vec![None; self.children_size],
            is_leaf: full_node.is_leaf,
        };
        let new_node_keys: Vec<Option<u32>> = full_node.keys.iter_mut().skip(t).map(Option::take).collect();
        for (i, key) in new_node_keys.into_iter().enumerate() {
            new_node.keys[i] = key;
        }
        if !full_node.is_leaf {
            let new_node_children: Vec<Option<BTreeNode>> = full_node.children.iter_mut().skip(t).map(Option::take).collect();
            for (i, child) in new_node_children.into_iter().enumerate() {
                new_node.children[i] = child;
            }
        }
        // shift children to make space for new child
        for i in (index+2..self.children_size).rev() {
            parent.children[i] = parent.children[i-1].take();
        }
        parent.children[index+1] = Some(new_node);

        // shift keys one element to make space for new key
        for i in (index+1..self.key_size).rev() {
            parent.keys[i] = parent.keys[i-1].take();
        }
        parent.keys[index] = full_node.keys[t-1].take(); // median value of node
        parent.children[index] = Some(full_node);
    }
}

fn main() {
    let t = 3;
    let mut root_node = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: false,
    };
    root_node.keys[0] = Some(4);
    root_node.keys[1] = Some(10);

    let mut split_node = BTreeNode {
        keys: vec![None; 2*t - 1],
        children: vec![None; 2*t],
        is_leaf: true,
    };
    split_node.keys[0] = Some(5);
    split_node.keys[1] = Some(6);
    split_node.keys[2] = Some(7);
    split_node.keys[3] = Some(8);
    split_node.keys[4] = Some(9);

    root_node.children[1] = Some(split_node);

    let mut tree = BTree {
        degree: t,
        key_size: 2*t-1,
        children_size: 2*t,
        root: root_node,
    };

    tree.split_child( 1);
    println!("{:?}", tree);
}

// type Children<T> = Option<RefCell<Rc<Vec<BTreeNode<T>>>>>;
// type Keys<T> = Option<RefCell<Rc<Vec<T>>>>;

#[derive(Clone, Debug)]
struct BTreeNode {
    keys: Vec<Option<u32>>,
    children: Vec<Option<BTreeNode>>,
    is_leaf: bool,
}

struct BTree {
    degree: usize,
    root: BTreeNode,
}

impl BTree {
    pub fn create_tree(degree: usize) -> Self {
        let root = BTreeNode {
            keys: vec![None; 2*degree - 1],
            children: vec![None; 2*degree],
            is_leaf: true,
        };
        BTree { degree, root }
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

    fn split_child(&mut self, parent: &mut BTreeNode, index: usize) {
        let t = self.degree;
        let mut full_node = parent.children[index].take().unwrap();
        let mut new_node: BTreeNode = BTreeNode {
            keys: vec![None; 2*t - 1],
            children: vec![None; 2*t],
            is_leaf: false,
        };
        new_node.is_leaf = full_node.is_leaf;
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
        parent.children[index+1] = Some(new_node);
        parent.keys[index] = full_node.keys[t-1]; // median value of node
        parent.children[index] = Some(full_node);
    }
}

// #[derive(Clone, Debug)]
// struct Node {
//     key: usize,
//     children: Vec<Option<Node>>
// }

fn main() {
    // let mut node = Node {
    //     key: 32,
    //     children: vec![None; 5]
    // };
    // for i in 0..5 {
    //     node.children[i] = Some(
    //         Node {
    //             key: i,
    //             children: vec![None; 5]
    //         }
    //     )
    // }
    // let mut new_node = Node {
    //     key: 100,
    //     children: vec![None; 5]
    // };
    // let node_children: Vec<Option<Node>> = node.children.iter_mut().skip(2).map(Option::take).collect();
    // for (i, child) in node_children.into_iter().enumerate() {
    //     new_node.children[i] = child;
    // }
    // for child in &new_node.children {
    //     println!("{:?}", child)
    // }

    // for i in node_children {
    //     println!("{:?}", i);
    // }


    // let n = 5;
    // let mut v: Vec<Option<u32>> = vec![None; n];
    // for (index, i) in (101 .. 106).enumerate() {
    //     v[index] = Some(i);
    // }
    // for i in &v {
    //     println!("{:?}", i);
    // }
    // let mut prior_median_slice: Vec<Option<u32>> = v.iter_mut().take(2).map(Option::take).collect();
    // let median = v[2].take();
    // let mut after_median_slice: Vec<Option<u32>> = v.iter_mut().skip(3).map(Option::take).collect();
    // println!("Prior median slice values");
    // for i in prior_median_slice {
    //     println!("{:?}", i);
    // }
    // println!("Median value");
    // println!("{:?}", median);
    // println!("After median slice values");
    // for i in after_median_slice {
    //     println!("{:?}", i);
    // }
    // println!("Original vector slice values after extraction");
    // for i in v {
    //     println!("{:?}", i);
    // }
}

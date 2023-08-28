use std::collections::{HashMap, HashSet, LinkedList};

fn hashmaps() {
    let map_one: HashMap<String, String> = HashMap::new();
    let map_two: HashMap<String, String> = HashMap::new();
    let map_three: HashMap<String, String> = HashMap::new();
    let mut map_set: HashSet<i32> = HashSet::new();

    for i in 1..10 {
        map_set.insert(i);
        map_set.insert(i - 1);
    }

    println!("MAP SET {map_set:?} {}", map_set.len());
    for i in map_set {
        println!("MAP SET I {i}");
    }

    for map in [map_one, map_two, map_three] {
        for i in map {
            println!("MAP I {:?} ", i);
        }
    }
}

fn vectors() {
    let vec_one = vec![1, 2, 2, 3, 1, 23, 3, 4];
    let vec_two: Vec<i32> = Vec::new();
    let vec_three: Vec<i32> = Vec::new();

    for vect in [vec_one, vec_two, vec_three] {
        for i in vect {
            println!("VEC I {} ", i)
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    num: i32,
}

impl Node {
    fn update(&mut self, num: i32) -> &Self {
        self.num = num;
        return self;
    }
}

fn linked_lists() {
    let _list = LinkedList::from([1, 2, 3, 3]);
    let mut list_nodes: LinkedList<Node> = LinkedList::new();

    let mut node = Node { num: 32 };
    list_nodes.push_back(node.update(12).clone());
    list_nodes.push_back(node.update(70).clone());

    for node in list_nodes {
        println!("NODE : {node:?} {}", node.num);
    }
}
pub fn simulate() {
    vectors();
    hashmaps();
    linked_lists()
}

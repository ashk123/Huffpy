/* Main Huffman Encoding system */

use std::collections::HashMap;

/* Main structure of huffman tree nodes */
#[derive(Debug, Default)]
pub struct Node {
    freq: i32,
    ch: Option<char>, // the 'char' field of non-leaf nodes of the tree also do not cotain any valid values
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(freq: i32, ch: Option<char>) -> Self {
        Self {
            freq,
            ch,
            left: None,
            right: None,
        }
    }
}

pub fn huffman_array_maker(res: HashMap<char, i32>) -> Vec<Box<Node>> {
    let mut p: Vec<Box<Node>> = res
        .iter()
        .map(|x| new_box(Node::new(*(x.1), Some(*(x.0)))))
        .collect();
    p
}

pub fn huffman_tree_maker(p: &mut Vec<Box<Node>>) {
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b: Box<Node> = p.pop().unwrap();

        let mut c = new_box(Node::new(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }
}

/* Make a new instance of Box with a Node inside */
pub fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

/* Found the count (frequency) of chars in a text */
pub fn frequency(s: &str) -> HashMap<char, i32> {
    let mut res: HashMap<char, i32> = HashMap::new();

    for i in s.chars() {
        // let mut current = res.get_mut(&i).unwrap().clone() + 1;
        // res.insert(i, current);
        let counter = res.entry(i).or_insert(0);
        *counter += 1;
    }

    res
}

pub fn assign_codes(p: &Box<Node>, h: &mut HashMap<char, String>, s: String) {
    if let Some(ch) = p.ch {
        // if there was a actual word
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            assign_codes(l, h, s.clone() + "0")
        }
        if let Some(ref r) = p.right {
            assign_codes(r, h, s.clone() + "1")
        }
    }
}

// fn encode(H: &HashMap<char, String>) -> String {
//     let mut r: String = String::from("");
//     let mut t: Option<&String>;

//     for ch in H.iter() {
//         t = H.get(ch.0);
//         r.push_str(t.unwrap());
//     }
//     r
// }
pub fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t: Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    r
}

pub fn decode_string(s: &str, root1: &Box<Node>) -> String {
    let mut retval = "".to_string();
    let mut nodeptr = root1;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root1;
        }
    }
    retval
}

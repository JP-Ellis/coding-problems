use crate::Problem;

pub struct P;

const STATEMENT: &str = r#"Daily Coding Problem 3

Given the root to a binary tree, implement serialize(root), which serializes the
tree into a string, and deserialize(s), which deserializes the string back into
the tree.

For example, given the following Node class

```
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

The following test should pass:

```
node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
```
"#;

/// Node which contains a value and an optional left and optional right value.
#[derive(Eq, PartialEq)]
struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new<S>(val: S, left: Option<Node>, right: Option<Node>) -> Node
    where
        S: Into<String>,
    {
        Node {
            val: val.into(),
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}

/// Serialize a node into a string.
fn serialize(n: &Node) -> String {
    let val = n.val.replace('\\', "\\\\").replace('"', "\\\"");
    match (&n.left, &n.right) {
        (Some(left), Some(right)) => {
            format!("\"{}\"[{},{}]", val, serialize(left), serialize(right))
        }
        (Some(left), None) => format!("\"{}\"[{},None]", val, serialize(left)),
        (None, Some(right)) => format!("\"{}\"[None,{}]", val, serialize(right)),
        (None, None) => format!("\"{}\"[None,None]", val),
    }
}

fn expect_next<I>(s: &mut I, c: char) -> Result<(), String>
where
    I: Iterator<Item = (usize, char)>,
{
    match s.next() {
        Some((_, sc)) if c == sc => Ok(()),
        Some((i, sc)) => Err(format!(
            "Error at position {}.  Expected {} but got {}.",
            i, c, sc
        )),
        None => Err(format!(
            "Error at end of string.  Expected {} but got nothing.",
            c
        )),
    }
}

/// Deserialize an iterator over characters into a Node.
///
/// If iterator is empty or is `"None"`, `Ok(None)` is returned.  In all other
/// cases, it is assumed there is a Node and it will be deserialized.  If an
/// error occurs during deserialization, it is given in the error message.
fn deserialize<I>(mut s: &mut I) -> Result<Option<Node>, String>
where
    I: Iterator<Item = (usize, char)>,
{
    match s.next() {
        None => Ok(None),
        Some((_, '"')) => {
            let mut val = String::new();
            let mut escaping = false;
            for (_, c) in &mut s {
                if escaping {
                    val.push(c);
                    escaping = false;
                } else if c == '\\' {
                    escaping = true;
                } else if c == '"' {
                    break;
                } else {
                    val.push(c);
                }
            }

            match s.next() {
                None => Ok(Some(Node::new(val, None, None))),
                Some((_, '[')) => {
                    let left = deserialize(s)?;
                    expect_next(s, ',')?;
                    let right = deserialize(s)?;
                    expect_next(s, ']')?;

                    Ok(Some(Node::new(val, left, right)))
                }
                Some((i, c)) => Err(format!("Error at position {}.  Unexpected {}", i, c)),
            }
        }
        Some((_, 'N')) => {
            expect_next(s, 'o')?;
            expect_next(s, 'n')?;
            expect_next(s, 'e')?;
            Ok(None)
        }
        Some((i, c)) => Err(format!("Error at position {}.  Unexpected {}", i, c)),
    }
}

impl Problem for P {
    fn statement(&self) {
        println!("{}", STATEMENT);
    }

    fn solve(&self) -> Result<(), String> {
        let leaf = Node::new("leaf", None, None);
        let left = Node::new("left-only", Some(Node::new("leaf", None, None)), None);
        let right = Node::new("right-only", None, Some(Node::new("left", None, None)));
        let root = Node::new(
            "root",
            Some(Node::new(
                "left",
                Some(Node::new("left.left", None, None)),
                None,
            )),
            Some(Node::new("right", None, None)),
        );
        let special = Node::new("\"'\\\"None[\"]\"'\\\"\\", None, None);

        for node in &[leaf, left, right, root, special] {
            let serialized = serialize(node);
            println!("Serialized node: {}", serialized);
            match deserialize(&mut serialized.chars().enumerate())? {
                Some(n) => {
                    if &n != node {
                        return Err("Deserialized node does not match original node.".to_string());
                    } else {
                        println!("-> Successful deserialization.")
                    }
                }
                None => return Err("Expected a node, but deserialized 'None'".to_string()),
            }
        }

        Ok(())
    }
}

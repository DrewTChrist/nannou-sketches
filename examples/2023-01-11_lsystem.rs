use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

//#[derive(Debug, Clone)]
#[derive(Clone)]
struct Node {
    pos: Vec2,
    c: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Node {{ pos: {}, c: {}, left: {}, right: {} }}",
            self.pos,
            self.c,
            if self.left.is_some() {
                "Some(...)"
            } else {
                "None"
            },
            if self.right.is_some() {
                "Some(...)"
            } else {
                "None"
            }
        )
    }
}

impl Node {
    fn new(pos: Vec2, c: char, left: Option<Node>, right: Option<Node>) -> Self {
        let mut leftnode = None;
        let mut rightnode = None;
        if let Some(n) = left {
            leftnode = Some(Box::new(n));
        }
        if let Some(n) = right {
            rightnode = Some(Box::new(n));
        }
        Self {
            pos,
            c,
            left: leftnode,
            right: rightnode,
        }
    }
    fn has_child(&self) -> bool {
        self.right.is_some() || self.left.is_some()
    }
}

struct D0LSystem {
    /// each string iteration is
    /// added to this vector
    iterations: Vec<String>,
    /// a tree structure built from
    /// the system iterations
    nodes: Node,
    /// tuples of "rules" describe
    /// what each character in the
    /// sequence is replaced with
    rules: Vec<(String, String)>,
    /// the current depth of the
    /// iteration
    index: usize,
    /// the maximum depth to build
    /// the system
    depth: u32,
}

impl D0LSystem {
    fn new(axiom: String, depth: u32) -> Self {
        let mut iterations = Vec::new();
        iterations.push(axiom.clone());
        Self {
            iterations,
            nodes: Node::new(pt2(0.0, 0.0), axiom.as_bytes()[0] as char, None, None),
            rules: Vec::new(),
            index: 0,
            depth,
        }
    }
    fn add_rule(&mut self, rule: (String, String)) {
        self.rules.push(rule);
    }
    fn iterate(&mut self) {
        if (self.index as u32) < self.depth {
            let mut new_level = String::new();
            for c in self.iterations[self.index].bytes().into_iter() {
                for rule in &self.rules {
                    if c as char == rule.0.as_bytes()[0] as char {
                        new_level.push_str(&rule.1);
                    }
                }
            }
            self.iterations.push(new_level);
            self.index += 1;
        }
    }
    fn build_nodes(i: usize, depth: usize, node: &mut Node) {
        if i < depth {
            println!("{i} {depth} {}", node.c);
            match node.c {
                'a' => {
                    node.left = Some(Box::new(Node::new(pt2(0.0, 0.0), 'a', None, None)));
                    if let Some(n) = &mut node.left {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                    node.right = Some(Box::new(Node::new(pt2(0.0, 0.0), 'b', None, None)));
                    if let Some(n) = &mut node.right {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                }
                'b' => {
                    node.left = Some(Box::new(Node::new(pt2(0.0, 0.0), 'a', None, None)));
                    if let Some(n) = &mut node.left {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                }
                _ => {}
            }
        }
    }
    fn iter_nodes(node: &Node, node_list: &mut Vec<Node>) {
        node_list.push(node.clone());
        if node.has_child() {
            if let Some(n) = &node.right {
                D0LSystem::iter_nodes(n, node_list);
            }
            if let Some(n) = &node.left {
                D0LSystem::iter_nodes(n, node_list);
            }
        }
    }
    fn draw(&self, draw: &Draw) {
        let mut y = 0.0;
        let mut x = 0.0;
        let mut x_row_start;
        for (level_idx, level) in self.iterations.iter().enumerate() {
            let level_bytes = level.as_bytes();
            for c in level_bytes.iter() {
                match *c as char {
                    'a' => {
                        draw.ellipse().x_y(x, y).radius(1.0).color(RED);
                    }
                    'b' => {
                        draw.ellipse().x_y(x, y).radius(1.0).color(BLUE);
                    }
                    _ => {}
                }
                x += 10.0;
            }
            y -= 5.0;
            x_row_start = -(level_idx as f32 * 5.0);
            x = x_row_start;
        }
    }
}

struct Model {
    lsys: D0LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut lsys = D0LSystem::new(String::from("b"), 4);
    lsys.add_rule((String::from("a"), String::from("ab")));
    lsys.add_rule((String::from("b"), String::from("a")));
    for _ in 0..lsys.depth {
        lsys.iterate();
    }
    let chars: Vec<char> = lsys
        .iterations
        .iter()
        .flat_map(|line| line.as_bytes())
        .map(|b| *b as char)
        .collect();
    println!("{chars:?} {}", chars.len());
    println!("depth: {}", lsys.iterations.len());
    D0LSystem::build_nodes(0, lsys.iterations.len(), &mut lsys.nodes);
    let mut nodes: Vec<Node> = Vec::new();
    D0LSystem::iter_nodes(&lsys.nodes, &mut nodes);
    println!("nodes: {}", nodes.len());
    for node in nodes {
        println!("{node:?}");
    }
    Model { lsys }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

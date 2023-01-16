use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

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
            },
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
    fn new_boxed(pos: Vec2, c: char, left: Option<Node>, right: Option<Node>) -> Option<Box<Self>> {
        Some(Box::new(Node::new(pos, c, left, right)))
    }
    fn has_child(&self) -> bool {
        self.right.is_some() || self.left.is_some()
    }
}

struct D0LSystem {
    /// a tree structure built from
    /// the system iterations
    nodes: Node,
    /// the current depth of the
    /// iteration
    index: usize,
    /// the maximum depth to build
    /// the system
    depth: usize,
}

impl D0LSystem {
    fn new(axiom: String, depth: usize) -> Self {
        Self {
            nodes: Node::new(pt2(0.0, 0.0), axiom.as_bytes()[0] as char, None, None),
            index: 0,
            depth,
        }
    }
    fn build_nodes(i: usize, depth: usize, node: &mut Node) {
        if i < depth {
            match node.c {
                'a' => {
                    node.left =
                        Node::new_boxed(pt2(node.pos.x - 10.0, node.pos.y - 25.0), 'a', None, None);
                    if let Some(n) = &mut node.left {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                    node.right =
                        Node::new_boxed(pt2(node.pos.x + 10.0, node.pos.y - 25.0), 'b', None, None);
                    if let Some(n) = &mut node.right {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                }
                'b' => {
                    node.left =
                        Node::new_boxed(pt2(node.pos.x + 15.0, node.pos.y - 25.0), 'a', None, None);
                    if let Some(n) = &mut node.left {
                        D0LSystem::build_nodes(i + 1, depth, n);
                    }
                }
                _ => {}
            }
        }
    }
    fn iter_nodes<'a>(node: &'a Node, node_list: &mut Vec<&'a Node>) {
        node_list.push(node);
        if node.has_child() {
            if let Some(n) = &node.right {
                D0LSystem::iter_nodes(n, node_list);
            }
            if let Some(n) = &node.left {
                D0LSystem::iter_nodes(n, node_list);
            }
        }
    }
    fn recurse(node: &Node, draw: &Draw) {
        draw.ellipse()
            .x_y(node.pos.x, node.pos.y)
            .radius(2.5)
            .color(WHITE);
        if node.has_child() {
            if let Some(n) = &node.right {
                draw.line()
                    .start(node.pos)
                    .end(n.pos)
                    .color(WHITE)
                    .weight(1.0);
                D0LSystem::recurse(n, draw);
            }
            if let Some(n) = &node.left {
                draw.line()
                    .start(node.pos)
                    .end(n.pos)
                    .color(WHITE)
                    .weight(1.0);
                D0LSystem::recurse(n, draw);
            }
        }
    }
    fn draw_nodes(&self, draw: &Draw) {
        D0LSystem::recurse(&self.nodes, draw);
    }
}

struct Model {
    lsys: D0LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut lsys = D0LSystem::new(String::from("b"), 10);
    D0LSystem::build_nodes(1, lsys.depth, &mut lsys.nodes);
    //let mut nodes: Vec<Node> = Vec::new();
    //D0LSystem::iter_nodes(&lsys.nodes, &mut nodes);
    Model { lsys }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw_nodes(&draw);
    draw.to_frame(app, &frame).unwrap();
}

use nannou::color::Alpha;
use nannou::ease;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

enum ConnectionType {
    _Empty,
    OneTwoSix,
}

// (layer index, node index)
type Connection = (usize, usize);

#[derive(Debug)]
struct Node {
    location: Vec2,
    color: Alpha<Rgb<f32>, f32>,
    connections: Vec<Connection>,
}

impl Node {
    fn new(x: f32, y: f32, color: Alpha<Rgb<f32>, f32>) -> Self {
        Self {
            location: pt2(x, y),
            color,
            connections: Vec::new(),
        }
    }
    fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }
    fn draw(&self, draw: &Draw) {
        let radius = pt2(0.0, 0.0).distance(self.location) / 10.0;
        draw.ellipse()
            .no_fill()
            .xy(self.location)
            .stroke(WHITE)
            .stroke_weight(2.0)
            //.radius(10.0)
            .radius(radius)
            .color(self.color);
    }
}

#[derive(Debug)]
struct Graph {
    layers: Vec<Vec<Node>>,
}

impl Graph {
    fn new() -> Self {
        Self { layers: Vec::new() }
    }
    fn draw_nodes(&self, draw: &Draw) {
        for layer in &self.layers {
            for node in layer {
                node.draw(draw);
            }
        }
    }
    fn draw_connections(&self, draw: &Draw) {
        for layer in &self.layers {
            for node in layer {
                for connection in &node.connections {
                    draw.line()
                        .start(node.location)
                        .end(self.layers[connection.0][connection.1].location)
                        .weight(2.0)
                        .color(WHITE);
                }
            }
        }
    }
    fn draw(&self, draw: &Draw) {
        self.draw_nodes(draw);
        self.draw_connections(draw);
    }
    fn add_layer(&mut self, layer: Vec<Node>) {
        self.layers.push(layer);
    }
}

fn create_layer(radius: f32, num_points: f32, start_angle: f32) -> Vec<Node> {
    let mut layer = Vec::new();
    let mut t = start_angle;
    //while t <= start_angle + 2.0 * PI {
    for _ in 0..num_points as usize {
        let x = t.cos() * radius;
        let y = t.sin() * radius;
        layer.push(Node::new(x, y, srgba(1.0, 1.0, 1.0, 1.0)));
        t += (start_angle + 2.0 * PI) / num_points;
    }
    layer
}

fn connect_126(layer: &mut Vec<Node>) {
    for i in 0..layer.len() {
        let two = if i + 2 < layer.len() {
            i + 2
        } else {
            i + 2 - layer.len()
        };
        let six = if i + 6 < layer.len() {
            i + 6
        } else {
            i + 6 - layer.len()
        };
        let left = if i > 0 { i - 1 } else { layer.len() - 1 };
        let right = if i + 1 < layer.len() { i + 1 } else { 0 };
        layer[i].add_connection((0, two));
        layer[i].add_connection((0, six));
        layer[i].add_connection((0, left));
        layer[i].add_connection((0, right));
    }
}

fn connect_nodes(layer: &mut Vec<Node>, connection: ConnectionType) {
    match connection {
        ConnectionType::OneTwoSix => connect_126(layer),
        ConnectionType::_Empty => {}
    }
}

fn ease_to_radius(model: &mut Model) {
    for layer in &mut model.graph.layers {
        for (idx, node) in layer.iter_mut().enumerate() {
            if node.location.x == model.ease_to_layer[idx].location.x
                && node.location.y == model.ease_to_layer[idx].location.y
            {
                model.reverse = true;
                model.ease_t = 0.0;
                //println!("model.reverse = true");
            }
            let change_x = model.ease_to_layer[idx].location.x - node.location.x;
            let change_y = model.ease_to_layer[idx].location.y - node.location.y;
            let x = ease::quad::ease_in(model.ease_t, node.location.x, change_x, 1.0);
            let y = ease::quad::ease_in(model.ease_t, node.location.y, change_y, 1.0);
            node.location.x = x;
            node.location.y = y;
        }
    }
}

fn ease_to_zero(model: &mut Model) {
    for layer in &mut model.graph.layers {
        for node in layer.iter_mut() {
            if node.location.distance(pt2(0.0, 0.0)) < 1.0 {
                model.reverse = false;
                model.ease_t = 0.0;
                //println!("model.reverse = false");
            }
            let change_x = 0.0 - node.location.x;
            let change_y = 0.0 - node.location.y;
            let x = ease::quad::ease_in(model.ease_t, node.location.x, change_x, 1.0);
            let y = ease::quad::ease_in(model.ease_t, node.location.y, change_y, 1.0);
            node.location.x = x;
            node.location.y = y;
        }
    }
}

struct Model {
    graph: Graph,
    ease_to_layer: Vec<Node>,
    ease_t: f32,
    reverse: bool,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut graph = Graph::new();
    let ease_to_layer = create_layer(250.0, 12.0, 0.0);
    //let mut layer = create_layer(250.0, 12.0, 0.0);
    let mut layer = Vec::new();
    for _ in 0..ease_to_layer.len() {
        layer.push(Node::new(0.0, 0.0, srgba(1.0, 1.0, 1.0, 1.0)));
        //layer.push(Node::new(
        //    random_range(-300.0, 300.0),
        //    random_range(-300.0, 300.0),
        //    srgba(1.0, 1.0, 1.0, 1.0),
        //));
    }
    connect_nodes(&mut layer, ConnectionType::OneTwoSix);
    graph.add_layer(layer);
    Model {
        graph,
        ease_to_layer,
        ease_t: 0.0,
        reverse: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.reverse {
        ease_to_radius(model);
    } else {
        ease_to_zero(model);
    }
    model.ease_t += 0.0125;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let bounds = app.window_rect();
    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .w_h(bounds.w(), bounds.h())
        .color(srgba(0.0, 0.0, 0.0, 0.75));
    model.graph.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

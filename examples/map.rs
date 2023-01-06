use nannou::prelude::*;
use simple_xml::from_string;

const QUERY: &str = "[bbox:45.5041100, -122.6456300, 45.5048300, -122.6446000];node;out skel;";

fn main() {
    nannou::app(model).update(update).run();
}

struct MapData {
    nodes: Vec<Vec2>,
    query: String,
    client: reqwest::blocking::Client,
}

impl MapData {
    fn new(query: &str) -> Self {
        Self {
            nodes: Vec::<Vec2>::new(),
            query: String::from(query),
            client: reqwest::blocking::Client::new(),
        }
    }
    fn make_request(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let q = String::from(self.query.as_str());
        let request = self
            .client
            .post("https://overpass-api.de/api/interpreter")
            .body(q);
        request.send()
    }
    fn get_nodes(&mut self) -> Result<(), String> {
        let response = match self.make_request() {
            Ok(res) => res,
            Err(_e) => {
                return Err(format!("get_nodes: {}", _e));
            }
        };

        let xml = match response.text() {
            Ok(x) => x,
            Err(_e) => {
                return Err(format!("get_nodes: {}", _e));
            }
        };

        let xml_parsed = match from_string(&xml) {
            Ok(x) => x,
            Err(_e) => {
                return Err(format!("get_nodes: {:?}", _e));
            }
        };

        let nodes = &xml_parsed.get_nodes("node");

        let nodes = match nodes {
            Some(n) => n,
            None => {
                return Err(format!("get_nodes: no nodes found"));
            }
        };

        for node in *nodes {
            let coord = pt2(
                node.attributes["lon"]
                    .replace('"', "")
                    .parse::<f32>()
                    .unwrap(),
                node.attributes["lat"]
                    .replace('"', "")
                    .parse::<f32>()
                    .unwrap(),
            );
            self.nodes.push(coord);
        }
        Ok(())
    }
}

struct Model {
    map_data: MapData,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut map_data: MapData;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        map_data = MapData::new(QUERY);
    } else {
        map_data = MapData::new(&args[1]);
    }
    if let Err(e) = map_data.get_nodes() {
        println!("{e}");
        std::process::exit(1);
    }
    Model { map_data }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    let boundary = app.window_rect();
    for node in &model.map_data.nodes {
        let x_scaled = map_range(
            node.x,
            -122.644,
            -122.646,
            boundary.right(),
            boundary.left(),
        );
        let y_scaled = map_range(node.y, 45.504, 45.505, boundary.bottom(), boundary.top());
        draw.ellipse()
            .color(BLACK)
            .w_h(5.0, 5.0)
            .x_y(x_scaled, y_scaled);
    }
    draw.to_frame(app, &frame).unwrap();
}

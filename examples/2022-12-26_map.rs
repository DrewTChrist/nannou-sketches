use nannou::prelude::*;
use simple_xml::from_string;

const QUERY: &str = "[bbox:45.5041100, -122.6456300, 45.5048300, -122.6446000];node;out skel;";

fn main() {
    nannou::app(model).update(update).run();
}

fn get_coords() -> Vec<Vec2> {
    let client = reqwest::blocking::Client::new();

    let request = client
        .post("https://overpass-api.de/api/interpreter")
        .body(QUERY);

    let response = match request.send() {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let xml = match response.text() {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    //println!("{:?}", xml);

    let xml_parsed = match from_string(&xml) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };

    //println!("{:?}", xml_parsed);

    let nodes = &xml_parsed.get_nodes("node");

    let nodes = match nodes {
        Some(n) => n,
        None => {
            println!("Error: Could not find nodes");
            std::process::exit(1);
        }
    };

    let mut coordinates: Vec<Vec2> = Vec::<Vec2>::new();

    //println!("{}", nodes[0].attributes["lat"]);
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
        coordinates.push(coord);
        //println!("Lat: {}, Lon: {}", node.attributes["lat"], node.attributes["lon"]);
    }
    coordinates
}

struct Model {
    nodes: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();

    Model {
        nodes: get_coords(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    let boundary = app.window_rect();
    for node in &model.nodes {
        let x_scaled = map_range(
            node.x,
            -122.644,
            -122.646,
            boundary.right(),
            boundary.left(),
        );
        let y_scaled = map_range(node.y, 45.504, 45.505, boundary.bottom(), boundary.top());
        //println!("{}, {}", x_scaled, y_scaled);
        draw.ellipse()
            .color(BLACK)
            .w_h(5.0, 5.0)
            .x_y(x_scaled * 1.0, y_scaled * 1.0);
    }
    draw.to_frame(app, &frame).unwrap();
}

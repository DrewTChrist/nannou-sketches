use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Axiom(char);

#[derive(Clone)]
struct Rule(String, String, fn(&Draw, &Vec2) -> ());

#[derive(Clone)]
struct LPosition {
}

#[derive(Clone)]
struct LSystem {
    alphabet: Vec<char>,
    axiom: Axiom,
    rules: Vec<Rule>,
    string: String,
    pos: Vec2,
    angle: f32,
    depth: usize,
}

impl LSystem {
    fn new(axiom: char, pos: Vec2, angle: f32, depth: usize) -> Self {
        Self {
            alphabet: vec![],
            axiom: Axiom(axiom),
            rules: vec![],
            string: String::new(),
            pos,
            angle,
            depth,
        }
    }
    fn add_alphabet(&mut self, alphabet: Vec<char>) -> Self {
        self.alphabet = alphabet;
        self.to_owned()
    }
    fn add_rule(&mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self.to_owned()
    }
    fn mutate(&mut self) {
        self.string.push(self.axiom.0);
        let mut tmp = String::new();
        for _ in 0..self.depth {
            let chars = self.string.as_bytes();
            for ch in chars {
                for rule in &self.rules {
                    if *ch as char == rule.0.as_bytes()[0] as char {
                        tmp.push_str(&rule.1);
                    }
                }
            }
            self.string.clear();
            self.string.push_str(&tmp);
            tmp.clear();
        }
    }
    fn update(&mut self) {}
    fn draw(&self, draw: &Draw) {
        for ch in self.string.as_bytes() {
            for rule in &self.rules {
                if *ch as char == rule.0.as_bytes()[0] as char {
                    rule.2(draw, &self.pos);
                }
            }
        }
    }
}

struct Model {
    lsys: LSystem,
}

fn model(app: &App) -> Model {
    let _window_id = app.new_window().size(600, 600).view(view).build().unwrap();
    let mut lsys = LSystem::new('a', pt2(0.0, 0.0), 90.0, 4)
        .add_alphabet(vec!['a', 'b'])
        .add_rule(Rule(String::from("a"), String::from("ab"), |draw, pos| {}))
        .add_rule(Rule(String::from("b"), String::from("a"), |draw, pos| {}));
    lsys.mutate();
    Model { lsys }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

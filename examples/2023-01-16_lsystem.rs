use nannou::math::deg_to_rad;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Axiom(char);

#[derive(Clone)]
struct Rule(char, String, fn(&Draw, &mut Turtle) -> ());

#[derive(Clone, Debug)]
struct Turtle {
    position: Vec2,
    previous: Vec2,
    states: Vec<(Vec2, f32)>,
    angle_step: f32,
    angle: f32,
    distance: f32,
}

impl Turtle {
    pub fn new(position: Vec2, angle: f32, angle_step: f32, distance: f32) -> Self {
        Self {
            position,
            previous: position,
            states: Vec::new(),
            angle_step,
            angle,
            distance,
        }
    }
    pub fn forward(&mut self) {
        self.previous = self.position.clone();
        let new_position = pt2(
            self.previous.x + deg_to_rad(self.angle).cos() * self.distance,
            self.previous.y + deg_to_rad(self.angle).sin() * self.distance,
        );
        self.position = new_position;
    }
    pub fn increase_angle(&mut self) {
        self.angle += self.angle_step;
    }
    pub fn decrease_angle(&mut self) {
        self.angle -= self.angle_step;
    }
    pub fn push_state(&mut self) {
        self.states.push((self.position, self.angle));
    }
    pub fn pop_state(&mut self) {
        if let Some(state) = self.states.pop() {
            self.position = state.0;
            self.angle = state.1;
        }
    }
}

#[derive(Clone)]
struct LSystem {
    alphabet: Vec<char>,
    axiom: Axiom,
    rules: Vec<Rule>,
    turtle: Turtle,
    string: String,
    depth: usize,
}

impl LSystem {
    fn new(axiom: char, turtle: Turtle, depth: usize) -> Self {
        Self {
            alphabet: vec![],
            axiom: Axiom(axiom),
            rules: vec![],
            turtle,
            string: String::new(),
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
    fn build(&mut self) {
        self.string.push(self.axiom.0);
        let mut tmp = String::new();
        for _ in 0..self.depth {
            let chars = self.string.as_bytes();
            for ch in chars {
                for rule in &self.rules {
                    if *ch as char == rule.0 {
                        tmp.push_str(&rule.1);
                    }
                }
            }
            self.string.clear();
            self.string.push_str(&tmp);
            tmp.clear();
        }
    }
    fn draw(&self, draw: &Draw) {
        let mut turtle = self.turtle.clone();
        for ch in self.string.as_bytes() {
            for rule in &self.rules {
                if *ch as char == rule.0 {
                    rule.2(draw, &mut turtle);
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
    //let mut lsys = LSystem::new('X', Turtle::new(pt2(0.0, 0.0), 90.0, 25.0, 5.0), 5)
    //    .add_alphabet(vec!['X', 'F', '-', '+', '[', ']'])
    //    .add_rule(Rule(
    //        'X',
    //        //String::from("F+[[X]-X]-F[-FX]+X"),
    //        String::from("F+[[X]-X]-F-[FX]+X"),
    //        |_draw, _turtle| {},
    //    ))
    //    .add_rule(Rule('F', String::from("FF"), |draw, turtle| {
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .weight(1.0)
    //            .color(WHITE);
    //        turtle.forward();
    //    }))
    //    .add_rule(Rule('-', String::from('-'), |_draw, turtle| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('+', String::from('+'), |_draw, turtle| {
    //        turtle.decrease_angle();
    //    }))
    //    .add_rule(Rule('[', String::from('['), |_draw, turtle| {
    //        turtle.push_state();
    //    }))
    //    .add_rule(Rule(']', String::from(']'), |_draw, turtle| {
    //        turtle.pop_state();
    //    }));
    let mut lsys = LSystem::new('F', Turtle::new(pt2(0.0, 0.0), 0.0, 90.0, 5.0), 4)
        .add_alphabet(vec!['F', '-', '+'])
        .add_rule(Rule('F', String::from("F+F-F-F+F"), |draw, turtle| {
        //.add_rule(Rule('F', String::from("F+F-F-F+F-"), |draw, turtle| {
            //draw.line()
            //    .start(turtle.previous)
            //    .end(turtle.position)
            //    .color(WHITE)
            //    .weight(1.0);
            draw.ellipse()
                .x_y(turtle.previous.x, turtle.previous.y)
                .color(WHITE)
                .resolution(4.0)
                .radius(1.0);
            turtle.forward();
        }))
        .add_rule(Rule('+', String::from("+"), |_draw, turtle| {
            turtle.increase_angle();
        }))
        .add_rule(Rule('-', String::from("-"), |_draw, turtle| {
            turtle.decrease_angle();
        }));
    //let mut lsys = LSystem::new('A', Turtle::new(pt2(0.0, 0.0), 50.0), 6)
    //    .add_alphabet(vec!['A', 'B', '-', '+'])
    //    //.add_rule(Rule('A', String::from("B-A-B"), |draw, turtle| {
    //    .add_rule(Rule('A', String::from("-A-B-A-"), |draw, turtle| {
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .color(WHITE)
    //            .weight(1.0);
    //        turtle.forward();
    //    }))
    //    //.add_rule(Rule('B', String::from("A+B+A"), |draw, turtle| {
    //    .add_rule(Rule('B', String::from("+B+A+B+"), |draw, turtle| {
    //        draw.line()
    //            .start(turtle.previous)
    //            .end(turtle.position)
    //            .color(WHITE)
    //            .weight(1.0);
    //        turtle.forward();
    //    }))
    //    .add_rule(Rule('+', String::from("+"), |_draw, turtle| {
    //        turtle.increase_angle();
    //    }))
    //    .add_rule(Rule('-', String::from("-"), |_draw, turtle| {
    //        turtle.decrease_angle();
    //    }));
    lsys.build();
    Model { lsys }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.lsys.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

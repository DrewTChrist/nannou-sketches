use nannou::noise::Perlin;
use nannou::prelude::*;

pub mod default_rules {
    use super::Rule;
    pub fn left_bracket() -> Rule {
        Rule('[', String::from("["), |_, turtle, _| {
            turtle.push_state();
        })
    }
    pub fn right_bracket() -> Rule {
        Rule(']', String::from("]"), |_, turtle, _| {
            turtle.pop_state();
        })
    }
    pub fn plus() -> Rule {
        Rule('+', String::from("+"), |_, turtle, _| {
            turtle.increase_angle();
        })
    }
    pub fn minus() -> Rule {
        Rule('-', String::from("-"), |_, turtle, _| {
            turtle.decrease_angle();
        })
    }
}

#[derive(Clone)]
pub struct Rule(
    pub char,
    pub String,
    pub fn(&Draw, &mut Turtle, usize) -> (),
);

#[derive(Clone, Debug)]
pub struct Turtle {
    pub position: Vec2,
    pub previous: Vec2,
    pub states: Vec<(Vec2, Vec2, f32)>,
    pub noise: Vec<(f64, f64)>,
    pub perlin: Perlin,
    pub angle_step: f32,
    pub angle: f32,
    pub distance: f32,
}

impl Turtle {
    pub fn new(position: Vec2, angle: f32, angle_step: f32, distance: f32) -> Self {
        Self {
            position,
            previous: position,
            states: Vec::new(),
            noise: Vec::new(),
            perlin: Perlin::new(),
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
        self.states.push((self.previous, self.position, self.angle));
    }
    pub fn pop_state(&mut self) {
        if let Some(state) = self.states.pop() {
            self.previous = state.0;
            self.position = state.1;
            self.angle = state.2;
        }
    }
}

#[derive(Clone)]
pub struct LSystem {
    pub alphabet: Vec<char>,
    pub axiom: String,
    pub rules: Vec<Rule>,
    pub noise: Vec<(f64, f64)>,
    pub turtle: Turtle,
    pub string: String,
    pub depth: usize,
}

impl LSystem {
    pub fn new(
        axiom: &str,
        depth: usize,
        position: Vec2,
        angle: f32,
        angle_step: f32,
        distance: f32,
    ) -> Self {
        Self {
            alphabet: vec![],
            axiom: String::from(axiom),
            rules: vec![],
            noise: Vec::new(),
            turtle: Turtle::new(position, angle, angle_step, distance),
            string: String::new(),
            depth,
        }
    }
    pub fn add_rule(&mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self.to_owned()
    }
    pub fn build(&mut self) {
        self.string.clear();
        self.string.push_str(&self.axiom);
        let mut tmp = String::new();
        let mut xnoise = random_range(-500.0, 500.0);
        let mut ynoise = random_range(-500.0, 500.0);
        for i in 0..self.depth {
            let chars = self.string.as_bytes();
            for ch in chars {
                for rule in &self.rules {
                    if *ch as char == rule.0 {
                        tmp.push_str(&rule.1);
                    }
                    if i == self.depth - 1 {
                        self.noise.push((xnoise, ynoise));
                    }
                    xnoise += 0.1;
                    ynoise += 0.1;
                }
            }
            self.string.clear();
            self.string.push_str(&tmp);
            tmp.clear();
        }
    }
    pub fn draw(&self, draw: &Draw) {
        let mut turtle = self.turtle.clone();
        turtle.noise = self.noise.clone();
        let mut index = 0;
        for ch in self.string.as_bytes() {
            for rule in &self.rules {
                if *ch as char == rule.0 {
                    rule.2(draw, &mut turtle, index);
                    index += 1;
                }
            }
        }
    }
}

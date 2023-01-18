use nannou::prelude::*;

#[derive(Clone)]
pub struct Axiom(pub char);

#[derive(Clone)]
pub struct Rule(pub char, pub String, pub fn(&Draw, &mut Turtle) -> ());

#[derive(Clone, Debug)]
pub struct Turtle {
    pub position: Vec2,
    pub previous: Vec2,
    pub states: Vec<(Vec2, f32)>,
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
pub struct LSystem {
    pub alphabet: Vec<char>,
    pub axiom: Axiom,
    pub rules: Vec<Rule>,
    pub turtle: Turtle,
    pub string: String,
    pub depth: usize,
}

impl LSystem {
    pub fn new(axiom: char, turtle: Turtle, depth: usize) -> Self {
        Self {
            alphabet: vec![],
            axiom: Axiom(axiom),
            rules: vec![],
            turtle,
            string: String::new(),
            depth,
        }
    }
    pub fn add_alphabet(&mut self, alphabet: Vec<char>) -> Self {
        self.alphabet = alphabet;
        self.to_owned()
    }
    pub fn add_rule(&mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self.to_owned()
    }
    pub fn build(&mut self) {
        self.string.clear();
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
    pub fn draw(&self, draw: &Draw) {
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

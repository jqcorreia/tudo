fn ease_out(a: &mut Animation, tick: u128) {
    let duration = 125.0;
    let distance = a.target as i32 - a.start_value as i32;

    let elapsed_percent = ((tick - a.start_tick) as f32 / duration).clamp(0.0, 1.0);

    let alpha = (elapsed_percent * 90.0).to_radians().sin();

    *a.value = (a.start_value as i32 + (distance as f32 * alpha) as i32) as u32;
}

fn linear(a: &mut Animation, tick: u128) {
    let duration = 100.0;
    let distance = a.target as i32 - a.start_value as i32;

    let elapsed_percent = ((tick - a.start_tick) as f32 / duration).clamp(0.0, 1.0);

    *a.value = (a.start_value as i32 + (distance as f32 * elapsed_percent) as i32) as u32;
}

pub enum AnimationType {
    Linear,
    EaseOut,
}

impl AnimationType {
    pub fn func(&self) -> fn(&mut Animation, u128) {
        match self {
            Self::Linear => linear,
            Self::EaseOut => ease_out,
        }
    }
}

// Hey sis!
//
// Low level animation code

pub struct Animation<'a> {
    start_value: u32,
    pub value: &'a mut u32,
    running: bool,
    start_tick: u128,
    pub target: u32,
    animation_type: AnimationType,
}

impl<'a> Animation<'a> {
    pub fn new(value: &'a mut u32, target: u32, atype: AnimationType) -> Animation {
        Animation {
            start_value: *value,
            value,
            running: false,
            start_tick: 0,
            target,
            animation_type: atype,
        }
    }
    pub fn start(&mut self, tick: u128) {
        self.start_value = *self.value;
        self.running = true;
        self.start_tick = tick;
    }
    pub fn set_target(&mut self, target: u32, tick: Option<u128>) {
        if target == self.target {
            return;
        }
        self.target = target;
        match tick {
            Some(v) => self.start(v),
            None => (),
        };
    }

    pub fn tick(&mut self, tick: u128) {
        (self.animation_type.func())(self, tick);
    }
}

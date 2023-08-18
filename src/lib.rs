#[derive(Debug)]
pub struct App {
    funcs: Vec<Func>,
}

impl App {
    pub fn new() -> Self {
        App { funcs: vec![] }
    }

    pub fn register_fn(&mut self, func: Func) {
        self.funcs.push(func);
    }
}

#[derive(Debug)]
pub struct Func {
    pub opts: FuncOpts,
    pub trigger: Trigger,
}

#[derive(Debug)]
pub struct FuncOpts {
    pub name: String,
}

#[derive(Debug)]
pub enum Trigger {
    Event {
        name: String,
        expression: Option<String>,
    },
    Cron {
        name: String,
    },
}

pub fn create_function(opts: FuncOpts, trigger: Trigger) -> Func {
    return Func { opts, trigger };
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct App {
    funcs: HashMap<String, Func>,
}

impl App {
    pub fn new() -> Self {
        App {
            funcs: HashMap::new(),
        }
    }

    pub fn register_fn(&mut self, func: Func) {
        self.funcs.insert(func.slug(), func);
    }
}

#[derive(Debug)]
pub struct Func {
    pub opts: FuncOpts,
    pub trigger: Trigger,
}

impl Func {
    pub fn slug(&self) -> String {
        self.opts.name.clone()
    }
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

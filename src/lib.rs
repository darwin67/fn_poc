use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct App<T: Serialize + for<'a> Deserialize<'a>> {
    funcs: HashMap<String, Box<Func<T>>>,
}

impl<T> App<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        App {
            funcs: HashMap::new(),
        }
    }

    pub fn register_fn(&mut self, func: Func<T>) {
        self.funcs.insert(func.slug(), Box::new(func));
    }
}

pub struct Input<T: Serialize + for<'a> Deserialize<'a>> {
    pub event: T,
    pub events: Vec<T>,
}

pub struct Func<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub opts: FuncOpts,
    pub trigger: Trigger,
    pub func: fn(Input<T>) -> Result<String, String>,
}

impl<T> Func<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub fn slug(&self) -> String {
        self.opts.name.clone()
    }
}

pub struct FuncOpts {
    pub name: String,
}

pub enum Trigger {
    Event {
        name: String,
        expression: Option<String>,
    },
    Cron {
        name: String,
    },
}

pub fn create_function<T>(
    opts: FuncOpts,
    trigger: Trigger,
    func: fn(Input<T>) -> Result<String, String>,
) -> Func<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    return Func {
        opts,
        trigger,
        func,
    };
}

#[derive(Deserialize, Serialize)]
pub struct Event<D, U> {
    id: Option<String>,
    name: String,
    data: D,
    user: Option<U>,
    ts: Option<u64>,
}

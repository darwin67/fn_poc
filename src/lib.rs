pub struct Func {
    pub opts: FuncOpts,
    pub trigger: Trigger,
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

pub fn create_function(opts: FuncOpts, trigger: Trigger) -> Func {
    return Func { opts, trigger };
}

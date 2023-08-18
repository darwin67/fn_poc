use fn_proc::{create_function, App, Func, FuncOpts, Trigger::Event};

fn main() {
    let mut app = App::new();

    app.register_fn(dummy_func());

    println!("App: {:#?}", app);
}

fn dummy_func() -> Func {
    create_function(
        FuncOpts {
            name: "dummy".to_string(),
        },
        Event {
            name: "test/event".to_string(),
            expression: None,
        },
    )
}

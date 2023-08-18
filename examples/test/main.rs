use fn_proc::{create_function, App, Event, Func, FuncOpts, Input, Trigger};
use serde::{Deserialize, Serialize};

fn main() {
    let mut app = App::new();

    app.register_fn(dummy_func());
    app.register_fn(hello_func());

    // println!("App: {:#?}", app);
}

#[derive(Deserialize, Serialize)]
struct Yolo {
    yo: String,
    lo: String,
}

#[derive(Deserialize, Serialize)]
struct YoloU {}

fn dummy_func() -> Func<Event<Yolo, YoloU>> {
    create_function(
        FuncOpts {
            name: "dummy".to_string(),
        },
        Trigger::Event {
            name: "test/event".to_string(),
            expression: None,
        },
        |_input: Input<Event<Yolo, YoloU>>| {
            println!("in dummy func!");
            Ok("something".to_string())
        },
    )
}

#[derive(Deserialize, Serialize)]
struct Hello {
    val: i32,
}

#[derive(Deserialize, Serialize)]
struct HelloU {}

fn hello_func() -> Func<Event<Hello, HelloU>> {
    create_function(
        FuncOpts {
            name: "hello".to_string(),
        },
        Trigger::Event {
            name: "test/hello".to_string(),
            expression: None,
        },
        |_input: Input<Event<Hello, HelloU>>| {
            println!("in hello func!");
            Ok("hello".to_string())
        },
    )
}

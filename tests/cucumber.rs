use cucumber::{cucumber, before, after};

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    s: String
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            s: "a default string".to_string()
        }
    }
}

mod example {
    use cucumber::steps;

// Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given regex "the following (.*) transports" |world, name, _step| {
            // Set up your context in given steps
            world.s = format!("{}", name[1]);
        };
    });

}


// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./tests/features", // Path to our feature files
    world: crate::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ], 
    after: &[
        an_after_fn // Optional; called after each scenario
    ] 
}
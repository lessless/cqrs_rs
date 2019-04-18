use actix::prelude::*;
use actix_rt::spawn;
use futures::Future;

/// Define `Increment` message
struct Increment();

impl Message for Increment {
    type Result = usize;
}

/// Actor
struct MyActor {
    counter: usize,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Increment` message
impl Handler<Increment> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Increment, _: &mut Context<Self>) -> Self::Result {
        self.count + 1;
        self.count
    }
}

fn main() -> std::io::Result<()> {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let addr = MyActor { counter: 0 }.start();

        // send message and get future for result
        let res = addr.send(Increment());

        // handle() returns tokio handle
        spawn(
            res.map(|res| {
                println!("RESULT: {}", res == 20);

                // stop system and exit
                System::current().stop();
            })
            .map_err(|_| ()),
        );
    })
}

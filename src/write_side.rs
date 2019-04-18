// mod event_store;

#[derive(Debug)]
struct CreateCommand{
    kind: String,
    counter_id: u32,
}

impl CreateCommand {
    pub fn new(counter_id: u32) -> CreateCommand {
        CreateCommand{kind: String::from("create"), counter_id}
    }
}

pub fn create(counter_id: u32) {
    let create = CreateCommand::new(counter_id);
    handle(create)
}

fn handle(create: CreateCommand) {
    print!("Handling {:?}", create );
    print!("{:?} ", event_store::find(create.counter_id))
}

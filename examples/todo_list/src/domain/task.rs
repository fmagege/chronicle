use chronicle_commander::Aggregate;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Event {
    Create { description: String },
    ChangeDescription { description: String },
    Complete,
    Archive,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Command {
    Create { description: String },
    ChangeDescription { description: String },
    Complete,
    Archive,
}

#[derive(Debug, Clone)]
pub enum ValidationError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub description: String,
}

pub struct Task;

impl Aggregate for Task {
    type State = Option<State>;
    type Event = Event;
    type Command = Command;
    type ValidationError = ValidationError;

    fn initial_state() -> Option<State> {
        None
    }

    fn apply_event(_state: &mut Option<State>, _command: Event) {
        unimplemented!()
    }

    fn handle_command(_state: &Option<State>,
                      _command: Command)
                      -> Result<Vec<Event>, ValidationError> {
        unimplemented!()
    }
}
use eventsourced::{DateTime, EmittedEvent, Event, State};

mod eventsourced;

enum EventKind {
    Swap { a: usize, b: usize },
    Remove(usize),
    Add(Board),
}

enum ActorKind {
    Admin { user_name: String },
    User { id: u32 },
}

#[derive(Debug, Clone)]
enum Tile {
    Black,
    White,
}

#[derive(Debug, Clone)]
struct Board {
    tile: Tile,
}

struct ProgramState {
    boards: Vec<Board>,
}

impl State<EventKind, ActorKind> for ProgramState {
    type Event = EmittedEvent<EventKind, ActorKind>;

    fn initial() -> Self {
        Self { boards: Vec::new() }
    }

    fn handle_event(&mut self, event: Self::Event) -> Result<&mut Self, ()> {
        match event.kind() {
            EventKind::Add(b) => self.boards.push(b.clone()),
            &EventKind::Remove(idx) => {
                if self.boards.len() < idx {
                    return Err(());
                }

                self.boards.remove(idx);
            }
            &EventKind::Swap { a, b } => {
                if a > self.boards.len() || b > self.boards.len() {
                    return Err(());
                }

                self.boards.swap(a, b);
            }
        }

        Ok(self)
    }
}

fn main() {
    println!("Hello, world!");
}

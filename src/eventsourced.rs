#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    inner: u64,
}

impl DateTime {
    pub fn now() -> Self {
        DateTime { inner: 0 }
    }
}

pub trait Event {
    type Kind;
    type Actor;

    fn kind(&self) -> &Self::Kind;
    fn into_kind(self) -> Self::Kind;

    fn created_at(&self) -> DateTime;

    fn last_updated_at(&self) -> Option<DateTime>;

    fn actor(&self) -> &Self::Actor;
    fn into_actor(self) -> Self::Actor;
}

/// Default implementation of an Event
pub struct EmittedEvent<Kind, Actor = ()> {
    kind: Kind,
    actor: Actor,
    created_at: DateTime,
    last_updated_at: Option<DateTime>,
}

impl<Kind, Actor> EmittedEvent<Kind, Actor> {
    fn new(kind: Kind, actor: Actor) -> Self {
        EmittedEvent {
            kind,
            actor,
            created_at: DateTime::now(),
            last_updated_at: None,
        }
    }
}

impl<Kind, Actor> Event for EmittedEvent<Kind, Actor> {
    type Kind = Kind;
    type Actor = Actor;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }

    fn into_kind(self) -> Self::Kind {
        self.kind
    }

    fn actor(&self) -> &Self::Actor {
        &self.actor
    }

    fn into_actor(self) -> Self::Actor {
        self.actor
    }

    fn created_at(&self) -> DateTime {
        self.created_at
    }

    fn last_updated_at(&self) -> Option<DateTime> {
        self.last_updated_at
    }
}

pub trait State<Kind, Actor = (), Failure = ()>
where
    Self: Sized,
{
    type Event: Event;

    fn initial() -> Self;

    fn handle_event(&mut self, event: Self::Event) -> Result<&mut Self, Failure>;

    fn aggregate(events: Vec<Self::Event>) -> Result<Self, Failure> {
        let mut state = Self::initial();

        for event in events {
            state.handle_event(event)?;
        }

        Ok(state)
    }
}

#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Event {
    id: u64,
    name: String,
    details: String,
    location: String,
    timestamp: u64,
    attendees: Vec<String>,
    // Add more fields as needed
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Ticket {
    ticket_id: u64,
    event_id: u64,
    attendee: String,
    ticket_type: String,
    ticket_price: u64,
    // Add more ticket-specific details as needed
}

impl Storable for Event {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Event {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Ticket {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Ticket {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static EVENTS_STORAGE: RefCell<StableBTreeMap<u64, Event, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TICKETS_STORAGE: RefCell<StableBTreeMap<u64, Ticket, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EventPayload {
    name: String,
    details: String,
    location: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TicketPayload {
    event_id: u64,
    attendee: String,
    ticket_type: String,
    ticket_price: u64,
}

#[ic_cdk::update]
fn add_event(event: EventPayload) -> Option<Event> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let event = Event {
        id,
        name: event.name,
        details: event.details,
        location: event.location,
        timestamp: time(),
        attendees: Vec::new(),
    };
    do_insert_event(&event);
    Some(event)
}

#[ic_cdk::update]
fn update_event(event_id: u64, payload: EventPayload) -> Result<Event, Error> {
    match EVENTS_STORAGE.with(|service| service.borrow().get(&event_id)) {
        Some(mut event) => {
            event.name = payload.name;
            event.details = payload.details;
            event.location = payload.location;
            do_insert_event(&event);
            Ok(event)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update an event with id={}. event not found", event_id),
        }),
    }    
}

#[ic_cdk::update]
fn delete_event(event_id: u64) -> Result<Event, Error> {
    match EVENTS_STORAGE.with(|service| service.borrow_mut().remove(&event_id)) {
        Some(event) => Ok(event),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete an event with id={}. event not found.", event_id),
        }),
    }
}

#[ic_cdk::query]
fn get_event(event_id: u64) -> Result<Event, Error> {
    match _get_event(&event_id) {
        Some(event) => Ok(event),
        None => Err(Error::NotFound {
            msg: format!("an event with id={} not found", event_id),
        }),
    }
}

#[ic_cdk::query]
fn get_event_by_name(name: String) -> Option<Event> {
    let lowercase_name = name.to_lowercase();
    EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        for (_, event) in borrowed_service.iter() {
            if event.name.to_lowercase() == lowercase_name {
                return Some(event.clone());
            }
        }
        None
    })
}

#[ic_cdk::query]
fn get_events() -> Vec<Event> {
    EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        borrowed_service.iter().map(|(_, event)| event.clone()).collect()
    })
}


#[ic_cdk::query]
fn get_tickets(event_id: u64) -> Vec<Ticket> {
    TICKETS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        borrowed_service
            .iter()
            .filter(|(_, ticket)| ticket.event_id == event_id)
            .map(|(_, ticket)| ticket.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn add_ticket(ticket: TicketPayload) -> Option<Ticket> {
    let event_id = ticket.event_id;
    match _get_event(&event_id) {
        Some(_) => {
            let id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");
            let ticket = Ticket {
                ticket_id: id,
                event_id: ticket.event_id,
                attendee: ticket.attendee,
                ticket_type: ticket.ticket_type,
                ticket_price: ticket.ticket_price,
            };
            do_insert_ticket(&ticket);
            Some(ticket)
        }
        None => None,
    }
}

#[ic_cdk::update]
fn delete_ticket(ticket_id: u64) -> Result<Ticket, Error> {
    match TICKETS_STORAGE.with(|service| service.borrow_mut().remove(&ticket_id)) {
        Some(ticket) => Ok(ticket),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a ticket with id={}. ticket not found.", ticket_id),
        }),
    }
}

// helper method to perform insert for events.
fn do_insert_event(event: &Event) {
    EVENTS_STORAGE.with(|service| service.borrow_mut().insert(event.id, event.clone()));
}

// helper method to perform insert for tickets.
fn do_insert_ticket(ticket: &Ticket) {
    TICKETS_STORAGE.with(|service| service.borrow_mut().insert(ticket.ticket_id, ticket.clone()));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// a helper method to get an event by id.
fn _get_event(id: &u64) -> Option<Event> {
    EVENTS_STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();

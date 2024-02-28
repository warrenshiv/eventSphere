#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use sha2::{Digest, Sha256};
use std::fmt;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Event {
    id: u64,
    event_name: String,
    details: String,
    location: String,
    start_date: Date,
    end_date: Date,
    timestamp: u64,
    attendees: Vec<Attendee>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
    password: String,
    role: UserRole,
    created_at: u64,
    updated_at: Option<u64>,
}

// Define the Date struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
enum TicketType {
    #[default]
    Regular,
    VVIP,
    VIP,
    Discount,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]

enum UserRole {
    #[default]
    Admin,
    User,
}

impl fmt::Display for TicketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketType::Regular => write!(f, "Regular"),
            TicketType::VVIP => write!(f, "VVIP"),
            TicketType::VIP => write!(f, "VIP"),
            TicketType::Discount => write!(f, "Discount"),
        }
    }
}

impl Date {
    // Constructor method to create a new Date instance
    fn new(day: u32, month: u32, year: u32) -> Self {
        Date { year, month, day }
    }

    // Method to parse a date string in "DD-MM-YYYY" format and create a Date instance
    fn from_string(date_string: &str) -> Option<Self> {
        let parts: Vec<&str> = date_string.split('-').collect();
        if parts.len() == 3 {
            if let (Ok(day), Ok(month), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                return Some(Date::new(day, month, year));
            }
        }
        None
    }
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Attendee {
    attendee_name: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Ticket {
    ticket_id: u64,
    event_id: u64,
    ticket_type: String,
    ticket_price: u64,
    num_tickets: u32,
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

// Implement Storable and BoundedStorable  traits for User
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Attendee {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Attendee {
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

    static ATTENDEES_STORAGE: RefCell<StableBTreeMap<u64, Attendee, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static TICKETS_STORAGE: RefCell<StableBTreeMap<u64, Ticket, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))), 0)
            .expect("Cannot create a counter")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EventPayload {
    event_name: String,
    details: String,
    location: String,
    username: String,
    password: String,
    start_date: String,
    end_date: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct AttendeePayload {
    attendee_name: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TicketPayload {
    event_id: u64,
    ticket_type: TicketType,
    ticket_price: u64,
    num_tickets: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct TicketPurchasePayload {
    event_id: u64,
    ticket_type: TicketType,
    attendee_name: String,
    num_tickets: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    username: String,
    email: String,
    password: String,
    role: UserRole,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload1 {
    username: String,
    password: String,
}

#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, Error> {
    // Validate user payload all fields are required
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::CustomError("All fields are required".to_string()));
    }

    // Hash the password
    let password = hash_password(&payload.password)?;

    let id = USER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let user = User {
        id,
        username: payload.username,
        email: payload.email,
        password,
        role: payload.role,
        created_at: time(),
        updated_at: None,
    };

    do_insert_user(user.clone());

    Ok(user)
}

// Helper function to hash passwords using SHA-256
fn hash_password(password: &str) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    hasher.update(password);
    Ok(hex::encode(hasher.finalize()))
}

#[ic_cdk::query]
fn get_user(user_id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|storage| storage.borrow().get(&user_id)) {
        Some(user) => Ok(user.clone()),
        None => Err(Error::NotFound {
            msg: format!("User with ID {} not found.", user_id),
        }),
    }
}

#[ic_cdk::update]
fn update_user(user_id: u64, payload: UserPayload, payload2: UserPayload1) -> Result<User, Error> {
    // Validate user payload: all fields are required
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::CustomError("All fields are required".to_string()));
    }

    // Clone the username before passing it to authenticate_admin
    let username = payload2.username.clone();
    let password = payload2.password.clone();

    let _user = authenticate_user(username, password)?;

    let updated_user = USER_STORAGE.with(|storage| {
        let mut user_storage = storage.borrow_mut();
        if let Some(user) = user_storage.get(&user_id) {
            let updated_user = User {
                id: user.id,
                username: payload.username.clone(),
                email: payload.email.clone(),
                password: payload.password.clone(),
                role: payload.role,
                created_at: user.created_at,
                updated_at: Some(time()),
            };
            user_storage.insert(user_id, updated_user.clone());
            Some(updated_user)
        } else {
            None
        }
    });

    match updated_user {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("User with ID {} not found.", user_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_user(user_id: u64, payload: UserPayload1) -> Result<User, Error> {
    // Clone the username before passing it to authenticate_admin
    let username = payload.username.clone();
    let password = payload.password.clone();

    let _user = authenticate_user(username, password)?;
    match USER_STORAGE.with(|storage| storage.borrow_mut().remove(&user_id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("User with ID {} not found.", user_id),
        }),
    }
}

// helper method to perform insert for users.
fn do_insert_user(user: User) {
    USER_STORAGE.with(|m| m.borrow_mut().insert(user.id, user));
}

fn authenticate_admin(username: String, password: String) -> Result<User, Error> {
    // Find the user by username (assuming username is unique)
    let user = USER_STORAGE.with(|storage| {
        let borrowed_storage = storage.borrow();
        borrowed_storage
            .iter()
            .find(|(_, user)| user.username == username)
            .map(|(_, user)| user.clone())
    });

    // Check if the user was found
    match user {
        Some(user) => {
            // Hash the provided password
            let hashed_password = hash_password(&password)?;

            // Check if the hashed password matches
            if user.password == hashed_password {
                // Check if the user's role is admin
                if user.role == UserRole::Admin {
                    Ok(user) // Authentication successful
                } else {
                    Err(Error::CustomError("Insufficient privileges".to_string()))
                    // User is not admin
                }
            } else {
                Err(Error::CustomError("Incorrect password".to_string())) // Incorrect password
            }
        }
        None => Err(Error::NotFound {
            msg: "User not found".to_string(),
        }), // User not found
    }
}

fn authenticate_user(username: String, password: String) -> Result<User, Error> {
    // Find the user by username (assuming username is unique)
    let user = USER_STORAGE.with(|storage| {
        let borrowed_storage = storage.borrow();
        borrowed_storage
            .iter()
            .find(|(_, user)| user.username == username)
            .map(|(_, user)| user.clone())
    });

    // Check if the user was found
    match user {
        Some(user) => {
            // Hash the provided password
            let hashed_password = hash_password(&password)?;

            // Check if the hashed password matches
            if user.password == hashed_password {
                // Check if the user's role is User or Admin
                if user.role == UserRole::User || user.role == UserRole::Admin {
                    Ok(user) // Authentication successful
                } else {
                    Err(Error::CustomError("Insufficient privileges".to_string()))
                    // User does not have sufficient privileges
                }
            } else {
                Err(Error::CustomError("Incorrect password".to_string())) // Incorrect password
            }
        }
        None => Err(Error::NotFound {
            msg: "User not found".to_string(),
        }), // User not found
    }
}

// Event Queries
#[ic_cdk::update]
fn add_event(event: EventPayload) -> Result<Event, Error> {
    // Clone the username before passing it to authenticate_admin
    let username = event.username.clone();
    let password = event.password.clone();

    let _user = authenticate_admin(username, password)?;

    // Validate event payload: all fields are required
    if event.event_name.is_empty()
        || event.details.is_empty()
        || event.location.is_empty()
        || event.start_date.is_empty()
        || event.end_date.is_empty()
    {
        return Err(Error::CustomError("All fields are required".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let timestamp = time(); // Get the current timestamp

    // Parse start_date and end_date strings into Date structs
    let start_date = Date::from_string(&event.start_date)
        .ok_or_else(|| Error::CustomError("Invalid start date format".to_string()))?;
    let end_date = Date::from_string(&event.end_date)
        .ok_or_else(|| Error::CustomError("Invalid end date format".to_string()))?;

    let event = Event {
        id,
        event_name: event.event_name,
        details: event.details,
        location: event.location,
        start_date,
        end_date,
        timestamp,
        attendees: Vec::new(),
    };
    do_insert_event(&event);
    Ok(event)
}

#[ic_cdk::update]
fn update_event(event_id: u64, payload: EventPayload) -> Result<Event, Error> {
    // Clone the username before passing it to authenticate_admin
    let username = payload.username.clone();
    let password = payload.password.clone();

    let _user = authenticate_admin(username, password)?;

    // Validate that all fields in the payload are filled
    if payload.event_name.is_empty()
        || payload.details.is_empty()
        || payload.location.is_empty()
        || payload.start_date.is_empty()
        || payload.end_date.is_empty()
    {
        return Err(Error::CustomError(
            "All fields in the payload are required".to_string(),
        ));
    }

    match EVENTS_STORAGE.with(|service| service.borrow().get(&event_id)) {
        Some(mut event) => {
            // Parse start_date and end_date strings into Date structs
            let start_date = Date::from_string(&payload.start_date)
                .ok_or_else(|| Error::CustomError("Invalid start date format".to_string()))?;
            let end_date = Date::from_string(&payload.end_date)
                .ok_or_else(|| Error::CustomError("Invalid end date format".to_string()))?;

            event.event_name = payload.event_name;
            event.details = payload.details;
            event.location = payload.location;
            event.start_date = start_date; // Update start_date field with parsed Date
            event.end_date = end_date; // Update end_date field with parsed Date

            do_insert_event(&event);
            Ok(event)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update an event with id={}. event not found",
                event_id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_event(payload: UserPayload1, event_id: Option<u64>) -> Result<Event, Error> {
    // Authenticate the user
    let _user = authenticate_admin(payload.username.clone(), payload.password)?;

    // Validate that the event_id is provided
    let event_id = match event_id {
        Some(id) => id,
        None => return Err(Error::CustomError("Event ID must be provided".to_string())),
    };

    // Proceed with the deletion as before
    match EVENTS_STORAGE.with(|service| service.borrow_mut().remove(&event_id)) {
        Some(event) => Ok(event),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete an event with id={}. event not found.",
                event_id
            ),
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
fn get_event_by_name(event_name: String) -> Result<Option<Event>, Error> {
    // Validate that the event username is not empty
    if event_name.is_empty() {
        return Err(Error::CustomError(
            "Event name cannot be empty".to_string(),
        ));
    }

    let lowercase_name = event_name.to_lowercase();
    let mut found_event = None;

    EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        for (_, event) in borrowed_service.iter() {
            if event.event_name.to_lowercase() == lowercase_name {
                found_event = Some(event.clone());
                break;
            }
        }
    });

    // Check if an event with the provided username was found
    if let Some(event) = found_event {
        Ok(Some(event))
    } else {
        Err(Error::NotFound {
            msg: format!("Event with name '{}' not found", event_name),
        })
    }
}

#[ic_cdk::query]
fn get_events() -> Result<Vec<Event>, Error> {
    let events = EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        let events: Vec<Event> = borrowed_service
            .iter()
            .map(|(_, event)| event.clone())
            .collect();

        events
    });

    if events.is_empty() {
        Err(Error::CustomError("No events available".to_string()))
    } else {
        Ok(events)
    }
}

#[ic_cdk::query]
fn get_upcoming_events() -> Result<Vec<Event>, Error> {
    let current_time = time();

    let upcoming_events = EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        let events: Vec<Event> = borrowed_service
            .iter()
            .filter(|(_, event)| event.timestamp <= current_time)
            .map(|(_, event)| event.clone())
            .collect();

        if events.is_empty() {
            Err(Error::CustomError(
                "No upcoming events available".to_string(),
            ))
        } else {
            Ok(events)
        }
    });

    upcoming_events
}

#[ic_cdk::query]
fn get_past_events() -> Result<Vec<Event>, Error> {
    let current_time = time();

    let past_events = EVENTS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        let events: Vec<Event> = borrowed_service
            .iter()
            .filter(|(_, event)| event.timestamp > current_time)
            .map(|(_, event)| event.clone())
            .collect();

        if events.is_empty() {
            Err(Error::CustomError("No past events available".to_string()))
        } else {
            Ok(events)
        }
    });

    past_events
}

#[ic_cdk::update]
fn add_attendees(
    payload: UserPayload1,
    event_id: u64,
    attendee_payload: AttendeePayload,
) -> Result<(), Error> {
    // Authenticate the user
    let _user = authenticate_user(payload.username.clone(), payload.password.clone())?;

    // Validate that all fields in the payload are filled
    if payload.username.is_empty()
        || payload.password.is_empty()
        || attendee_payload.attendee_name.is_empty()
    {
        return Err(Error::CustomError(
            "All fields in the payload are required".to_string(),
        ));
    }

    match EVENTS_STORAGE.with(|service| {
        let mut events_storage = service.borrow_mut();
        if let Some(event) = events_storage.get(&event_id) {
            // Use get to get a reference to the event
            let attendee = Attendee {
                attendee_name: attendee_payload.attendee_name.clone(), // Cloning the username field
            };
            let mut cloned_event = event.clone(); // Cloning the event to modify it
            cloned_event.attendees.push(attendee); // Add the new attendee to the cloned event's attendees vector
            events_storage.insert(event_id, cloned_event); // Replace the old event with the modified one
            Ok(()) // Return Ok if successful
        } else {
            Err(Error::NotFound {
                msg: format!("Event with id={} not found", event_id),
            })
        }
    }) {
        Ok(()) => {
            // Return a success message
            Ok(println!("Attendee successfully added to the event!"))
        }
        Err(err) => Err(err),
    }
}

#[ic_cdk::query]
fn get_attendees(event_id: u64) -> Result<Vec<Attendee>, Error> {
    match EVENTS_STORAGE.with(|service| {
        let events_storage = service.borrow();
        if let Some(event) = events_storage.get(&event_id) {
            Ok(event.attendees.clone()) // Return a clone of the attendees vector
        } else {
            Err(Error::NotFound {
                msg: format!("event with id={} not found", event_id),
            })
        }
    }) {
        Ok(attendees) => Ok(attendees),
        Err(err) => Err(err),
    }
}

// Ticket Queries
#[ic_cdk::update]
fn generate_tickets(
    ticket_payload: TicketPayload,
    payload: UserPayload1,
) -> Result<Option<Vec<Ticket>>, Error> {
    // Clone the username before passing it to authenticate_admin
    let username = payload.username.clone();
    let password = payload.password.clone();

    let _user = authenticate_admin(username, password)?;

    let event_id = ticket_payload.event_id;
    let num_tickets = ticket_payload.num_tickets;

    match _get_event(&event_id) {
        Some(_) => {
            let mut tickets = Vec::new();
            for _ in 0..num_tickets {
                let id = ID_COUNTER
                    .with(|counter| {
                        let current_value = *counter.borrow().get();
                        counter.borrow_mut().set(current_value + 1)
                    })
                    .expect("cannot increment id counter");
                let new_ticket = Ticket {
                    ticket_id: id,
                    event_id: ticket_payload.event_id,
                    ticket_type: match ticket_payload.ticket_type {
                        TicketType::VVIP => "VVIP".to_string(),
                        TicketType::VIP => "VIP".to_string(),
                        TicketType::Regular => "Regular".to_string(),
                        TicketType::Discount => "Discount".to_string(),
                    },
                    ticket_price: ticket_payload.ticket_price,
                    num_tickets: 1,
                };
                do_insert_ticket(&new_ticket);
                tickets.push(new_ticket);
            }
            Ok(Some(tickets))
        }
        None => Ok(None),
    }
}

#[ic_cdk::query]
fn get_tickets(event_id: u64) -> Result<Vec<Ticket>, Error> {
    TICKETS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        let tickets: Vec<Ticket> = borrowed_service
            .iter()
            .filter(|(_, ticket)| ticket.event_id == event_id)
            .map(|(_, ticket)| ticket.clone())
            .collect();

        if tickets.is_empty() {
            // If no tickets found for the specified event_id, return an error
            return Err(Error::NotFound {
                msg: format!("No tickets found for event with ID: {}", event_id),
            });
        }

        Ok(tickets)
    })
}

#[ic_cdk::update]
fn delete_ticket(ticket_id: u64, payload: UserPayload1) -> Result<Ticket, Error> {
    // Clone the username before passing it to authenticate_admin
    let username = payload.username.clone();
    let password = payload.password.clone();

    let _user = authenticate_admin(username, password)?;

    match TICKETS_STORAGE.with(|service| service.borrow_mut().remove(&ticket_id)) {
        Some(ticket) => Ok(ticket),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a ticket with id={}. ticket not found.",
                ticket_id
            ),
        }),
    }
}

// Get the number of available tickets for a specific event and ticket type
#[ic_cdk::query]
fn get_available_tickets_count(event_id: u64, ticket_type: TicketType) -> usize {
    TICKETS_STORAGE.with(|service| {
        let borrowed_service = service.borrow();
        borrowed_service
            .iter()
            .filter(|(_, ticket)| {
                ticket.event_id == event_id && &ticket.ticket_type == &ticket_type.to_string()
            })
            .map(|(_, ticket)| ticket.num_tickets as usize)
            .sum()
    })
}

// Ticket Purchase
#[ic_cdk::update]
fn purchase_ticket(
    payload: TicketPurchasePayload,
    payload1: UserPayload1,
) -> Result<(Vec<Ticket>, u64), Error> {
    // Validate that all fields in the payload are filled
    if payload.attendee_name.is_empty() {
        return Err(Error::CustomError(
            "All fields in the payload are required".to_string(),
        ));
    }

    let event_id = payload.event_id;
    let ticket_type = payload.ticket_type.clone();
    let attendee_name = payload.attendee_name;
    let num_tickets = payload.num_tickets as usize; // Convert num_tickets to usize

    // Check if the event exists
    match _get_event(&event_id) {
        Some(_event) => {
            // Check if there are enough tickets available
            let available_tickets_count =
                get_available_tickets_count(event_id, ticket_type.clone());
            if available_tickets_count < num_tickets {
                return Err(Error::CustomError(format!(
                    "Not enough tickets available for type: {}",
                    ticket_type
                )));
            }

            // Fetch ticket price from TICKETS_STORAGE
            let ticket_price = TICKETS_STORAGE
                .with(|service| {
                    let borrowed_service = service.borrow();
                    // Find the ticket by event_id and ticket_type
                    borrowed_service
                        .iter()
                        .find(|(_, ticket)| {
                            ticket.event_id == event_id
                                && &ticket.ticket_type == &ticket_type.to_string()
                        })
                        .map(|(_, ticket)| ticket.ticket_price)
                })
                .unwrap_or_else(|| {
                    // If the ticket price is not found, return a default price or handle the error as needed
                    ic_cdk::print(
                        "Ticket price not found for the specified ticket type and event ID.",
                    );
                    0
                });

            // Calculate total cost
            let total_cost = ticket_price * num_tickets as u64;

            // Create tickets for the purchase
            let mut tickets = Vec::new();
            for _ in 0..num_tickets {
                // Fetch a ticket from TICKETS_STORAGE
                let ticket = TICKETS_STORAGE.with(|service| {
                    let mut borrowed_service = service.borrow_mut();
                    // Find and remove the first available ticket by event_id and ticket_type
                    let ticket = borrowed_service
                        .iter()
                        .find(|(_, ticket)| {
                            ticket.event_id == event_id
                                && &ticket.ticket_type == &ticket_type.to_string()
                        })
                        .map(|(id, mut ticket)| {
                            let cloned_ticket = ticket.clone();
                            if ticket.num_tickets == 1 {
                                // If it's the last ticket of this type, remove it from storage
                                borrowed_service.remove(&id);
                            } else {
                                // Decrement the number of available tickets
                                ticket.num_tickets -= 1;
                            }
                            cloned_ticket
                        });
                    ticket
                });

                if let Some(ticket) = ticket {
                    tickets.push(ticket);
                } else {
                    // Handle the case where a ticket is not found (should not occur if availability is properly checked)
                    return Err(Error::CustomError(format!(
                        "Failed to find available ticket for type: {}",
                        ticket_type
                    )));
                }
            }

            // Update the event with the attendee's name
            add_attendees(
                UserPayload1 {
                    username: payload1.username,
                    password: payload1.password,
                },
                event_id,
                AttendeePayload {
                    attendee_name: attendee_name,
                },
            )
            .expect("Failed to add attendee");

            // Return purchased tickets and total cost
            Ok((tickets, total_cost))
        }
        None => Err(Error::NotFound {
            msg: format!("Event with ID {} not found.", event_id),
        }),
    }
}

// helper method to perform insert for events.
fn do_insert_event(event: &Event) {
    EVENTS_STORAGE.with(|service| service.borrow_mut().insert(event.id, event.clone()));
}

// helper method to perform insert for tickets.
fn do_insert_ticket(ticket: &Ticket) {
    TICKETS_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(ticket.ticket_id, ticket.clone())
    });
}

#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
enum Error {
    NotFound { msg: String },
    AlreadyExists { msg: String },
    CustomError(String),
}

// a helper method to get an event by id.
fn _get_event(id: &u64) -> Option<Event> {
    EVENTS_STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();

# Event Management System 📅👥🎟️

This is a Rust application for managing events, users, and tickets. It provides functionality to register users, add, update, delete events, manage attendees, generate tickets, and purchase tickets.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with event_sphere, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd event_sphere/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

## Table of Contents
- [Dependencies](#dependencies)
- [Usage](#usage)
  - [Register User](#register-user)
  - [Get User](#get-user)
  - [Update User](#update-user)
  - [Delete User](#delete-user)
  - [Add Event](#add-event)
  - [Update Event](#update-event)
  - [Delete Event](#delete-event)
  - [Get Event](#get-event)
  - [Get Event By Name](#get-event-by-name)
  - [Get Events](#get-events)
  - [Get Upcoming Events](#get-upcoming-events)
  - [Get Past Events](#get-past-events)
  - [Add Attendees](#add-attendees)
  - [Get Attendees](#get-attendees)
  - [Generate Tickets](#generate-tickets)
  - [Get Tickets](#get-tickets)
  - [Delete Ticket](#delete-ticket)
  - [Get Available Tickets Count](#get-available-tickets-count)
  - [Purchase Ticket](#purchase-ticket)
- [Error Handling](#error-handling)
- [Candid Interface](#candid-interface)

## Dependencies
- `serde` for serialization and deserialization.
- `candid` for Candid serialization.
- `ic_cdk` for interacting with the Internet Computer.
- `ic_stable_structures` for stable storage of data.
- `sha2` for hashing passwords.

## Usage

### Register User 📝
Register a new user with the provided username, email, password, and role.

### Get User 🔍
Retrieve user details by user ID.

### Update User 🔄
Update user details by user ID.

### Delete User 🗑️
Delete user by user ID.

### Add Event 🎉
Add a new event with details like event name, details, location, start date, and end date.

### Update Event 📅
Update event details by event ID.

### Delete Event 🗑️
Delete event by event ID.

### Get Event 📅
Retrieve event details by event ID.

### Get Event By Name 📅
Retrieve event details by event name.

### Get Events 📅
Retrieve details of all events.

### Get Upcoming Events 📅
Retrieve details of upcoming events.

### Get Past Events 📅
Retrieve details of past events.

### Add Attendees 👥
Add attendees to an event.

### Get Attendees 👥
Retrieve list of attendees for an event.

### Generate Tickets 🎟️
Generate tickets for an event.

### Get Tickets 🎟️
Retrieve tickets for an event.

### Delete Ticket 🗑️
Delete ticket by ticket ID.

### Get Available Tickets Count 🎟️
Retrieve the number of available tickets for a specific event and ticket type.

### Purchase Ticket 💳
Purchase tickets for an event.

## Error Handling ❌
Errors are handled with specific error types:
- `NotFound`: Indicates that the requested resource was not found.
- `AlreadyExists`: Indicates that the resource being added already exists.
- `CustomError`: Indicates custom error messages.

## Candid Interface 🌐
This application exposes a Candid interface for interacting with the Internet Computer.


### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor

# Event Management System on the Internet Computer 🌐

The Event Management System is a decentralized application (dApp) deployed on the Internet Computer (IC), designed to facilitate the creation, management, and participation in events. It showcases the IC's capabilities for scalability, security, and persistence. This documentation provides a detailed overview of the system's architecture, functionalities, and core components.

## Core Dependencies 🛠️

- **Serde**: Used for serialization and deserialization of Rust data structures, facilitating efficient data storage and communication.
- **Candid**: A language for specifying interfaces on the IC, enabling seamless interaction between canisters and users or external systems.
- **IC CDK (Canister Development Kit)**: Provides tools and libraries for canister development, including stable storage access and inter-canister communication.
- **SHA-2**: Employs SHA-256 for hashing, enhancing security for sensitive information like user passwords.
- **Stable Storage Structures**: Custom data structures optimized for the IC's stable storage, ensuring data persistence across canister upgrades.

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
## Data Structures 📚

### Event
The core entity containing details such as event name, location, dates, and attendee information.

### User
Manages user-specific data, including credentials and roles, essential for authentication and authorization.

### Date
A simple structure for representing dates, facilitating event scheduling.

### TicketType & UserRole
Enums defining available ticket types and user roles, streamlining data validation and role-based access control.

### Attendee & Ticket
Structures for managing event participation and ticket transactions, integral to the event lifecycle.

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

## Functionalities 🌟

### User Management 👤

- **register_user**: Registers a new user, hashing the password for security.
- **get_user**: Retrieves a user's details by their unique ID.
- **update_user**: Allows users to update their profile information upon authentication.
- **delete_user**: Removes a user's profile and associated data, contingent on successful authentication.

### Event Management 📅

- **add_event**: Enables authorized users to create new events with comprehensive details.
- **update_event**: Allows for modifications to event details post-creation.
- **delete_event**: Supports the removal of events from the system.
- **get_event / get_event_by_name**: Facilitates access to event details by ID or name.

### Ticket Management 🎫

- **generate_tickets**: Manages the creation and allocation of tickets for events.
- **purchase_ticket**: Handles the purchase process, including ticket allocation and attendee registration.
- **delete_ticket**: Enables the cancellation of tickets and adjusts event capacities.
- **get_tickets**: Provides an overview of tickets for an event.


## Advanced Features and Error Handling 🔧

The system incorporates a custom `Error` enum for robust error handling, enhancing user experience by addressing potential issues proactively.

## Candid Interface 📡

Defines the publicly accessible functions of the canister, enabling interaction with external clients or other canisters on the IC network. This interface is crucial for the dApp's integration within the broader IC ecosystem.

## Conclusion 🏁

The Event Management System exemplifies the potential of decentralized applications on the Internet Computer, leveraging the platform's unique features to create a secure, scalable, and user-friendly solution for event management.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor

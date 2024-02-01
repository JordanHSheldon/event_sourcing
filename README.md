# event_sourcing
My attempt at building a cloud native simple event sourcing web server.


# Issues
- [ ] Find out how sql persists data.
- [ ] Find out how sql is so fast at storing data
- [ ] Find out what kind of data structures are best for loading event sourcing data into and out of.
- [ ] Find out how to keep certain data alive so it can be accessed at anytime.
- [ ] Find out how to encrypt/decrypt all of the data

# Event sourcing Research
- Events: 
    - Define the events that will be stored in the database. 
    - Events represent changes or state transitions in the system. 
    - Each event should capture all the information necessary to describe the change that occurred.

- Event Store: 
    - Create a data structure to store the events. This could be a database table, a log file, or a specialized storage system optimized for event sourcing.

- Aggregate Roots: 
    - Identify the aggregate roots in your domain model. 
    - An aggregate root is an entity that serves as the entry point to a group of related objects. 
    - All changes to the objects within the aggregate are made through the aggregate root.

- Event Handlers: 
    - Implement event handlers to process events and update the aggregate roots accordingly.
    - When a new event is received, the corresponding event handler is invoked to update the state of the affected aggregate root.

- Query Models: 
    - Optionally, create query models to efficiently query the data stored in the event store. 
    - Query models can be denormalized views optimized for specific read operations.

- Concurrency Control: 
    - Implement mechanisms to handle concurrency, such as optimistic locking or versioning, to ensure that concurrent modifications to the same aggregate are handled correctly.

- Event Replay and Projection: 
    - Provide mechanisms for event replay and projection to rebuild the current state of an aggregate root or generate query models from the event store.

- Infrastructure: 
    - Develop the necessary infrastructure components, such as APIs for interacting with the event store, data access layers, and integration with other parts of the system.

- Testing: 
    - Write tests to ensure the correctness and robustness of the event sourcing implementation, including tests for event handling, concurrency control, and rebuilding aggregate state from events.

- Deployment and Monitoring: 
    - Deploy the event sourcing database and monitoring tools to track system performance and ensure reliability.   



# Documentation
https://actix.rs/docs/getting-started/


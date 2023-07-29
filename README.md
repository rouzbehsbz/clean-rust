#Clean Rust
My attempt to implement clean architecture with Rust.

###Motivation
The main goal of clean architecture is separation of concerns, which brings high testability and maintainability to the software. This project aims to implement the principles of this architecture in a web application written in the Rust language with use of Actix framework.

###Run
You can simply run the project using docker, make sure you passed correct values to the `.env` file.
```
docker compose up
```

###Structure
- `application`: Handles top level business logics and data flow between different domain entites.
- `domain`: Core application logics as standalone entities.
- `infrastructure`: Contains all application dependencies like databases, caching, authentication service and ...
- `presentation`: Handles top layer data flow to the application using HTTP protocol.
- `config.rs`: Responsible for providing application configurations.
- `container.rs`: Contains services, repositories and all other application dependencies.

###Featurs
- Fully support of async operations for I/O bound tasks like HTTP request handling and database quering.
- Handling CPU intensive tasks in non-blocking manner.
- Fully testable application and domain layer.
- Mocking database interactions in tests using native in-memory mock service.
- Unit tests.
- Dcokerized.

###To Do List
- [ ] Add logger service
- [ ] Add integration tests
- [ ] Better error handling
- [ ] Better optimiztions
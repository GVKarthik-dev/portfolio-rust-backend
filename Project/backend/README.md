# Portfolio API - Actix Web Backend

A RESTful API backend built with Actix Web framework for managing portfolio projects and skills with authentication support.

## Features

- **Authentication**: User registration and login with JWT tokens
- **CRUD Operations**: Complete CRUD functionality for projects and skills
- **Error Handling**: Comprehensive JSON error responses
- **Logging**: Built-in logging with env_logger
- **Security**: Password hashing with bcrypt and JWT token validation

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Login and receive JWT token

### Projects
- `GET /api/v1/projects` - List all projects
- `GET /api/v1/projects/{id}` - Get a specific project
- `POST /api/v1/projects` - Create a new project
- `PUT /api/v1/projects/{id}` - Update a project
- `DELETE /api/v1/projects/{id}` - Delete a project

### Skills
- `GET /api/v1/skills` - List all skills
- `GET /api/v1/skills/{id}` - Get a specific skill
- `POST /api/v1/skills` - Create a new skill
- `PUT /api/v1/skills/{id}` - Update a skill
- `DELETE /api/v1/skills/{id}` - Delete a skill

## Getting Started

### Prerequisites
- Rust 1.56 or higher
- Cargo

### Installation

1. Navigate to the backend directory:
```bash
cd backend
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## Project Structure

```
backend/
├── src/
│   ├── main.rs           - Application entry point
│   ├── models.rs         - Data models
│   ├── errors.rs         - Error handling
│   ├── auth.rs           - Authentication utilities
│   ├── db.rs             - In-memory database
│   └── handlers/         - Request handlers
│       ├── mod.rs
│       ├── auth.rs
│       ├── projects.rs
│       └── skills.rs
├── Cargo.toml            - Dependencies
└── README.md             - This file
```

## Development

### Running tests:
```bash
cargo test
```

### Building for production:
```bash
cargo build --release
```

## Dependencies

- **actix-web** - Web framework
- **serde/serde_json** - Serialization
- **jsonwebtoken** - JWT authentication
- **bcrypt** - Password hashing
- **uuid** - ID generation
- **chrono** - Date/time handling

## License

MIT

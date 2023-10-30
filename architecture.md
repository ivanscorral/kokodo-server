# Todo Items API Server in Rust

Generated by GPT-4. All content below is subject to change without notice.

## Overview

This is a monolithic, CRUD-centric API server for managing todo items. The project is written in Rust and aims to provide a simple yet effective way to manage a todo list via HTTP API calls.

## Architecture

The architecture is monolithic, with a focus on CRUD operations. The project will have a modular design to separate concerns like HTTP routing, business logic, and database access.

### Components

#### Main Application

- `main.rs`: The entry point for the API server. Initializes routing, middleware, and starts the server.

#### HTTP Routing

- `routes.rs`: Defines all the API routes and maps them to their respective handlers.

#### Business Logic

- `controllers/`
  - `todo_controller.rs`: Handles business logic related to todo items.
  - `auth_controller.rs`: Handles user authentication and authorization.

#### Database Access

- `models/`
  - `todo_model.rs`: Defines the data model and related database operations for todo items.
  - `user_model.rs`: Defines the data model and related database operations for users.

#### Utilities

- `utils/`
  - `error.rs`: Custom error types and error handling utilities.
  - `constants.rs`: Holds any constants that are used across the application.

#### Authentication

- `auth/`
  - `jwt.rs`: Functions related to JWT token generation and validation.
  - `middleware.rs`: Middleware for handling user authentication.

### Dependencies

- Web Framework: Actix-Web
- Database: SQLite (for simplicity and ease of setup)
- ORM: Diesel (SQLite support)
- Authentication: JSON Web Tokens (JWT)

## Features

### User Authentication

- Register
- Login
- JWT Token generation and validation

### Todo Item Management

- Create todo items
- Read todo items
- Update todo items
- Delete todo items

### Pagination and Limits

- Implement pagination for the todo list
- Limit the number of todo items returned per request

## Future Enhancements

- Add support for categories or tags
- Implement search functionality

## Local Development

Instructions for setting up local development.

### Database Setup

- SQLite setup instructions

### Running the API Server

- Instructions to build and run the project locally.

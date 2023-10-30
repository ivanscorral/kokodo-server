
# Database Design for Todo API Server in Rust

## Overview

This document outlines the database schemas and relationships for the Todo API Server implemented in Rust. The design accounts for future scalability, including the addition of tags for todo items.

## Tables

### Users Table

#### Schema (Users)

| Column         | Type      | Constraints     |
|----------------|-----------|-----------------|
| id             | INTEGER   | PRIMARY KEY, AUTOINCREMENT |
| username       | TEXT      | UNIQUE, NOT NULL |
| email          | TEXT      | UNIQUE, NOT NULL |
| password_hash  | TEXT      | NOT NULL        |

#### SQL Definition (Users)

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
```

---

### Todos Table

#### Schema (Todos)

| Column       | Type      | Constraints                  |
|--------------|-----------|------------------------------|
| id           | INTEGER   | PRIMARY KEY, AUTOINCREMENT   |
| title        | TEXT      | NOT NULL                     |
| description  | TEXT      | NULLABLE                     |
| completed    | BOOLEAN   | NOT NULL, DEFAULT 0          |
| user_id      | INTEGER   | FOREIGN KEY REFERENCES users(id) |

#### SQL Definition (Todos)

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    user_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

### Tags Table (Future Expansion)

#### Schema (Tags)

| Column       | Type      | Constraints                |
|--------------|-----------|----------------------------|
| id           | INTEGER   | PRIMARY KEY, AUTOINCREMENT |
| name         | TEXT      | UNIQUE, NOT NULL           |

#### SQL Definition (Tags)

```sql
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);
```

---

### Todo_Tags Table (Future Expansion)

#### Schema (Todo_Tags)

| Column       | Type      | Constraints                      |
|--------------|-----------|----------------------------------|
| todo_id      | INTEGER   | FOREIGN KEY REFERENCES todos(id) |
| tag_id       | INTEGER   | FOREIGN KEY REFERENCES tags(id)  |

#### SQL Definition

```sql
CREATE TABLE todo_tags (
    todo_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (todo_id, tag_id),
    FOREIGN KEY (todo_id) REFERENCES todos(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```

## Entity-Relationship Diagram (ERD)

```
  +--------+    +--------+    +-------+    +-----------+
  |  users |----|  todos |----|  tags |----|  todo_tags |
  +--------+    +--------+    +-------+    +-----------+
  |   id   |    |   id   |    |  id   |    |  todo_id   |
  |  ...   |    |  ...   |    | name  |    |   tag_id   |
  +--------+    +--------+    +-------+    +-----------+
```

- A user can have multiple todos.
- A todo belongs to a single user.
- A todo can have multiple tags.
- A tag can be associated with multiple todos.

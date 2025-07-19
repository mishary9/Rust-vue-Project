# Rust & Vue 3 RESTful API

A full-stack user authentication system built with a **Rust (Axum)** backend, **MySQL** database, and a **Vue 3** frontend using **Vite**.

## Features

- User registration
- User login
- RESTful API using Rust and Axum
- SPA frontend using Vue 3
- MySQL as the relational database

---

## Getting Started

### 1. Clone the Repository

```bash
git clone <your-repository-url>
cd <your-project-directory>

``` 
### 2. Configure Database
Navigate to your backend project folder and edit the .env file:
```env
DATABASE_URL="mysql://username:password@hostname:port/database_name"
```

Replace DATABASE_URL with your actual one.


### 3. Set Up the Users Table
Make sure your MySQL database contains a users table. You can create it with the following SQL query:
```sql
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 4. Run the Rust Server

```bash
cargo run
``` 
The API will be available at:
http://localhost:3000

### 5. Navigate to the Frontend Folder

```bash
cd frontend
``` 
### 6. Install Dependencies
Run the following command in order
```bash
npm install
npm install vue-router@4
``` 
### 7. Start the Development Server
```bash
npm run dev
```
The frontend will be available at the URL shown in the console, typically:
http://localhost:5173




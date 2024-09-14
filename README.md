# Printer Supplies API 🖨️

A **REST API** using **Axum** for managing **printer supplies**, such as toners and drums. The project includes CRUD functionalities for various resources and inventory movement tracking.

## Technologies Used

- **Axum:** Asynchronous web framework for Rust.
- **SQLx:** For interaction with the database and migrations.
- **Just:** For utility scripts.
- **Docker:** For database container.
- **Postman:** API documentation.

## Features

- CRUD operations for printers, brands, drums, and toners.
- Inventory management for toners and drums.
- Docker Compose setup for easy deployment.
- Database migrations included.
- API documentation available in Postman.

---

# Getting Started 🎯
## Prerequisites:

- **Rust** *(latest stable version)*
- **Docker** and **Docker Compose**
- **Just** for scripts

## 1. Installation

``` bash
git clone https://github.com/allansomensi/printer-supplies-api.git
cd printer-supplies-api
```
For the scripts:

``` elixir
cargo install just
```

## 2. Build and run the Docker container:

``` elixir
just services-up
```

## 3. Run migrations:

``` elixir
just migrate-run
```

## 4. Start server 🚀 🚀 

``` elixir
just dev
```

# Running Tests 👨‍🔬

For once:
``` elixir
just test
```

For watching mode:
``` elixir
just test-watch
```

## Endpoint tests 🧪

> Endpoint tests are located at `/tests` folder

![Test Status](https://github.com/user-attachments/assets/fff6aff7-45d7-4801-8d5b-7b7768853ee1)

## API Documentation 📚

API endpoints and usage details are documented in **Postman**, you can import the `postman_endpoints.json` file.

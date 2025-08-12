# Nexantara API

A high-performance RESTful API for Indonesian administrative divisions (Provinces, Regencies, Districts, and Villages) built with Rust and Actix-web.

## Features

- 🚀 **High Performance**: Built with Rust and Actix-web for maximum performance
- 📊 **Complete Data**: Covers all provinces, regencies, districts, and villages in Indonesia
- 🔍 **Easy Querying**: Simple and intuitive API endpoints for all administrative levels
- 🔒 **Secure**: API key authentication for protected endpoints
- 🗺️ **Postal Codes**: Coming soon...

## Tech Stack

- **Framework**: Actix-web
- **Database**: PostgreSQL
- **ORM**: SQLx (async)
- **Authentication**: API Key
- **Containerization**: Docker (optional)

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (v12+)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/arjunstein/nexantara-api.git
   cd nexantara-api
   ```

2. Create a `.env` file in the root directory with the following variables:

   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/db
   API_KEY=your_secure_api_key_here
   RUST_LOG=debug
   ```

3. Run database migrations:

   ```bash
   sqlx migrate run
   ```

4. Start the server:

   ```bash
   cargo run --release
   ```

   The API will be available at `http://localhost:8080`

## API Endpoints

### Provinces

- `GET /api/v1/provinces` - Get all provinces

### Regencies

- `GET /api/v1/provinces/{province_id}/regencies` - Get regencies by province ID

### Districts

- `GET /api/v1/regencies/{regency_id}/districts` - Get districts by regency ID

### Villages

- `GET /api/v1/districts/{district_id}/villages` - Get villages by district ID

### Authentication

All endpoints require an API key in the `X-API-Key` header.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Data sourced from official Indonesian government sources
- Built with ❤️ using Rust

# Quick Start Guide

Get BetterNewTab running locally in 5 minutes.

## Prerequisites

- **Bun** installed ([bun.sh](https://bun.sh))
- **Rust** installed ([rustup.rs](https://rustup.rs))
- **Docker** installed ([docker.com](https://docker.com))

## 1. Start the Database

```bash
# From project root
docker-compose up -d

# Wait for PostgreSQL to be ready (about 10 seconds)
```

The database will automatically run migrations on first start.

## 2. Start the Backend

```bash
cd server

# Copy environment file
cp .env.example .env

# Edit .env and set JWT_SECRET to any random string
# Example: JWT_SECRET=my-super-secret-key-12345

# Run the server
cargo run
```

Server will start on **http://localhost:3000**

## 3. Start the Frontend

```bash
cd client

# Install dependencies
bun install

# Copy environment file
cp .env.example .env

# No changes needed for local development

# Run the dev server
bun dev
```

Frontend will start on **http://localhost:5173**

## 4. Test It Out

### Register a New User

```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"testpass123"}'
```

You'll receive a JWT token in the response.

### Login

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"testpass123"}'
```

### Test Authenticated Endpoint

```bash
# Use the token from login response
TOKEN="your-jwt-token-here"

curl http://localhost:3000/user_data \
  -H "Authorization: Bearer $TOKEN" \
  -H "X-User-Id: your-user-id" \
  -H "X-User-Email: test@example.com"
```

## What's Next?

### Frontend Auth UI Needed

The backend is fully functional, but the frontend still has old Clerk authentication UI. You'll need to:

1. **Create Login/Register forms** that call your new endpoints
2. **Update the user store** to handle JWT tokens
3. **Remove Clerk references** from components

See `MIGRATION_SUMMARY.md` for details on what needs updating.

### Recommended Next Steps

1. **Set up Stripe** (for payments)
   - Get API keys from [dashboard.stripe.com](https://dashboard.stripe.com)
   - Add to `server/.env`

2. **Configure Brave API** (for search suggestions)
   - Get API key from [brave.com/search/api](https://brave.com/search/api)
   - Add to `server/.env`

3. **Set up Sentry** (for error tracking)
   - Create project at [sentry.io](https://sentry.io)
   - Add DSN to code (currently hardcoded)

## Troubleshooting

### Database won't start
```bash
# Check if port 5432 is in use
lsof -i :5432

# Stop docker-compose and start again
docker-compose down
docker-compose up -d
```

### Backend won't compile
```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build
```

### Frontend won't start
```bash
# Clear cache and reinstall
rm -rf node_modules
bun install
```

### Migrations didn't run
```bash
# Run manually
docker-compose exec postgres psql -U postgres -d betternewtab -f /docker-entrypoint-initdb.d/001_initial_schema.sql
docker-compose exec postgres psql -U postgres -d betternewtab -f /docker-entrypoint-initdb.d/002_functions.sql
```

## Development Workflow

### Backend Changes

```bash
cd server

# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Check for errors without building
cargo check
```

### Frontend Changes

```bash
cd client

# Dev server with hot reload
bun dev

# Type checking in watch mode
bun type-check --watch

# Run tests
bun test:unit
```

### Database Changes

```bash
# Connect to database
docker-compose exec postgres psql -U postgres -d betternewtab

# Useful commands:
\dt          # List tables
\d users     # Describe users table
SELECT * FROM users;
```

## Stop Everything

```bash
# Stop backend: Ctrl+C

# Stop frontend: Ctrl+C

# Stop database:
docker-compose down

# Stop and remove data (CAUTION):
docker-compose down -v
```

## Need Help?

- 📖 Full documentation: `README.md`
- 🚀 Deployment guide: `DEPLOYMENT.md`
- 📝 Migration details: `MIGRATION_SUMMARY.md`
- 🔧 AGENTS.md for coding conventions

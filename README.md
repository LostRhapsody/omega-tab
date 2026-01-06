# BetterNewTab

A customizable browser new tab replacement that helps you organize your most-used links, search the web, and boost productivity.

## Tech Stack

### Frontend
- **Vue.js 3** with Composition API and TypeScript
- **Vuetify 3** for UI components
- **Pinia** for state management
- **Vite** for build tooling
- **Tailwind CSS** for styling

### Backend
- **Rust** with Axum web framework
- **PostgreSQL** for data persistence
- **SQLx** for type-safe database queries
- **JWT** for authentication
- **Stripe** for payments
- **Sentry** for error tracking

## Features

- **Custom Links**: Organize bookmarks in categorized columns
- **Search Bar**: Quick search with fuzzy matching and multiple search engines
- **Auto-suggestions**: Get search suggestions as you type (premium)
- **Metadata Fetching**: Automatically fetch favicons, titles, and descriptions
- **Keyboard Shortcuts**: Navigate links with Ctrl+1-9, open command palette with Ctrl+K
- **Subscription Plans**: Free, Plus, and Pro tiers with Stripe integration
- **Integrations**: Jira, Confluence, and Linear API integration (premium)

## Prerequisites

- **Bun** v1.0+ (for frontend)
- **Rust** 1.70+ (for backend)
- **PostgreSQL** 16+ (or Docker)
- **Node.js** 18+ (optional, for some tooling)

## Getting Started

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd betternewtab
```

### 2. Database Setup

#### Option A: Using Docker (Recommended)

```bash
# Start PostgreSQL with Docker Compose
docker-compose up -d

# The database will be available at localhost:5432
# Database: betternewtab
# User: postgres
# Password: postgres
```

#### Option B: Local PostgreSQL

Install PostgreSQL 16+ and create a database:

```sql
CREATE DATABASE betternewtab;
```

### 3. Run Database Migrations

```bash
cd server

# Set the DATABASE_URL environment variable
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/betternewtab"

# Run migrations using psql
psql $DATABASE_URL -f migrations/001_initial_schema.sql
psql $DATABASE_URL -f migrations/002_functions.sql
```

### 4. Backend Setup

```bash
cd server

# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# Required variables:
# - DATABASE_URL
# - JWT_SECRET
# - STRIPE_SECRET_KEY (get from Stripe Dashboard)
# - FREE_PLAN_ID (from database after seeding)

# Install dependencies and run
cargo build
cargo run
```

The server will start on `http://localhost:3000`.

### 5. Frontend Setup

```bash
cd client

# Install dependencies
bun install

# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# Required variables:
# - VITE_API_BASE_URL=http://localhost:3000

# Run development server
bun dev
```

The frontend will be available at `http://localhost:5173`.

## Development

### Frontend Commands

```bash
cd client

# Development server with hot reload
bun dev

# Development server + Tailwind watcher
bun dev:all

# Lint + dev server + Tailwind watcher
bun start

# Type checking
bun type-check

# Build for production
bun build

# Run tests
bun test:unit         # Watch mode
bun test:run          # Single run
bun test:e2e          # Cypress E2E tests
```

### Backend Commands

```bash
cd server

# Development server (with auto-reload using cargo-watch)
cargo watch -x run

# Build for production
cargo build --release

# Run tests
cargo test

# Check for errors without building
cargo check

# Linting
cargo clippy
```

## Project Structure

```
betternewtab/
├── client/                 # Vue.js frontend
│   ├── src/
│   │   ├── assets/        # CSS, images
│   │   ├── components/    # Vue components
│   │   ├── composables/   # Vue composables
│   │   ├── constants/     # API endpoints, config
│   │   ├── router/        # Vue Router
│   │   ├── services/      # API service layer
│   │   ├── stores/        # Pinia stores
│   │   ├── types/         # TypeScript types
│   │   ├── utils/         # Utility functions
│   │   └── views/         # Page components
│   ├── package.json
│   └── vite.config.ts
├── server/                 # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point & handlers
│   │   ├── database.rs    # Database client
│   │   ├── middleware.rs  # Auth middleware
│   │   ├── user_jwt.rs    # JWT utilities
│   │   ├── stripe_client.rs
│   │   ├── brave.rs       # Search API
│   │   └── resend.rs      # Email service
│   ├── migrations/        # SQL migration files
│   └── Cargo.toml
├── docker-compose.yml      # PostgreSQL setup
└── README.md
```

## Environment Variables

### Backend (.env)

```bash
# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/betternewtab

# Authentication
JWT_SECRET=your-super-secret-jwt-key-change-this

# Stripe (get from https://dashboard.stripe.com/apikeys)
STRIPE_SECRET_KEY=sk_test_...
STRIPE_ENDPOINT_SECRET=whsec_...
STRIPE_VERIFY_WEBHOOK_SIGNATURE=true

# Plans
FREE_PLAN_ID=a0b1c2d3-e4f5-6789-abcd-ef0123456789

# External Services
BRAVE_SUGGEST_URL=https://api.search.brave.com/res/v1/suggest/search
BRAVE_API_KEY=your-brave-api-key
CUSTOMER_SUPPORT_EMAIL=support@example.com

# Environment
ENVIRONMENT=development
DOMAIN=localhost
```

### Frontend (.env)

```bash
# API
VITE_API_BASE_URL=http://localhost:3000

# Stripe URLs
VITE_PLUS_PLAN_URL=https://buy.stripe.com/...
VITE_PRO_PLAN_URL=https://buy.stripe.com/...
VITE_TEAM_PLAN_URL=https://buy.stripe.com/...
VITE_STRIPE_MANAGE_URL=https://billing.stripe.com/...

# Features
VITE_AUTO_SUGGEST_ON=true
VITE_MAX_HISTORY_ENTRIES=10
```

## Deployment

### Railway Deployment

This project is configured for Railway deployment with PostgreSQL.

1. **Create a new project on Railway**
2. **Add PostgreSQL database**:
   - Railway will provide a `DATABASE_URL`
   - Run migrations manually after deployment

3. **Deploy Backend**:
   ```bash
   # Railway will detect the Rust server and build it automatically
   # Set environment variables in Railway dashboard
   ```

4. **Deploy Frontend**:
   ```bash
   # Build the frontend
   cd client
   bun build

   # The build output will be in client/dist
   # Serve with a static file server or add to your backend
   ```

5. **Run Migrations**:
   ```bash
   # Connect to Railway PostgreSQL
   railway run psql $DATABASE_URL -f server/migrations/001_initial_schema.sql
   railway run psql $DATABASE_URL -f server/migrations/002_functions.sql
   ```

## API Endpoints

### Authentication
- `POST /register` - Create a new user account
- `POST /login` - Login and receive JWT token
- `GET /health` - Health check endpoint

### Users
- `GET /user` - Get current user
- `GET /user_data` - Get user with all related data

### Links
- `GET /user/links` - Get all user links
- `POST /link` - Create a new link
- `PUT /link` - Update a link
- `DELETE /link/:id` - Delete a link

### Subscriptions
- `GET /confirm` - Confirm subscription status
- `POST /cancel` - Cancel subscription
- `GET /plan/:id` - Get plan details

### Settings
- `GET /settings` - Get user settings
- `POST /settings` - Create user settings
- `PUT /settings` - Update user settings

### Other
- `GET /suggest/:query` - Get search suggestions
- `POST /feedback` - Submit feedback

## Database Schema

### Tables
- **users** - User accounts with password hashes
- **links** - User bookmarks
- **plans** - Subscription tiers
- **subscriptions** - User/organization subscriptions
- **user_memberships** - Team memberships (future feature)
- **user_settings** - User preferences
- **organizations** - Workspaces (future feature)
- **teams** - Team groups (future feature)
- **feedback_timestamps** - Rate limiting for feedback

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

[Add your license here]

## Support

For issues and questions, please create an issue in the GitHub repository.


# Things the AI told me

I need to read through this later. Just in case.

Migration Complete! Backend Successfully Compiles
I've successfully fixed all the compilation errors in the Rust backend. Here's what was done:
Issues Fixed
1. Replaced all supabase references with database throughout the codebase:
  - Variable names (local supabase → database)
  - Type references (supabase::User → database::User)
  - Method calls on wrong variables
2. Fixed User struct initializations - Added password_hash field to all User struct creations (3 locations)
3. Fixed create_subscription call - Updated to use individual parameters instead of a Subscription object
4. Fixed type annotation in database.rs - Added explicit type for password_hash field retrieval
Compilation Status
✅ cargo check - SUCCESS
Finished dev profile [unoptimized + debuginfo] target(s) in 6.97s
Next Steps
1. Set up local environment:
  # Start PostgreSQL
  docker-compose up -d
  
  # Create .env files
  cp server/.env.example server/.env
  cp client/.env.example client/.env
  
  # Run migrations
  psql  -f server/migrations/001_initial_schema.sql
  psql  -f server/migrations/002_functions.sql
  
2. Test the backend:
  cd server
  cargo run
  
  # In another terminal:
  curl -X POST http://localhost:3000/register \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com","password":"test123"}'
  
3. Update frontend (see MIGRATION_SUMMARY.md for details):
  - Remove Clerk references
  - Implement login/register UI
  - Update auth flow to use JWT tokens
All documentation has been created:
- ✅ README.md - Complete project documentation
- ✅ DEPLOYMENT.md - Railway deployment guide
- ✅ MIGRATION_SUMMARY.md - Detailed change log
- ✅ QUICKSTART.md - Fast local setup
- ✅ COMPILE_STATUS.md - Current status and next steps
The backend is ready to run! 🚀

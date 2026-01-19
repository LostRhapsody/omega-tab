# OmegaTab

A customizable new tab replacement for your browser that helps you organize your most-used links, search the web, and boost productivity.

## Landing Page

[Click here to check out our official landing page](https://omega-tab.evanrobertson.dev), which has some screenshots and descriptions of features.

## Documenation

[Click here to visit the official guides](https://omega-tab.evanrobertson.dev/docs/).

## Getting Started

A "proper" deployment system is in the works, as Omega Tab can be run as a single executable, so installing and starting up *should* be easy. Right now, there are a few steps. You'll need to clone the repository, have bun and rust installed, then install everything, instructions are below!

There are also instructions on how to set Omega Tab as your new tab page in a few different browsers.

## Tech Stack

Omega Tab is built to be lean and fast. A Vue.js is compiled into static assets and served via a *blazing fast* rust backend. The Landing Page and Docs are also Vue.js and served via a lightweight docker container running Nginx.

Landing Page and Docs are hosted by me at [omega-tab](https://omega-tab.evanrobertson.dev) and [omega-tab-docs](https://omega-tab.evanrobertson.dev/docs), so you don't need to run them locally if you need either of them.

## Features

- **Custom Links**: Organize bookmarks in categorized columns
- **Search Bar**: Quick search with fuzzy matching and multiple search engines
- **Auto-suggestions**: Get search suggestions as you type (premium)
- **Metadata Fetching**: Automatically fetch favicons, titles, and descriptions
- **Keyboard Shortcuts**: Navigate links with Ctrl+1-9, open command palette with Ctrl+K

## Setting as New Tab Page

After setting up OmegaTab locally, you can configure your browser to use it as the new tab page.

### Chrome / Edge / Helium

Chromium browsers don't allow custom new tab URLs directly, so you'll need an extension:

1. Install the [Custom New Tab URL](https://chrome.google.com/webstore/detail/custom-new-tab-url/) extension
2. Click the extension icon and set the URL to `http://localhost:5173`
3. (Optional) Set as homepage: Settings > On startup > Open a specific page

### Brave

1. Open Settings, Get Started
2. On Startup -> Open the New Tab page
3. New Tab Page Shows -> Homepage
4. Appearance -> Show home button (true)
5. Enter custom web address (http://localhost:5173)

### Firefox

1. Install the [New Tab Override](https://addons.mozilla.org/en-US/firefox/addon/new-tab-override/) add-on
2. In the add-on settings, set the custom URL to `http://localhost:5173`
3. (Optional) Set as homepage: Settings > Home > Homepage and new windows

### Safari

Safari doesn't support custom new tab pages without third-party tools. Consider:
1. Set as homepage: Safari > Settings > General > Homepage
2. Use a keyboard shortcut (Cmd+Shift+H) to open homepage in new tabs

### Arc

1. Open Settings (Cmd+,)
2. Go to "Links" tab
3. Under "New Tab", select "Custom URL"
4. Enter `http://localhost:5173`

## Prerequisites

- **Bun** v1.0+ (for frontend)
- **Rust** 1.70+ (for backend)
- **Node.js** 18+ (optional, for some tooling)

## Getting Started

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd omega-tab
```

### 2. Backend Setup

```bash
cd server

# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# Required variable:
# - JWT_SECRET (any random string for signing tokens)

# Build and run
cargo build
cargo run
```

The server will start on `http://localhost:3000`. The SQLite database is created automatically at:
- Windows: `%APPDATA%/OmegaTab/omega-tab.db`
- macOS: `~/Library/Application Support/OmegaTab/omega-tab.db`
- Linux: `~/.local/share/OmegaTab/omega-tab.db`

### 3. Client App Setup

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
bun dev:all
```

The client app will be available at `http://localhost:5173`.

### 4. Landing Page Setup (Optional)

```bash
cd landing-page

# Install dependencies
bun install

# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# - VITE_API_BASE_URL=http://localhost:3000
# - VITE_APP_URL=http://localhost:5173

# Build Tailwind and run development server
bun tailwind:build && bun dev
```

The landing page will be available at `http://localhost:5175`.

### 5. Documentation Setup (Optional)

```bash
cd docs

# Install dependencies
bun install

# Run development server
bun dev
```

The documentation will be available at `http://localhost:5174`.

## Development

### Client App Commands

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

### Landing Page Commands

```bash
cd landing-page

# Development server
bun dev

# Development server + Tailwind watcher
bun dev:all

# Type checking
bun type-check

# Build for production
bun build
```

### Documentation Commands

```bash
cd docs

# Development server
bun dev

# Build for production
bun build

# Preview production build
bun preview
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
omega-tab/
├── client/                 # Vue.js main app (authenticated users)
│   ├── src/
│   │   ├── assets/        # CSS, images
│   │   ├── components/    # Vue components
│   │   ├── composables/   # Vue composables
│   │   ├── constants/     # API endpoints, config
│   │   ├── router/        # Vue Router (with auth guards)
│   │   ├── services/      # API service layer
│   │   ├── stores/        # Pinia stores
│   │   ├── types/         # TypeScript types
│   │   ├── utils/         # Utility functions
│   │   └── views/         # Page components (Home, Settings, Login)
│   ├── package.json
│   └── vite.config.ts
├── landing-page/           # Vue.js marketing site (standalone)
│   ├── src/
│   │   ├── assets/        # CSS, images
│   │   ├── components/    # Header, Footer, AuthModal
│   │   ├── data/          # Pricing plans
│   │   ├── router/        # Vue Router
│   │   ├── services/      # Auth service
│   │   ├── types/         # TypeScript types
│   │   └── views/         # LandingPage, Contact, Privacy, Terms
│   ├── public/copy/       # Marketing images
│   ├── package.json
│   └── vite.config.ts
├── docs/                   # VitePress documentation
│   ├── .vitepress/        # VitePress config
│   ├── guides/            # Guide markdown files
│   ├── index.md           # Docs home
│   └── package.json
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
DATABASE_URL=postgres://postgres:postgres@localhost:5432/omega-tab

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

### Client App (.env)

```bash
# API
VITE_API_BASE_URL=http://localhost:3000

# Landing page URL (for signup links)
VITE_LANDING_URL=http://localhost:5175

# Stripe URLs
VITE_PLUS_PLAN_URL=https://buy.stripe.com/...
VITE_PRO_PLAN_URL=https://buy.stripe.com/...
VITE_TEAM_PLAN_URL=https://buy.stripe.com/...
VITE_STRIPE_MANAGE_URL=https://billing.stripe.com/...

# Features
VITE_AUTO_SUGGEST_ON=true
VITE_MAX_HISTORY_ENTRIES=10
```

### Landing Page (.env)

```bash
# API
VITE_API_BASE_URL=http://localhost:3000

# App URL (redirect after login/signup)
VITE_APP_URL=http://localhost:5173

# Docs URL
VITE_DOCS_URL=http://localhost:5174

# Stripe URLs
VITE_PLUS_PLAN_URL=https://buy.stripe.com/...
VITE_PRO_PLAN_URL=https://buy.stripe.com/...
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

4. **Deploy Client App**:
   ```bash
   cd client
   bun build
   # The build output will be in client/dist
   # Deploy to app.yourdomain.com
   ```

5. **Deploy Landing Page**:
   ```bash
   cd landing-page
   bun build
   # The build output will be in landing-page/dist
   # Deploy to yourdomain.com
   ```

6. **Deploy Documentation**:
   ```bash
   cd docs
   bun build
   # The build output will be in docs/.vitepress/dist
   # Deploy to docs.yourdomain.com
   ```

7. **Run Migrations**:
   ```bash
   # Connect to Railway PostgreSQL
   railway run psql $DATABASE_URL -f server/migrations/001_initial_schema.sql
   railway run psql $DATABASE_URL -f server/migrations/002_functions.sql
   ```

### Domain Structure (Recommended)

- `yourdomain.com` - Landing page
- `app.yourdomain.com` - Client app
- `docs.yourdomain.com` - Documentation
- `api.yourdomain.com` - Backend API

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

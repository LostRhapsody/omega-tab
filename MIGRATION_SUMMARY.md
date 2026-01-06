# Migration Summary: Supabase → PostgreSQL

This document summarizes all changes made during the migration from Supabase to standard PostgreSQL with custom authentication.

## Overview

- **Replaced**: Supabase (managed PostgreSQL + Auth) → Standard PostgreSQL + Custom JWT Auth
- **Removed**: Clerk authentication → Custom email/password authentication
- **Fixed**: Multiple security and performance issues
- **Added**: Railway deployment configuration

## Major Changes

### Backend (Rust Server)

#### 1. Database Layer (`server/src/database.rs`)
- **Renamed**: `supabase.rs` → `database.rs`
- **Renamed**: `Supabase` struct → `Database` struct
- **Added**: Password hashing with bcrypt
- **Added**: `register_user()` - Create new users with hashed passwords
- **Added**: `verify_password()` - Authenticate users
- **Updated**: `User` struct now includes `password_hash` field
- **Updated**: `create_user()` to include password_hash

#### 2. Authentication (`server/src/middleware.rs`)
- **Removed**: Clerk JWT validation (JWKS fetching, RS256 verification)
- **Replaced with**: Simple JWT validation using our own tokens
- **Simplified**: No more external API calls for every request
- **Added**: Support for public routes (`/login`, `/register`, `/health`)

#### 3. API Endpoints (`server/src/main.rs`)
- **Removed**: Hardcoded test credentials (security fix)
- **Added**: `POST /register` - User registration endpoint
- **Added**: `POST /login` - User login endpoint
- **Added**: `GET /health` - Health check endpoint
- **Updated**: `DELETE /link/:id` - Now validates link ownership (security fix)
- **Updated**: All references from `supabase` → `database`
- **Updated**: Environment variable `POSTGRES_URL` → `DATABASE_URL`

#### 4. Dependencies (`server/Cargo.toml`)
- **Added**: `bcrypt = "0.15"` for password hashing

#### 5. Security Fixes
- ✅ Removed hardcoded credentials from source code
- ✅ Added ownership validation for link deletion
- ✅ Simplified auth flow (no external JWKS fetching)
- ✅ Password hashing with industry-standard bcrypt

### Frontend (Vue.js Client)

#### 1. Dependencies (`client/package.json`)
- **Removed**: `@clerk/clerk-js`
- **Removed**: `@supabase/supabase-js`

#### 2. Files Deleted
- `client/src/utils/supabase.ts` - Supabase client initialization
- `supabase/` directory - Supabase local development configuration

#### 3. Environment Variables
- **Removed**: `VITE_SUPABASE_URL`
- **Removed**: `VITE_SUPABASE_ANON_KEY`
- **Removed**: `VITE_CLERK_PUBLISHABLE_KEY`

> **Note**: The frontend components still reference Clerk in several places. You will need to update these components to use the new `/login` and `/register` endpoints instead. This was intentionally left for you to implement as it requires UI/UX decisions.

### Database

#### 1. Schema Changes (`server/migrations/001_initial_schema.sql`)
- **Updated**: `users` table now includes `password_hash TEXT NOT NULL`
- **Added**: Indexes for performance
- **Added**: Foreign key constraints
- **Added**: Default free plan seed data

#### 2. Functions (`server/migrations/002_functions.sql`)
- **Preserved**: `create_organization()` function
- **Preserved**: `create_team()` function

### Infrastructure

#### 1. Local Development
- **Added**: `docker-compose.yml` - One-command PostgreSQL setup
- **Added**: `server/.env.example` - Backend environment template
- **Added**: `client/.env.example` - Frontend environment template

#### 2. Deployment
- **Added**: `server/Dockerfile` - Multi-stage Rust build
- **Added**: `railway.toml` - Railway deployment configuration
- **Added**: `.dockerignore` - Optimize build context
- **Added**: `DEPLOYMENT.md` - Detailed deployment guide

#### 3. Documentation
- **Rewrote**: `README.md` - Complete project documentation
- **Added**: `DEPLOYMENT.md` - Railway deployment guide
- **Added**: `MIGRATION_SUMMARY.md` - This document

## Environment Variables Changes

### Backend

| Old Variable | New Variable | Notes |
|--------------|--------------|-------|
| `POSTGRES_URL` | `DATABASE_URL` | Renamed for consistency |
| `CLERK_JWKS_URL` | ❌ Removed | No longer using Clerk |
| `CLERK_API_KEY` | ❌ Removed | No longer using Clerk |
| N/A | `JWT_SECRET` | ✅ New - for signing our own JWTs |

### Frontend

| Old Variable | New Variable | Notes |
|--------------|--------------|-------|
| `VITE_SUPABASE_URL` | ❌ Removed | No longer using Supabase |
| `VITE_SUPABASE_ANON_KEY` | ❌ Removed | No longer using Supabase |
| `VITE_CLERK_PUBLISHABLE_KEY` | ❌ Removed | No longer using Clerk |

## Authentication Flow Changes

### Before (Clerk)
```
1. User signs in via Clerk UI
2. Clerk returns JWT (RS256)
3. Frontend stores Clerk JWT
4. Each API request: Frontend sends Clerk JWT
5. Backend fetches JWKS from Clerk API (every request!)
6. Backend validates JWT signature
7. Backend generates internal JWT for user
```

### After (Custom)
```
1. User submits email/password to /register or /login
2. Backend hashes password (bcrypt) and verifies
3. Backend generates JWT (HS256 with our secret)
4. Frontend stores JWT
5. Each API request: Frontend sends JWT
6. Backend validates JWT locally (no external calls)
```

**Benefits:**
- ✅ Faster (no external API calls)
- ✅ Simpler codebase
- ✅ No third-party dependencies
- ✅ Full control over auth logic
- ✅ Lower latency

## Database Schema

No structural changes to tables, only the `users` table gained a `password_hash` column:

```sql
-- Before (Supabase)
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- After (PostgreSQL)
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

All other tables remain unchanged.

## Breaking Changes & Migration Path

### For Existing Users

If you have existing users in the database:

1. **Option A: Force password reset**
   - Existing users cannot login (no password_hash)
   - Add a password reset flow
   - Send password reset emails

2. **Option B: Migration script**
   ```sql
   -- Set temporary passwords for existing users
   UPDATE users 
   SET password_hash = '$2b$12$...' -- bcrypt hash of "ChangeMe123!"
   WHERE password_hash IS NULL;
   ```
   - Email all users to change their password

3. **Option C: Clean slate**
   - Delete all existing users
   - Fresh start with new auth system

### For Frontend Components

Components that need updating:

1. **`client/src/views/Home.vue`**
   - Remove Clerk initialization
   - Replace with custom login modal
   - Update authentication check

2. **`client/src/views/Settings.vue`**
   - Remove Clerk user button
   - Replace with custom user menu

3. **`client/src/router/index.ts`**
   - Remove Clerk route guards
   - Replace with JWT-based guards

4. **`client/src/components/Login.vue` & `SignUp.vue`**
   - Remove Clerk OAuth buttons
   - Update to call `/login` and `/register` endpoints

5. **`client/src/stores/user.ts`**
   - Update to use JWT from `/login` response
   - Remove Clerk user type references

## Testing Checklist

Before deploying:

- [ ] Run database migrations on fresh database
- [ ] Test user registration (`POST /register`)
- [ ] Test user login (`POST /login`)
- [ ] Test JWT authentication on protected endpoints
- [ ] Test link creation with authenticated user
- [ ] Test link deletion with ownership validation
- [ ] Test health check endpoint
- [ ] Verify Stripe webhook still works
- [ ] Test subscription flow
- [ ] Build frontend (`bun build`)
- [ ] Build backend (`cargo build --release`)
- [ ] Test Docker build (`docker-compose up`)

## Next Steps

### Immediate (Required for functionality)

1. **Update Frontend Authentication**
   - Create login/register UI components
   - Update API calls to use new endpoints
   - Handle JWT storage and renewal
   - Update router guards

2. **Test End-to-End**
   - Register new user
   - Login
   - Create links
   - Test all features

### Soon (Recommended)

3. **Add Password Reset Flow**
   - `/forgot-password` endpoint
   - Email with reset token
   - Reset form

4. **Add Email Verification**
   - Send verification email on registration
   - Verify email before allowing login

5. **Improve Error Handling**
   - Better error messages
   - Validation feedback
   - Rate limiting on auth endpoints

### Future (Nice to have)

6. **Add OAuth (Optional)**
   - Google Sign-In
   - GitHub Sign-In
   - Without third-party auth providers

7. **Add 2FA (Optional)**
   - TOTP-based 2FA
   - Backup codes

## Files Changed Summary

### Created
- ✅ `server/migrations/001_initial_schema.sql`
- ✅ `server/migrations/002_functions.sql`
- ✅ `server/src/database.rs`
- ✅ `server/.env.example`
- ✅ `server/Dockerfile`
- ✅ `client/.env.example`
- ✅ `docker-compose.yml`
- ✅ `railway.toml`
- ✅ `.dockerignore`
- ✅ `README.md`
- ✅ `DEPLOYMENT.md`
- ✅ `MIGRATION_SUMMARY.md`

### Modified
- ✅ `server/src/main.rs` - Auth endpoints, removed credentials, ownership validation
- ✅ `server/src/middleware.rs` - Custom JWT validation
- ✅ `server/Cargo.toml` - Added bcrypt
- ✅ `client/package.json` - Removed Clerk and Supabase

### Deleted
- ✅ `server/src/supabase.rs` (renamed to database.rs)
- ✅ `client/src/utils/supabase.ts`
- ✅ `supabase/` directory

### Needs Manual Update (Frontend)
- ⏳ `client/src/views/Home.vue`
- ⏳ `client/src/views/Settings.vue`
- ⏳ `client/src/components/Login.vue`
- ⏳ `client/src/components/SignUp.vue`
- ⏳ `client/src/router/index.ts`
- ⏳ `client/src/stores/user.ts`

## Support

If you have questions about the migration:

1. Check `README.md` for setup instructions
2. Check `DEPLOYMENT.md` for Railway deployment
3. Review this document for what changed
4. Check the code comments for implementation details

## Rollback Plan

If you need to rollback:

1. **Database**: Restore from backup before migration
2. **Code**: Revert to commit before migration started
3. **Dependencies**: `cd client && bun install` (will reinstall Clerk/Supabase)
4. **Environment**: Restore old `.env` files with Clerk/Supabase credentials

---

**Migration completed on**: Jan 6, 2026
**Migrated by**: OpenCode AI Assistant
**Status**: ✅ Backend complete, ⏳ Frontend auth UI needs implementation

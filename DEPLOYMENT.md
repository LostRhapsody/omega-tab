# Deployment Guide - Railway

This guide covers deploying BetterNewTab to Railway.

## Prerequisites

- Railway account ([railway.app](https://railway.app))
- Railway CLI installed (optional)
- Project configured locally

## Deployment Steps

### 1. Create a New Railway Project

```bash
# Install Railway CLI (if not already installed)
npm install -g @railway/cli

# Login to Railway
railway login

# Create new project
railway init
```

Or use the Railway web dashboard at [railway.app/new](https://railway.app/new).

### 2. Add PostgreSQL Database

1. In your Railway project, click **"New"** → **"Database"** → **"PostgreSQL"**
2. Railway will create a PostgreSQL instance and provide connection details
3. The `DATABASE_URL` environment variable will be automatically set

### 3. Run Database Migrations

After the PostgreSQL service is created:

```bash
# Connect to the database and run migrations
railway run psql $DATABASE_URL -f server/migrations/001_initial_schema.sql
railway run psql $DATABASE_URL -f server/migrations/002_functions.sql
```

Or manually connect via psql:

```bash
# Get the DATABASE_URL from Railway dashboard
psql <your-database-url> -f server/migrations/001_initial_schema.sql
psql <your-database-url> -f server/migrations/002_functions.sql
```

### 4. Configure Environment Variables

In the Railway dashboard, set the following environment variables for your backend service:

#### Required Variables

```bash
# Database (automatically set by Railway if using their PostgreSQL)
DATABASE_URL=<provided-by-railway>

# JWT Secret (generate a secure random string)
JWT_SECRET=<your-super-secret-key>

# Stripe
STRIPE_SECRET_KEY=<your-stripe-secret-key>
STRIPE_ENDPOINT_SECRET=<your-webhook-secret>
STRIPE_VERIFY_WEBHOOK_SIGNATURE=true

# Plans (use the UUID from your database)
FREE_PLAN_ID=a0b1c2d3-e4f5-6789-abcd-ef0123456789

# Environment
ENVIRONMENT=production
DOMAIN=<your-railway-domain>
```

#### Optional Variables

```bash
# Brave Search API (if using search suggestions)
BRAVE_SUGGEST_URL=https://api.search.brave.com/res/v1/suggest/search
BRAVE_API_KEY=<your-brave-api-key>

# Support Email
CUSTOMER_SUPPORT_EMAIL=support@yourdomain.com

# Sentry (error tracking)
SENTRY_DSN=<your-sentry-dsn>
TRACING_SAMPLE_RATE=0.1
```

### 5. Deploy the Backend

Railway will automatically detect the Dockerfile and build your Rust backend.

```bash
# Deploy from CLI
railway up

# Or link to GitHub repo for automatic deployments
# Go to Railway dashboard → Settings → Connect to GitHub
```

### 6. Build and Deploy the Frontend

#### Option A: Serve from Backend (Simpler)

This approach serves the frontend static files from the Rust backend.

1. **Build the frontend locally**:

```bash
cd client
bun install
bun build
```

2. **Update backend to serve static files** - add to `main.rs`:

```rust
use tower_http::services::ServeDir;

// Add before existing routes
let app = Router::new()
    .nest_service("/", ServeDir::new("../client/dist"))
    .route("/api/register", post(register_handler))
    // ... rest of your routes with /api prefix
```

3. **Commit and push changes**

#### Option B: Separate Frontend Deployment (Better for production)

1. **Create a new Railway service** for the frontend
2. **Use a static site template**:

```bash
# In Railway, create new service
# Select "Deploy from GitHub repo"
# Choose your repository
# Set root directory to: client/
# Set build command: bun install && bun build
# Set start command: npx serve -s dist -l 3001
```

3. **Set frontend environment variables**:

```bash
VITE_API_BASE_URL=https://<your-backend-railway-url>
VITE_PLUS_PLAN_URL=<your-stripe-url>
VITE_PRO_PLAN_URL=<your-stripe-url>
# ... other frontend vars
```

### 7. Configure Custom Domain (Optional)

1. In Railway dashboard, go to your service
2. Click **Settings** → **Domains**
3. Add your custom domain
4. Update DNS records as instructed
5. Update `DOMAIN` environment variable in backend

### 8. Verify Deployment

```bash
# Test health check
curl https://<your-railway-domain>/health

# Test registration
curl -X POST https://<your-railway-domain>/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"testpass"}'
```

## Post-Deployment

### Setting Up Stripe Webhooks

1. Go to [Stripe Dashboard → Webhooks](https://dashboard.stripe.com/webhooks)
2. Click **Add endpoint**
3. Set URL to: `https://<your-railway-domain>/stripe_cancel_hook`
4. Select events: `customer.subscription.deleted`
5. Copy the **Signing secret** and add to `STRIPE_ENDPOINT_SECRET` in Railway

### Monitoring

Railway provides built-in monitoring:
- **Logs**: View real-time logs in the dashboard
- **Metrics**: CPU, memory, and network usage
- **Deployments**: Track deployment history

Additionally, Sentry is configured for error tracking:
- View errors at [sentry.io](https://sentry.io)

## Database Backups

Railway automatically backs up PostgreSQL:
- Backups are taken daily
- Restore from Railway dashboard → Database → Backups

## Troubleshooting

### Build Fails

Check Railway logs:
```bash
railway logs
```

Common issues:
- Missing dependencies in Dockerfile
- Environment variables not set
- Database connection issues

### Database Connection Errors

Verify `DATABASE_URL`:
```bash
railway variables

# Test connection
railway run psql $DATABASE_URL -c "SELECT 1"
```

### Migration Issues

Reset and re-run migrations:
```bash
# Drop and recreate database (WARNING: deletes all data)
railway run psql $DATABASE_URL -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

# Re-run migrations
railway run psql $DATABASE_URL -f server/migrations/001_initial_schema.sql
railway run psql $DATABASE_URL -f server/migrations/002_functions.sql
```

## Rollback

Railway keeps deployment history:
1. Go to **Deployments** tab
2. Find the working version
3. Click **Redeploy**

## Scaling

Railway provides automatic scaling:
- Increase resources in **Settings** → **Resources**
- Horizontal scaling available on Pro plan

## Cost Optimization

- **Development**: Use Hobby plan ($5/month)
- **Production**: Upgrade to Pro plan for better performance
- **Database**: Monitor usage to avoid overages
- **Cron Jobs**: Use Railway Cron for scheduled tasks

## CI/CD

Railway supports automatic deployments from GitHub:

1. Connect repository in Railway dashboard
2. Every push to `main` branch triggers deployment
3. Pull requests create preview environments (Pro plan)

## Security Checklist

- [ ] Change all default passwords and secrets
- [ ] Set `ENVIRONMENT=production`
- [ ] Enable `STRIPE_VERIFY_WEBHOOK_SIGNATURE=true`
- [ ] Use strong `JWT_SECRET` (64+ random characters)
- [ ] Enable HTTPS (automatic with Railway)
- [ ] Configure CORS for production domains
- [ ] Set up Sentry error tracking
- [ ] Regular database backups
- [ ] Monitor logs for suspicious activity

## Additional Resources

- [Railway Documentation](https://docs.railway.app)
- [PostgreSQL on Railway](https://docs.railway.app/databases/postgresql)
- [Custom Domains](https://docs.railway.app/deploy/exposing-your-app)
- [Environment Variables](https://docs.railway.app/develop/variables)

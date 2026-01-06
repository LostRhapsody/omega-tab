-- Initial database schema for BetterNewTab
-- This migration creates all necessary tables for the application

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table with password authentication
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- Plans table for subscription tiers
CREATE TABLE IF NOT EXISTS plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    max_pins INTEGER NOT NULL,
    features JSONB NOT NULL,
    stripe_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Organizations table for team workspaces (future feature)
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    owner_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_organizations_owner_id ON organizations(owner_id);

-- Teams table for sub-organization groups (future feature)
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    owner_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_teams_owner_id ON teams(owner_id);
CREATE INDEX idx_teams_organization_id ON teams(organization_id);

-- Subscriptions table
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    plan_id UUID NOT NULL REFERENCES plans(id) ON DELETE RESTRICT,
    status TEXT NOT NULL,
    stripe_subscription_id TEXT,
    current_period_end TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_subscriptions_entity ON subscriptions(entity_id, entity_type);
CREATE INDEX idx_subscriptions_stripe_id ON subscriptions(stripe_subscription_id);

-- User memberships for organizations and teams
CREATE TABLE IF NOT EXISTS user_memberships (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, entity_id, entity_type)
);

CREATE INDEX idx_user_memberships_user_id ON user_memberships(user_id);
CREATE INDEX idx_user_memberships_entity ON user_memberships(entity_id, entity_type);

-- Links table for user bookmarks
CREATE TABLE IF NOT EXISTS links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    icon TEXT,
    description TEXT,
    order_index INTEGER NOT NULL,
    column_type TEXT NOT NULL DEFAULT 'default',
    owner_id TEXT NOT NULL,
    owner_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_links_owner ON links(owner_id, owner_type);
CREATE INDEX idx_links_column_type ON links(column_type);

-- User settings table for preferences
CREATE TABLE IF NOT EXISTS user_settings (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    settings_blob JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Feedback timestamps for rate limiting
CREATE TABLE IF NOT EXISTS feedback_timestamps (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default free plan
INSERT INTO plans (id, name, max_pins, features, stripe_id)
VALUES (
    'a0b1c2d3-e4f5-6789-abcd-ef0123456789',
    'Free',
    10,
    '{"metadata": false, "jira": false, "confluence": false, "linear": false}',
    NULL
) ON CONFLICT (id) DO NOTHING;

-- Insert Plus plan (example - update with your actual Stripe plan ID)
INSERT INTO plans (id, name, max_pins, features, stripe_id)
VALUES (
    uuid_generate_v4(),
    'Plus',
    50,
    '{"metadata": true, "jira": true, "confluence": true, "linear": true}',
    'price_plus_plan_id_here'
) ON CONFLICT (id) DO NOTHING;

-- Insert Pro plan (example - update with your actual Stripe plan ID)
INSERT INTO plans (id, name, max_pins, features, stripe_id)
VALUES (
    uuid_generate_v4(),
    'Pro',
    999,
    '{"metadata": true, "jira": true, "confluence": true, "linear": true, "teams": true}',
    'price_pro_plan_id_here'
) ON CONFLICT (id) DO NOTHING;

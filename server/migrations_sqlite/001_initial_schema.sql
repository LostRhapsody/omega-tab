-- Initial database schema for BetterNewTab (SQLite)
-- This migration creates all necessary tables for the application

-- Users table with password authentication
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Plans table for subscription tiers
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    max_pins INTEGER NOT NULL,
    features TEXT NOT NULL,
    stripe_id TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Organizations table for team workspaces (future feature)
CREATE TABLE IF NOT EXISTS organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_organizations_owner_id ON organizations(owner_id);

-- Teams table for sub-organization groups (future feature)
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    organization_id TEXT REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_teams_owner_id ON teams(owner_id);
CREATE INDEX IF NOT EXISTS idx_teams_organization_id ON teams(organization_id);

-- Subscriptions table
CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    plan_id TEXT NOT NULL REFERENCES plans(id) ON DELETE RESTRICT,
    status TEXT NOT NULL,
    stripe_subscription_id TEXT,
    current_period_end TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_entity ON subscriptions(entity_id, entity_type);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe_id ON subscriptions(stripe_subscription_id);

-- User memberships for organizations and teams
CREATE TABLE IF NOT EXISTS user_memberships (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (user_id, entity_id, entity_type)
);

CREATE INDEX IF NOT EXISTS idx_user_memberships_user_id ON user_memberships(user_id);
CREATE INDEX IF NOT EXISTS idx_user_memberships_entity ON user_memberships(entity_id, entity_type);

-- Links table for user bookmarks
CREATE TABLE IF NOT EXISTS links (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    icon TEXT,
    description TEXT,
    order_index INTEGER NOT NULL,
    column_type TEXT NOT NULL DEFAULT 'default',
    owner_id TEXT NOT NULL,
    owner_type TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_links_owner ON links(owner_id, owner_type);
CREATE INDEX IF NOT EXISTS idx_links_column_type ON links(column_type);

-- User settings table for preferences
CREATE TABLE IF NOT EXISTS user_settings (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    settings_blob TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Feedback timestamps for rate limiting
CREATE TABLE IF NOT EXISTS feedback_timestamps (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default free plan
INSERT OR IGNORE INTO plans (id, name, max_pins, features, stripe_id)
VALUES (
    'a0b1c2d3-e4f5-6789-abcd-ef0123456789',
    'Free',
    10,
    '{"metadata": false, "jira": false, "confluence": false, "linear": false}',
    NULL
);

-- Insert Plus plan
INSERT OR IGNORE INTO plans (id, name, max_pins, features, stripe_id)
SELECT
    lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
    'Plus',
    50,
    '{"metadata": true, "jira": true, "confluence": true, "linear": true}',
    'price_plus_plan_id_here'
WHERE NOT EXISTS (SELECT 1 FROM plans WHERE name = 'Plus');

-- Insert Pro plan
INSERT OR IGNORE INTO plans (id, name, max_pins, features, stripe_id)
SELECT
    lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
    'Pro',
    999,
    '{"metadata": true, "jira": true, "confluence": true, "linear": true, "teams": true}',
    'price_pro_plan_id_here'
WHERE NOT EXISTS (SELECT 1 FROM plans WHERE name = 'Pro');

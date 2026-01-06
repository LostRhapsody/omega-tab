-- Database functions for BetterNewTab

-- Function to create an organization with owner membership
CREATE OR REPLACE FUNCTION create_organization(
    org_name TEXT,
    owner_id TEXT,
    plan_id UUID
) RETURNS TEXT AS $$
DECLARE
    new_org_id UUID;
    new_subscription_id UUID;
BEGIN
    -- Create the organization
    INSERT INTO organizations (name, owner_id)
    VALUES (org_name, owner_id)
    RETURNING id INTO new_org_id;

    -- Create subscription for the organization
    INSERT INTO subscriptions (entity_id, entity_type, plan_id, status)
    VALUES (new_org_id::TEXT, 'organization', plan_id, 'active')
    RETURNING id INTO new_subscription_id;

    -- Add owner as admin member
    INSERT INTO user_memberships (user_id, entity_id, entity_type, role)
    VALUES (owner_id, new_org_id::TEXT, 'organization', 'admin');

    RETURN new_org_id::TEXT;
END;
$$ LANGUAGE plpgsql;

-- Function to create a team with owner membership
CREATE OR REPLACE FUNCTION create_team(
    team_name TEXT,
    owner_id TEXT,
    plan_id UUID,
    org_id UUID DEFAULT NULL
) RETURNS TEXT AS $$
DECLARE
    new_team_id UUID;
    new_subscription_id UUID;
BEGIN
    -- Create the team
    INSERT INTO teams (name, owner_id, organization_id)
    VALUES (team_name, owner_id, org_id)
    RETURNING id INTO new_team_id;

    -- Create subscription for the team
    INSERT INTO subscriptions (entity_id, entity_type, plan_id, status)
    VALUES (new_team_id::TEXT, 'team', plan_id, 'active')
    RETURNING id INTO new_subscription_id;

    -- Add owner as admin member
    INSERT INTO user_memberships (user_id, entity_id, entity_type, role)
    VALUES (owner_id, new_team_id::TEXT, 'team', 'admin');

    RETURN new_team_id::TEXT;
END;
$$ LANGUAGE plpgsql;

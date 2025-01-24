/*
    Usage example
    // Get user's links
    const links = await linkUtils.getUserLinks(userId);

    // Create a new team
    const team = await teamUtils.createTeam({ name: 'My Team' }, userId);

    // Add member to organization
    await membershipUtils.addMember(userId, orgId, 'organization', 'member');
*/

import { createClient } from "@supabase/supabase-js";
import type { Database, Tables } from "../types/Database";

type Link = Tables<"links">;
type Organization = Tables<"organizations">;
type Team = Tables<"teams">;
type UserMembership = Tables<"user_memberships">;

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL;
const supabaseKey = import.meta.env.VITE_SUPABASE_KEY;

const supabase = createClient<Database>(supabaseUrl, supabaseKey);

export const linkUtils = {
	async getUserLinks(userId: string) {
		const { data, error } = await supabase
			.from("links")
			.select("*")
			.eq("owner_id", userId)
			.eq("owner_type", "user")
			.order("order_index");

		if (error) throw error;
		return data;
	},

	async getTeamLinks(teamId: string) {
		const { data, error } = await supabase
			.from("links")
			.select("*")
			.eq("owner_id", teamId)
			.eq("owner_type", "team")
			.order("order_index");

		if (error) throw error;
		return data;
	},

	async createLink(link: Omit<Link, "id" | "created_at">) {
		const { data, error } = await supabase
			.from("links")
			.insert(link)
			.select()
			.single();

		if (error) throw error;
		return data;
	},

	async updateLink(
		id: string,
		updates: Partial<Omit<Link, "id" | "created_at">>,
	) {
		const { data, error } = await supabase
			.from("links")
			.update(updates)
			.eq("id", id)
			.select()
			.single();

		if (error) throw error;
		return data;
	},

	async deleteLink(id: string) {
		const { error } = await supabase.from("links").delete().eq("id", id);

		if (error) throw error;
	},

	async reorderLinks(
		links: {
			id: string;
			order_index: number;
			owner_id: string;
			owner_type: string;
			title: string;
			url: string;
		}[],
	) {
		const { error } = await supabase
			.from("links")
			.upsert(links, { onConflict: "id" });

		if (error) throw error;
	},
};

export const teamUtils = {
	async getUserTeams(userId: string) {
		const { data, error } = await supabase
			.from("user_memberships")
			.select(`
        entity_id,
        role,
        teams (
          id,
          name,
          organization_id
        )
      `)
			.eq("user_id", userId)
			.eq("entity_type", "team");

		if (error) throw error;
		return data;
	},

	async createTeam(team: Omit<Team, "id" | "created_at">, userId: string) {
		const { data: teamData, error: teamError } = await supabase
			.from("teams")
			.insert(team)
			.select()
			.single();

		if (teamError) throw teamError;

		const membership: Omit<UserMembership, "created_at"> = {
			user_id: userId,
			entity_id: teamData.id,
			entity_type: "team",
			role: "owner",
		};

		const { error: membershipError } = await supabase
			.from("user_memberships")
			.insert(membership);

		if (membershipError) throw membershipError;

		return teamData;
	},
};

export const orgUtils = {
	async getUserOrgs(userId: string) {
		const { data, error } = await supabase
			.from("user_memberships")
			.select(`
        entity_id,
        role,
        organizations (
          id,
          name
        )
      `)
			.eq("user_id", userId)
			.eq("entity_type", "organization");

		if (error) throw error;
		return data;
	},

	async createOrg(
		org: Omit<Organization, "id" | "created_at">,
		userId: string,
	) {
		const { data: orgData, error: orgError } = await supabase
			.from("organizations")
			.insert(org)
			.select()
			.single();

		if (orgError) throw orgError;

		const membership: Omit<UserMembership, "created_at"> = {
			user_id: userId,
			entity_id: orgData.id,
			entity_type: "organization",
			role: "owner",
		};

		const { error: membershipError } = await supabase
			.from("user_memberships")
			.insert(membership);

		if (membershipError) throw membershipError;

		return orgData;
	},
};

export const membershipUtils = {
	async addMember(
		userId: string,
		entityId: string,
		entityType: "team" | "organization",
		role: string,
	) {
		const membership: Omit<UserMembership, "created_at"> = {
			user_id: userId,
			entity_id: entityId,
			entity_type: entityType,
			role,
		};

		const { error } = await supabase
			.from("user_memberships")
			.insert(membership);

		if (error) throw error;
	},

	async removeMember(userId: string, entityId: string) {
		const { error } = await supabase
			.from("user_memberships")
			.delete()
			.eq("user_id", userId)
			.eq("entity_id", entityId);

		if (error) throw error;
	},

	async updateMemberRole(userId: string, entityId: string, role: string) {
		const { error } = await supabase
			.from("user_memberships")
			.update({ role })
			.eq("user_id", userId)
			.eq("entity_id", entityId);

		if (error) throw error;
	},
};

export const subscriptionUtils = {
	async getUserPlan(userId: string) {
		// Check for direct user subscription
		const { data: userSub } = await supabase
			.from("subscriptions")
			.select(`
        plan_id,
        plans (*)
      `)
			.eq("entity_id", userId)
			.eq("entity_type", "user")
			.maybeSingle();

		if (userSub) return userSub.plans;

		// Check for team/org subscription
		const { data: memberships } = await supabase
			.from("subscriptions")
			.select(`
        plan_id,
        plans (*),
        user_memberships!inner (
          entity_id,
          entity_type
        )
      `)
			.eq("user_memberships.user_id", userId)
			.eq("status", "active");

		// Return highest tier plan from memberships or free plan
		if (!memberships?.length) {
			const { data: freePlan } = await supabase
				.from("plans")
				.select()
				.eq("name", "free")
				.single();
			return freePlan;
		}

		return memberships.reduce((highest, current) => {
			return current.plans.max_pins > highest.max_pins
				? current.plans
				: highest;
		}, memberships[0].plans);
	},

	async enforceUserLimits(
		userId: string,
		action: "pin" | "domain" | "analytics" | "team",
	) {
		const plan = await this.getUserPlan(userId);
		if (!plan) return false;

		const plan_features = plan.features as {
			custom_domains: boolean;
			analytics: boolean;
			team_features: boolean;
		};

		switch (action) {
			case "pin": {
				const { count } = await supabase
					.from("links")
					.select("*", { count: "exact" })
					.eq("owner_id", userId);
				return count !== null && count < plan.max_pins;
			}

			case "domain":
				return plan_features.custom_domains;

			case "analytics":
				return plan_features.analytics;

			case "team":
				return plan_features.team_features;
		}
	},
};

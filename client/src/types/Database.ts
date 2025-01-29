export type Json =
	| string
	| number
	| boolean
	| null
	| { [key: string]: Json | undefined }
	| Json[];

export type Database = {
	graphql_public: {
		Tables: {
			[_ in never]: never;
		};
		Views: {
			[_ in never]: never;
		};
		Functions: {
			graphql: {
				Args: {
					operationName?: string;
					query?: string;
					variables?: Json;
					extensions?: Json;
				};
				Returns: Json;
			};
		};
		Enums: {
			[_ in never]: never;
		};
		CompositeTypes: {
			[_ in never]: never;
		};
	};
	public: {
		Tables: {
			links: {
				Row: {
					column_type: string;
					created_at: string;
					description: string | null;
					icon: string | null;
					id: string;
					order_index: number;
					owner_id: string;
					owner_type: string;
					title: string;
					url: string;
				};
				Insert: {
					column_type?: string;
					created_at?: string;
					description?: string | null;
					icon?: string | null;
					id?: string;
					order_index: number;
					owner_id: string;
					owner_type: string;
					title: string;
					url: string;
				};
				Update: {
					column_type?: string;
					created_at?: string;
					description?: string | null;
					icon?: string | null;
					id?: string;
					order_index?: number;
					owner_id?: string;
					owner_type?: string;
					title?: string;
					url?: string;
				};
				Relationships: [];
			};
			organizations: {
				Row: {
					created_at: string;
					id: string;
					name: string;
					owner_id: string | null;
				};
				Insert: {
					created_at?: string;
					id?: string;
					name: string;
					owner_id?: string | null;
				};
				Update: {
					created_at?: string;
					id?: string;
					name?: string;
					owner_id?: string | null;
				};
				Relationships: [
					{
						foreignKeyName: "organizations_owner_id_fkey";
						columns: ["owner_id"];
						isOneToOne: false;
						referencedRelation: "users";
						referencedColumns: ["id"];
					},
				];
			};
			plans: {
				Row: {
					created_at: string | null;
					features: Json;
					id: string;
					max_pins: number;
					name: string;
					stripe_id: string | null;
				};
				Insert: {
					created_at?: string | null;
					features: Json;
					id?: string;
					max_pins: number;
					name: string;
					stripe_id?: string | null;
				};
				Update: {
					created_at?: string | null;
					features?: Json;
					id?: string;
					max_pins?: number;
					name?: string;
					stripe_id?: string | null;
				};
				Relationships: [];
			};
			subscriptions: {
				Row: {
					created_at: string | null;
					current_period_end: string | null;
					entity_id: string;
					entity_type: string;
					id: string;
					plan_id: string;
					status: string;
					stripe_subscription_id: string | null;
				};
				Insert: {
					created_at?: string | null;
					current_period_end?: string | null;
					entity_id: string;
					entity_type: string;
					id?: string;
					plan_id: string;
					status: string;
					stripe_subscription_id?: string | null;
				};
				Update: {
					created_at?: string | null;
					current_period_end?: string | null;
					entity_id?: string;
					entity_type?: string;
					id?: string;
					plan_id?: string;
					status?: string;
					stripe_subscription_id?: string | null;
				};
				Relationships: [
					{
						foreignKeyName: "subscriptions_plan_id_fkey";
						columns: ["plan_id"];
						isOneToOne: false;
						referencedRelation: "plans";
						referencedColumns: ["id"];
					},
				];
			};
			teams: {
				Row: {
					created_at: string;
					id: string;
					name: string;
					organization_id: string | null;
					owner_id: string | null;
				};
				Insert: {
					created_at?: string;
					id?: string;
					name: string;
					organization_id?: string | null;
					owner_id?: string | null;
				};
				Update: {
					created_at?: string;
					id?: string;
					name?: string;
					organization_id?: string | null;
					owner_id?: string | null;
				};
				Relationships: [
					{
						foreignKeyName: "teams_organization_id_fkey";
						columns: ["organization_id"];
						isOneToOne: false;
						referencedRelation: "organizations";
						referencedColumns: ["id"];
					},
					{
						foreignKeyName: "teams_owner_id_fkey";
						columns: ["owner_id"];
						isOneToOne: false;
						referencedRelation: "users";
						referencedColumns: ["id"];
					},
				];
			};
			user_memberships: {
				Row: {
					created_at: string;
					entity_id: string;
					entity_type: string;
					role: string;
					user_id: string;
				};
				Insert: {
					created_at?: string;
					entity_id: string;
					entity_type: string;
					role: string;
					user_id: string;
				};
				Update: {
					created_at?: string;
					entity_id?: string;
					entity_type?: string;
					role?: string;
					user_id?: string;
				};
				Relationships: [
					{
						foreignKeyName: "user_memberships_user_id_fkey";
						columns: ["user_id"];
						isOneToOne: false;
						referencedRelation: "users";
						referencedColumns: ["id"];
					},
				];
			};
			users: {
				Row: {
					created_at: string;
					email: string;
					id: string;
				};
				Insert: {
					created_at?: string;
					email: string;
					id: string;
				};
				Update: {
					created_at?: string;
					email?: string;
					id?: string;
				};
				Relationships: [];
			};
		};
		Views: {
			[_ in never]: never;
		};
		Functions: {
			create_organization: {
				Args: {
					org_name: string;
					owner_id: string;
					plan_id: string;
				};
				Returns: string;
			};
			create_team: {
				Args: {
					team_name: string;
					owner_id: string;
					plan_id: string;
					org_id?: string;
				};
				Returns: string;
			};
		};
		Enums: {
			[_ in never]: never;
		};
		CompositeTypes: {
			[_ in never]: never;
		};
	};
};

type PublicSchema = Database[Extract<keyof Database, "public">];

export type Tables<
	PublicTableNameOrOptions extends
		| keyof (PublicSchema["Tables"] & PublicSchema["Views"])
		| { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
				Database[PublicTableNameOrOptions["schema"]]["Views"])
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
			Database[PublicTableNameOrOptions["schema"]]["Views"])[TableName] extends {
			Row: infer R;
		}
		? R
		: never
	: PublicTableNameOrOptions extends keyof (PublicSchema["Tables"] &
				PublicSchema["Views"])
		? (PublicSchema["Tables"] &
				PublicSchema["Views"])[PublicTableNameOrOptions] extends {
				Row: infer R;
			}
			? R
			: never
		: never;

export type TablesInsert<
	PublicTableNameOrOptions extends
		| keyof PublicSchema["Tables"]
		| { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
			Insert: infer I;
		}
		? I
		: never
	: PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
		? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
				Insert: infer I;
			}
			? I
			: never
		: never;

export type TablesUpdate<
	PublicTableNameOrOptions extends
		| keyof PublicSchema["Tables"]
		| { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
			Update: infer U;
		}
		? U
		: never
	: PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
		? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
				Update: infer U;
			}
			? U
			: never
		: never;

export type Enums<
	PublicEnumNameOrOptions extends
		| keyof PublicSchema["Enums"]
		| { schema: keyof Database },
	EnumName extends PublicEnumNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicEnumNameOrOptions["schema"]]["Enums"]
		: never = never,
> = PublicEnumNameOrOptions extends { schema: keyof Database }
	? Database[PublicEnumNameOrOptions["schema"]]["Enums"][EnumName]
	: PublicEnumNameOrOptions extends keyof PublicSchema["Enums"]
		? PublicSchema["Enums"][PublicEnumNameOrOptions]
		: never;

export type CompositeTypes<
	PublicCompositeTypeNameOrOptions extends
		| keyof PublicSchema["CompositeTypes"]
		| { schema: keyof Database },
	CompositeTypeName extends PublicCompositeTypeNameOrOptions extends {
		schema: keyof Database;
	}
		? keyof Database[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"]
		: never = never,
> = PublicCompositeTypeNameOrOptions extends { schema: keyof Database }
	? Database[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"][CompositeTypeName]
	: PublicCompositeTypeNameOrOptions extends keyof PublicSchema["CompositeTypes"]
		? PublicSchema["CompositeTypes"][PublicCompositeTypeNameOrOptions]
		: never;

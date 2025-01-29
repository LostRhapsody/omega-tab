export type Link = {
	id: string;
	title: string;
	url: string;
	icon: string | null;
	order_index: number;
	owner_type: string;
	owner_id: string;
	created_at: string;
	description: string | null;
	column_type: string;
};

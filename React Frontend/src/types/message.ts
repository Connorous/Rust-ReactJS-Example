export interface Message {
    id: number;
    sender_id: number;
    relationship_id: number | null;
    group_id: number | null;
    message: string;
    created_at: string;
    updated_at: string;
}

export interface Relationship {
    id: number;
    requester_id: number;
    receiver_id: number
    status_id: number;
    blocked_by: number | null;
    declined_by: number | null;
    requester_username: string;
    receiver_username: string;
}

export interface DirectMessageRelationship {
    id: number;
    requester_id: number;
    receiver_id: number
    status_id: number;
    blocked_by: number | null;
    declined_by: number | null;
    requester_username: string;
    receiver_username: string;
    last_message: string | null;
    last_message_at: string | null;
    last_message_updated_at: string | null;
}

export interface RelationshipStatus {
    id: number;
    status: string;
}
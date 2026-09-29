export interface Relationship {
    id: number;
    requester_id: number;
    receiver_id: number
    status_id: number;
    blocked_by: number | null;
    declined_by: number | null;
}

export interface RelationshipStatus {
    id: number;
    status: string;
}
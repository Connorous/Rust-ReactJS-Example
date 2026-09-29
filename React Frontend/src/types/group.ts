export interface Group {
    id: number;
    name: string;
    updated_by: number | null;
    created_at: string;
    updated_at: string;
}

export interface GroupPermission {
    id: number;
    group_id: number;
    user_id: number;
    permission_type_id: number;
}

export interface GroupPermissionType {
    id: number;
    permission_type: string;
}

export interface GroupMember {
    id: number;
    username: string;
}
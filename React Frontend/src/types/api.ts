import type {User, Theme, UserType, UserStatus, AccountStatus, ShowNameChoice} from "./user"
import type { RelationshipStatus } from "./relationship";
import type { GroupPermissionType } from "./group";

export interface ApiResponse<T> {
    msg: string;
    success: boolean;
    data?: T;
    empty?: boolean;
}

/*export interface PaginationResponse<T> {
    msg: string;
    success: boolean;
    data: T[];
    empty?: boolean;
}*/

export interface LoginResponse {
    msg: string;
    success: boolean;
    access_token: string | null;
    user: User | null;
    themes: Theme[] | null;
    user_types: UserType[] | null;
    user_statuses: UserStatus[] | null;
    account_statuses: AccountStatus[] | null;
    relationship_statuses: RelationshipStatus[] | null;
    group_permission_types: GroupPermissionType[] | null;
    show_name_choices: ShowNameChoice[] | null;
}
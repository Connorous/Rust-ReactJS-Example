import { createContext } from "react";
import type {User, Theme, UserType, ShowNameChoice, AccountStatus, UserStatus} from '../types/user';
import type {RelationshipStatus} from '../types/relationship';
import type {GroupPermissionType} from '../types/group';

interface AuthContextType {
    accessToken: string | null;
    setAccessToken: (token: string | null) => void;
    sessionUser: User | null;
    setSessionUser: (user: User | null) => void;
    themes: Theme[];
    setThemes: (themes: Theme[]) => void;
    userTypes: UserType[];
    setUserTypes: (types: UserType[]) => void;
    userStatuses: UserStatus[];
    setUserStatuses: (types: UserStatus[]) => void;
    showNameChoices: ShowNameChoice[];
    setShowNameChoices: (types: ShowNameChoice[]) => void;
    accountStatuses: AccountStatus[];
    setAccountStatuses: (statuses: AccountStatus[]) => void;
    relationshipStatuses: RelationshipStatus[];
    setRelationshipStatuses: (statuses: RelationshipStatus[]) => void;
    groupPermissionTypes: GroupPermissionType[];
    setGroupPermissionTypes: (types: GroupPermissionType[]) => void;
    clearRefreshTimeout: () => void; 
    logout: () => void;
    API_URL: string;
    SECRET: string;
    isDesktop: boolean;
}

export const AuthContext = createContext<AuthContextType>({} as AuthContextType);
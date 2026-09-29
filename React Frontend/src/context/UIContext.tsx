import { createContext, useState } from 'react';

interface UIContextType {
    // Last selected items
    lastSelectedProfileId: number | null;
    setSelectedProfileId: (id: number | null) => void;
    lastSelectedDM: number | null;
    setLastSelectedDM: (id: number | null) => void;
    lastSelectedFriend: number | null;
    setLastSelectedFriend: (id: number | null) => void;
    lastSelectedGroup: number | null;
    setLastSelectedGroup: (id: number | null) => void;
    lastSelectedRequest: number | null;
    setLastSelectedRequest: (id: number | null) => void;
    lastSelectedManagedUser: number | null;
    setLastSelectedManagedUser: (id: number | null) => void;
   

    // Manage users view state
    // 0 = user table, 1 = user details, 2 = new user
    manageUsersView: number;
    setManageUsersView: (view: number) => void;
}

export const UIContext = createContext<UIContextType>({} as UIContextType);

export function UIProvider({ children }: { children: React.ReactNode }) {
    const [lastSelectedProfileId, setSelectedProfileId] = useState<number | null>(null);
    const [lastSelectedDM, setLastSelectedDM] = useState<number | null>(null);
    const [lastSelectedFriend, setLastSelectedFriend] = useState<number | null>(null);
    const [lastSelectedGroup, setLastSelectedGroup] = useState<number | null>(null);
    const [lastSelectedRequest, setLastSelectedRequest] = useState<number | null>(null);
    const [lastSelectedManagedUser, setLastSelectedManagedUser] = useState<number | null>(null);
    const [manageUsersView, setManageUsersView] = useState<number>(0);

    return (
        <UIContext.Provider value={{
            lastSelectedProfileId,
            setSelectedProfileId,
            lastSelectedDM,
            setLastSelectedDM,
            lastSelectedFriend,
            setLastSelectedFriend,
            lastSelectedGroup,
            setLastSelectedGroup,
            lastSelectedRequest,
            setLastSelectedRequest,
            lastSelectedManagedUser,
            setLastSelectedManagedUser,
            manageUsersView,
            setManageUsersView,
        }}>
            {children}
        </UIContext.Provider>
    );
}
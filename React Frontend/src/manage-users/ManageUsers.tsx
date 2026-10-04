import { useState } from 'react';
import { useWSEvent } from '../hooks/useWSEvent';
import UserTable from './UserTable';
import UserDetails from './UserDetails';
import NewUser from './NewUser';
import type { UserRow } from '../types/user';
import './manage-users.css';

function ManageUsers() {

    // 0 = table, 1 = details, 2 = new user
    const [view, setView] = useState(0);
    const [selectedUser, setSelectedUser] = useState<UserRow | null>(null);
    const [refreshTable, setRefreshTable] = useState(0);

    useWSEvent('users_list_refresh', (_data: any) => {
        setRefreshTable(prev => prev + 1);
    });

    function handleSelectUser(user: UserRow) {
        setSelectedUser(user);
        setView(1);
    }

    function handleBackToTable() {
        setView(0);
        setSelectedUser(null);
    }

    function handleUserUpdated(updatedUser: UserRow) {
        setSelectedUser(updatedUser);
        setRefreshTable(prev => prev + 1);
    }

    function handleUserDeleted() {
        setView(0);
        setSelectedUser(null);
        setRefreshTable(prev => prev + 1);
    }

    function handleNewUserCreated() {
        setView(0);
        setRefreshTable(prev => prev + 1);
    }

    if (view === 0) {
        return (
            <>
                <div className="manage-users-page">
                    <UserTable
                        onSelectUser={(user) => handleSelectUser(user)}
                        onNewUser={() => setView(2)}
                        refreshTrigger={refreshTable}
                    />
                </div>
            </>
        );
    }

    if (view === 1 && selectedUser) {
        return (
            <>
                <div className="manage-users-page">
                    <UserDetails
                        selectedUser={selectedUser}
                        onBack={() => handleBackToTable()}
                        onUserUpdated={(user) => handleUserUpdated(user)}
                        onUserDeleted={() => handleUserDeleted()}
                    />
                </div>
            </>
        );
    }

    if (view === 2) {
        return (
            <>
                <div className="manage-users-page">
                    <NewUser
                        onBack={() => setView(0)}
                        onUserCreated={() => handleNewUserCreated()}
                    />
                </div>
            </>
        );
    }

    return <></>;
}

export default ManageUsers;

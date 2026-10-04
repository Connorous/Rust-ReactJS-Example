import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { Group } from '../types/group';
import type { ApiResponse } from '../types/api';
import '../layout/layout.css';

function GroupList() {
    const { accessToken, API_URL } = useContext(AuthContext);
    const { lastSelectedGroup, setLastSelectedGroup } = useContext(UIContext);

    const [groups, setGroups] = useState<Group[]>([]);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        fetchGroups();
    }, []);

    useWSEvent('groups_refresh', (_data: any) => {
        fetchGroups();
    });

    useWSEvent('group_updated', (_data: any) => {
        fetchGroups();
    });

    useWSEvent('group_deleted', (_data: any) => {
        fetchGroups();
    });

    useWSEvent('group_permission_deleted', (_data: any) => {
        fetchGroups();
    });

    async function fetchGroups() {
        setLoading(true);

        try {
            const res = await fetch(`${API_URL}groups/list`, {
                method: 'GET',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
            });

            const data: ApiResponse<Group[]> = await res.json();

            if (data.success === true) {
                setGroups(data.data!);
            } else if (data.empty === true) {
                setGroups([]);
            }
        } catch (e) {
            console.log('Failed to fetch groups');
        }

        setLoading(false);
    }

    function handleSelect(groupId: number) {
        setLastSelectedGroup(groupId);
    }

    if (loading) {
        return (
            <>
                <p className="sidebar-list-loading">Loading...</p>
            </>
        );
    }

    if (groups.length === 0) {
        return (
            <>
                <p className="sidebar-list-empty">No groups yet</p>
            </>
        );
    }

    return (
        <>
            <div className="sidebar-list-items">
                {groups.map(group => (
                    <button
                        key={group.id}
                        className={lastSelectedGroup === group.id ? 'sidebar-list-item active' : 'sidebar-list-item'}
                        onClick={() => handleSelect(group.id)}
                    >
                        🗨️ {group.name}
                    </button>
                ))}
            </div>
        </>
    );
}

export default GroupList;
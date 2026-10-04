import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { UserRow } from '../types/user';
import type { Group } from '../types/group';
import type { ApiResponse } from '../types/api';
import './manage-users.css';

interface UserGroupListProps {
    selectedUser: UserRow;
    onSelectGroup: (groupId: number) => void;
}

function UserGroupList({ selectedUser, onSelectGroup }: UserGroupListProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    const [groups, setGroups] = useState<Group[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    var [search, setSearch] = useState('');
    var sr = search;

    useEffect(() => {
        fetchGroups();
    }, [selectedUser.id]);

    async function fetchGroups() {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}groups/list-users-groups`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_id: selectedUser.id }),
            });

            const data: ApiResponse<Group[]> = await res.json();

            if (data.success === true) {
                setGroups(data.data!);
            } else if (data.empty === true) {
                setGroups([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function searchGroups() {
        if (sr.trim().length < 2) {
            fetchGroups();
            return;
        }

        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}groups/search-users-groups`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    user_id: selectedUser.id,
                    search: sr,
                }),
            });

            const data: ApiResponse<Group[]> = await res.json();

            if (data.success === true) {
                setGroups(data.data!);
            } else if (data.empty === true) {
                setGroups([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            searchGroups();
        }
    }

    return (
        <>
            <div className="user-list-page">
                <div className="user-list-search">
                    <input
                        className="manage-input"
                        type="text"
                        placeholder="Search groups..."
                        value={sr}
                        onChange={(e) => {
                            setSearch(e.target.value);
                            sr = e.target.value;
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                    />
                    <button
                        className="manage-search-btn"
                        onClick={() => searchGroups()}
                        disabled={loading}
                    >
                        {loading ? '...' : 'Search'}
                    </button>
                </div>

                {error !== '' ? (
                    <p className="manage-error">{error}</p>
                ) : (<></>)}

                {groups.length === 0 && !loading ? (
                    <p className="manage-empty">No groups found</p>
                ) : (<></>)}

                <div className="user-list-items">
                    {groups.map(group => (
                        <div
                            key={group.id}
                            className="user-list-item"
                            onClick={() => onSelectGroup(group.id)}
                        >
                            <div className="user-list-item-info">
                                <span className="user-list-item-name">{group.name}</span>
                                {group.is_public ? (
                                    <span className="badge badge-blue">Public</span>
                                ) : (
                                    <span className="badge badge-gray">Private</span>
                                )}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </>
    );
}

export default UserGroupList;

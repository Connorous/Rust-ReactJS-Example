import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { UserRow } from '../types/user';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import './manage-users.css';

interface UserRelationshipListProps {
    selectedUser: UserRow;
    onSelectRelationship: (rel: Relationship) => void;
}

function UserRelationshipList({ selectedUser, onSelectRelationship }: UserRelationshipListProps) {
    const { accessToken, API_URL, relationshipStatuses } = useContext(AuthContext);

    const [relationships, setRelationships] = useState<Relationship[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    var [search, setSearch] = useState('');
    var sr = search;

    useEffect(() => {
        fetchRelationships();
    }, [selectedUser.id]);

    async function fetchRelationships() {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}relationships/list-users-relationships`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_id: selectedUser.id }),
            });

            const data: ApiResponse<Relationship[]> = await res.json();

            if (data.success === true) {
                setRelationships(data.data!);
            } else if (data.empty === true) {
                setRelationships([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function searchRelationships() {
        if (sr.trim().length < 2) {
            fetchRelationships();
            return;
        }

        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}relationships/search-users-relationships`, {
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

            const data: ApiResponse<Relationship[]> = await res.json();

            if (data.success === true) {
                setRelationships(data.data!);
            } else if (data.empty === true) {
                setRelationships([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    function getStatusName(statusId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < relationshipStatuses.length; i++) {
            if (relationshipStatuses[i].id === statusId) {
                name = relationshipStatuses[i].status;
                break;
            }
        }
        return name;
    }

    function getStatusClass(statusId: number): string {
        if (statusId === 2) return 'badge badge-green';
        if (statusId === 3) return 'badge badge-red';
        if (statusId === 1) return 'badge badge-blue';
        return 'badge badge-gray';
    }

    function getOtherUsername(rel: Relationship): string {
        if (rel.requester_id === selectedUser.id) {
            return rel.receiver_username;
        }
        return rel.requester_username;
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            searchRelationships();
        }
    }

    return (
        <>
            <div className="user-list-page">
                <div className="user-list-search">
                    <input
                        className="manage-input"
                        type="text"
                        placeholder="Search relationships..."
                        value={sr}
                        onChange={(e) => {
                            setSearch(e.target.value);
                            sr = e.target.value;
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                    />
                    <button
                        className="manage-search-btn"
                        onClick={() => searchRelationships()}
                        disabled={loading}
                    >
                        {loading ? '...' : 'Search'}
                    </button>
                </div>

                {error !== '' ? (
                    <p className="manage-error">{error}</p>
                ) : (<></>)}

                {relationships.length === 0 && !loading ? (
                    <p className="manage-empty">No relationships found</p>
                ) : (<></>)}

                <div className="user-list-items">
                    {relationships.map(rel => (
                        <div
                            key={rel.id}
                            className="user-list-item"
                            onClick={() => onSelectRelationship(rel)}
                        >
                            <div className="user-list-item-info">
                                <span className="user-list-item-name">
                                    @{getOtherUsername(rel)}
                                </span>
                                <span className={getStatusClass(rel.status_id)}>
                                    {getStatusName(rel.status_id)}
                                </span>
                            </div>
                            {rel.status_id === 3 && rel.blocked_by !== null ? (
                                <span className="user-list-item-sub">
                                    Blocked by @{rel.blocked_by === selectedUser.id
                                        ? selectedUser.username
                                        : getOtherUsername(rel)}
                                </span>
                            ) : (<></>)}
                        </div>
                    ))}
                </div>
            </div>
        </>
    );
}

export default UserRelationshipList;

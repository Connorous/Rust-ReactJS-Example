import { useState, useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import '../layout/layout.css';

function FriendsList() {
    const { sessionUser, accessToken, API_URL } = useContext(AuthContext);
    const { lastSelectedFriend, setLastSelectedFriend, setLastSelectedProfileId } = useContext(UIContext);
    const navigate = useNavigate();

    const [relationships, setRelationships] = useState<Relationship[]>([]);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        fetchRelationships();
    }, []);

    useWSEvent('relationships_refresh', (_data: any) => {
        fetchRelationships();
    });

    useWSEvent('relationship_updated', (_data: any) => {
        fetchRelationships();
    });

    async function fetchRelationships() {
        setLoading(true);

        try {
            const res = await fetch(`${API_URL}relationships/list`, {
                method: 'GET',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
            });

            const data: ApiResponse<Relationship[]> = await res.json();

            if (data.success === true) {
                var accepted: Relationship[] = [];

                for (var i = 0; i < data.data!.length; i++) {
                    if (data.data![i].status_id === 2) {
                        accepted.push(data.data![i]);
                    }
                }

                setRelationships(accepted);
            }
        } catch (e) {
            console.log('Failed to fetch relationships');
        }

        setLoading(false);
    }

    function getOtherUserId(rel: Relationship): number {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_id;
        }
        return rel.requester_id;
    }

    function getOtherUsername(rel: Relationship): string {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_username;
        }
        return rel.requester_username;
    }

    function handleSelect(rel: Relationship) {
        var otherId = getOtherUserId(rel);
        setLastSelectedFriend(otherId);
        setLastSelectedProfileId(otherId);
        navigate('/relationship');
    }

    if (loading) {
        return (
            <>
                <p className="sidebar-list-loading">Loading...</p>
            </>
        );
    }

    if (relationships.length === 0) {
        return (
            <>
                <p className="sidebar-list-empty">No friends yet</p>
            </>
        );
    }

    return (
        <>
            <div className="sidebar-list-items">
                {relationships.map(rel => (
                    <button
                        key={rel.id}
                        className={lastSelectedFriend === getOtherUserId(rel) ? 'sidebar-list-item active' : 'sidebar-list-item'}
                        onClick={() => handleSelect(rel)}
                    >
                        👤 User {getOtherUsername(rel)}
                    </button>
                ))}
            </div>
        </>
    );
}

export default FriendsList;
import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import '../layout/layout.css';

function RequestsList() {
    const { sessionUser, accessToken, API_URL } = useContext(AuthContext);
    const { lastSelectedRequest, setLastSelectedRequest } = useContext(UIContext);

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
                // Only show pending relationships where current user is receiver
                var pending: Relationship[] = [];

                for (var i = 0; i < data.data!.length; i++) {
                    if (data.data![i].status_id === 1
                        && data.data![i].receiver_id === sessionUser?.id) {
                        pending.push(data.data![i]);
                    }
                }

                setRelationships(pending);
            }
        } catch (e) {
            console.log('Failed to fetch relationships');
        }

        setLoading(false);
    }

    function handleSelect(rel: Relationship) {
        setLastSelectedRequest(rel.id);
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
                <p className="sidebar-list-empty">No pending requests</p>
            </>
        );
    }

    return (
        <>
            <div className="sidebar-list-items">
                {relationships.map(rel => (
                    <button
                        key={rel.id}
                        className={lastSelectedRequest === rel.id ? 'sidebar-list-item active' : 'sidebar-list-item'}
                        onClick={() => handleSelect(rel)}
                    >
                        📬 User {rel.requester_id}
                    </button>
                ))}
            </div>
        </>
    );
}

export default RequestsList;

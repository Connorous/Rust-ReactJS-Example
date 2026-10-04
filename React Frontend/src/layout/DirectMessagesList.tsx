import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { DirectMessageRelationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import '../layout/layout.css';

function DirectMessagesList() {
    const { sessionUser, accessToken, API_URL } = useContext(AuthContext);
    const { lastSelectedDM, setLastSelectedDM } = useContext(UIContext);

    const [relationships, setRelationships] = useState<DirectMessageRelationship[]>([]);
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

    useWSEvent('message_created', (_data: any) => {
        // Refresh list when new message arrives to update preview
        if (_data.relationship_id) {
            fetchRelationships();
        }
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

            const data: ApiResponse<DirectMessageRelationship[]> = await res.json();

            if (data.success === true) {
                var filtered: DirectMessageRelationship[] = [];

                for (var i = 0; i < data.data!.length; i++) {
                    if (data.data![i].status_id === 2 || data.data![i].status_id === 3) {
                        filtered.push(data.data![i]);
                    }
                }

                // Sort by last message date — most recent first
                filtered.sort((a, b) => {
                    if (a.last_message_at === null) return 1;
                    if (b.last_message_at === null) return -1;
                    return new Date(b.last_message_at).getTime() - new Date(a.last_message_at).getTime();
                });

                setRelationships(filtered);
            }
        } catch (e) {
            console.log('Failed to fetch relationships');
        }

        setLoading(false);
    }

    function getOtherUserId(rel: DirectMessageRelationship): number {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_id;
        }
        return rel.requester_id;
    }

    function getOtherUsername(rel: DirectMessageRelationship): string {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_username;
        }
        return rel.requester_username;
    }

    function getMessagePreview(rel: DirectMessageRelationship): string {
        if (rel.last_message === null) return '';
        if (rel.last_message.length > 30) {
            return rel.last_message.substring(0, 30) + '...';
        }
        return rel.last_message;
    }

    function handleSelect(rel: DirectMessageRelationship) {
        setLastSelectedDM(getOtherUserId(rel));
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
                <p className="sidebar-list-empty">No conversations yet</p>
            </>
        );
    }

    return (
        <>
            <div className="sidebar-list-items">
                {relationships.map(rel => (
                    <button
                        key={rel.id}
                        className={lastSelectedDM === getOtherUserId(rel) ? 'sidebar-list-item active' : 'sidebar-list-item'}
                        onClick={() => handleSelect(rel)}
                    >
                        <span className="sidebar-list-item-name">
                            💬 @{getOtherUsername(rel)}
                            {rel.status_id === 3 ? (
                                <span className="sidebar-list-tag-blocked"> Blocked</span>
                            ) : (
                                <></>
                            )}
                        </span>
                        {rel.last_message !== null ? (
                            <>
                                <span className="sidebar-list-preview">
                                    {getMessagePreview(rel)}
                                </span>
                            </>
                        ) : (
                            <></>
                        )}
                    </button>
                ))}
            </div>
        </>
    );
}

export default DirectMessagesList;
import { useState, useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import ConfirmModal from '../shared/ConfirmModel';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import './relationships.css';

function RelationshipPage() {
    const { sessionUser, accessToken, API_URL } = useContext(AuthContext);
    const {
        lastSelectedFriend,
        setLastSelectedFriend,
        lastSelectedRequest,
        setLastSelectedRequest,
        setLastSelectedDM,
        setLastSelectedProfileId,
    } = useContext(UIContext);
    const navigate = useNavigate();

    // 0 = search, 1 = friend detail, 2 = request detail
    const [view, setView] = useState(0);

    const [relationship, setRelationship] = useState<Relationship | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [actionLoading, setActionLoading] = useState(false);
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [showBlockModal, setShowBlockModal] = useState(false);

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;
    const [searchResults, setSearchResults] = useState<Relationship[]>([]);
    const [searchLoading, setSearchLoading] = useState(false);
    const [searchError, setSearchError] = useState('');

    useEffect(() => {
        if (lastSelectedFriend !== null) {
            setView(1);
            fetchRelationshipByUserId(lastSelectedFriend);
        }
    }, [lastSelectedFriend]);

    useEffect(() => {
        if (lastSelectedRequest !== null) {
            setView(2);
            fetchRelationshipById(lastSelectedRequest);
        }
    }, [lastSelectedRequest]);

    useWSEvent('relationship_updated', (data: any) => {
        if (relationship && data.relationship_id === relationship.id) {
            if (lastSelectedFriend !== null) {
                fetchRelationshipByUserId(lastSelectedFriend);
            } else if (lastSelectedRequest !== null) {
                fetchRelationshipById(lastSelectedRequest);
            }
        }
    });

    useWSEvent('relationships_refresh', (_data: any) => {
        if (lastSelectedFriend !== null) {
            fetchRelationshipByUserId(lastSelectedFriend);
        } else if (lastSelectedRequest !== null) {
            fetchRelationshipById(lastSelectedRequest);
        }
    });

    async function fetchRelationshipByUserId(userId: number) {
        setLoading(true);
        setError('');
        setRelationship(null);

        try {
            const res = await fetch(`${API_URL}relationships/get/relationship`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_id: userId }),
            });

            const data: ApiResponse<Relationship> = await res.json();

            if (data.success === true && data.data) {
                setRelationship(data.data);
            } else {
                setError('Relationship not found');
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function fetchRelationshipById(relationshipId: number) {
        setLoading(true);
        setError('');
        setRelationship(null);

        try {
            const res = await fetch(`${API_URL}relationships/list/pending`, {
                method: 'GET',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
            });

            const data: ApiResponse<Relationship[]> = await res.json();

            if (data.success === true) {
                var found: Relationship | null = null;

                for (var i = 0; i < data.data!.length; i++) {
                    if (data.data![i].id === relationshipId) {
                        found = data.data![i];
                        break;
                    }
                }

                if (found) {
                    setRelationship(found);
                } else {
                    setError('Request not found');
                }
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function searchRelationships() {
        if (st.trim().length < 2) {
            setSearchError('Search term must be at least 2 characters');
            return;
        }

        setSearchLoading(true);
        setSearchError('');
        setSearchResults([]);

        try {
            const res = await fetch(`${API_URL}relationships/search`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ search: st }),
            });

            const data: ApiResponse<Relationship[]> = await res.json();

            if (data.success === true) {
                setSearchResults(data.data!);
            } else if (data.empty === true) {
                setSearchResults([]);
            } else {
                setSearchError(data.msg);
            }
        } catch (e) {
            setSearchError('Cannot connect to server');
        }

        setSearchLoading(false);
    }

    function getOtherUserId(): number | null {
        if (!relationship) return null;
        if (relationship.requester_id === sessionUser?.id) {
            return relationship.receiver_id;
        }
        return relationship.requester_id;
    }

    function getOtherUsername(): string {
        if (!relationship) return 'Unknown';
        if (relationship.requester_id === sessionUser?.id) {
            return relationship.receiver_username;
        }
        return relationship.requester_username;
    }

    function getOtherUsernameFromResult(rel: Relationship): string {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_username;
        }
        return rel.requester_username;
    }

    function getOtherUserIdFromResult(rel: Relationship): number {
        if (rel.requester_id === sessionUser?.id) {
            return rel.receiver_id;
        }
        return rel.requester_id;
    }

    function isBlocker(): boolean {
        return relationship?.blocked_by === sessionUser?.id;
    }

    async function acceptRequest() {
        if (!relationship) return;

        setActionLoading(true);

        try {
            const res = await fetch(`${API_URL}relationships/relationship`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationship.id,
                    accepted: true,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                fetchRelationshipById(relationship.id);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setActionLoading(false);
    }

    async function declineRequest() {
        if (!relationship) return;

        setActionLoading(true);

        try {
            const res = await fetch(`${API_URL}relationships/relationship`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationship.id,
                    accepted: false,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setRelationship(null);
                setLastSelectedRequest(null);
                setView(0);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setActionLoading(false);
    }

    async function blockUnblock() {
        if (!relationship) return;

        setActionLoading(true);

        try {
            const res = await fetch(`${API_URL}relationships/block-relationship`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationship.id,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                fetchRelationshipByUserId(getOtherUserId()!);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setShowBlockModal(false);
        setActionLoading(false);
    }

    async function deleteRelationship() {
        if (!relationship) return;

        setActionLoading(true);

        try {
            const res = await fetch(`${API_URL}relationships/relationship`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationship.id,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setRelationship(null);
                setLastSelectedFriend(null);
                setView(0);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setShowDeleteModal(false);
        setActionLoading(false);
    }

    function handleViewProfile() {
        var otherId = getOtherUserId();
        if (otherId !== null) {
            setLastSelectedProfileId(otherId);
            navigate('/profile');
        }
    }

    function handleOpenDM() {
        var otherId = getOtherUserId();
        if (otherId !== null) {
            setLastSelectedDM(otherId);
            navigate('/direct-messages');
        }
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            searchRelationships();
        }
    }

    // Search view
    if (view === 0) {
        return (
            <>
                <div className="relationships-page">
                    <h2 className="relationships-title">Relationships</h2>

                    <div className="find-friends-search">
                        <input
                            className="find-friends-input"
                            type="text"
                            placeholder="Search relationships... (min 2 chars)"
                            value={st}
                            onChange={(e) => {
                                setSearchTerm(e.target.value);
                                st = e.target.value;
                                if (e.target.value.trim() !== '') setSearchError('');
                            }}
                            onKeyDown={(e) => handleKeyDown(e)}
                        />
                        <button
                            className="find-friends-search-btn"
                            onClick={() => searchRelationships()}
                            disabled={searchLoading}
                        >
                            {searchLoading ? 'Searching...' : 'Search'}
                        </button>
                    </div>

                    {searchError !== '' ? (
                        <>
                            <p className="relationships-error">{searchError}</p>
                        </>
                    ) : (
                        <></>
                    )}

                    {searchResults.length === 0 && !searchLoading && st.trim().length >= 2 ? (
                        <>
                            <p className="relationships-empty">No relationships found</p>
                        </>
                    ) : (
                        <></>
                    )}

                    <div className="find-friends-results">
                        {searchResults.map(rel => (
                            <div
                                key={rel.id}
                                className="find-friends-result"
                                onClick={() => {
                                    setLastSelectedFriend(getOtherUserIdFromResult(rel));
                                    setView(1);
                                    fetchRelationshipByUserId(getOtherUserIdFromResult(rel));
                                }}
                            >
                                <div className="find-friends-result-info">
                                    <span className="find-friends-username-btn">
                                        @{getOtherUsernameFromResult(rel)}
                                    </span>
                                    <span className="relationships-status-tag">
                                        {rel.status_id === 2 ? 'Friend' : rel.status_id === 3 ? 'Blocked' : 'Unknown'}
                                    </span>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </>
        );
    }

    if (loading) {
        return (
            <>
                <div className="relationships-page">
                    <p className="relationships-loading">Loading...</p>
                </div>
            </>
        );
    }

    if (error !== '' && !relationship) {
        return (
            <>
                <div className="relationships-page">
                    <p className="relationships-error">{error}</p>
                    <button
                        className="relationships-back-btn"
                        onClick={() => {
                            setView(0);
                            setLastSelectedFriend(null);
                            setLastSelectedRequest(null);
                        }}
                    >
                        ← Back
                    </button>
                </div>
            </>
        );
    }

    // Friend detail view
    if (view === 1 && relationship) {
        return (
            <>
                <div className="relationships-page">
                    <button
                        className="relationships-back-btn"
                        onClick={() => {
                            setView(0);
                            setLastSelectedFriend(null);
                            setRelationship(null);
                        }}
                    >
                        ← Back
                    </button>

                    <div className="relationships-card">
                        <div className="relationships-header">
                            <h2 className="relationships-username">@{getOtherUsername()}</h2>
                            {relationship.status_id === 3 ? (
                                <>
                                    <span className="relationships-blocked-tag">
                                        {isBlocker() ? 'Blocked by you' : 'Blocked you'}
                                    </span>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>

                        {error !== '' ? (
                            <>
                                <p className="relationships-error">{error}</p>
                            </>
                        ) : (
                            <></>
                        )}

                        <div className="relationships-btn-row">
                            <button
                                className="relationships-profile-btn"
                                onClick={() => handleViewProfile()}
                            >
                                View Profile
                            </button>

                            {relationship.status_id === 2 ? (
                                <>
                                    <button
                                        className="relationships-dm-btn"
                                        onClick={() => handleOpenDM()}
                                    >
                                        Message
                                    </button>
                                    <button
                                        className="relationships-block-btn"
                                        onClick={() => setShowBlockModal(true)}
                                    >
                                        Block
                                    </button>
                                    <button
                                        className="relationships-delete-btn"
                                        onClick={() => setShowDeleteModal(true)}
                                    >
                                        Remove Friend
                                    </button>
                                </>
                            ) : (
                                <></>
                            )}

                            {relationship.status_id === 3 && isBlocker() ? (
                                <>
                                    <button
                                        className="relationships-unblock-btn"
                                        onClick={() => blockUnblock()}
                                        disabled={actionLoading}
                                    >
                                        {actionLoading ? '...' : 'Unblock'}
                                    </button>
                                    <button
                                        className="relationships-delete-btn"
                                        onClick={() => setShowDeleteModal(true)}
                                    >
                                        Remove
                                    </button>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>
                    </div>

                    {showBlockModal ? (
                        <>
                            <ConfirmModal
                                message={`Are you sure you want to block @${getOtherUsername()}?`}
                                onConfirm={() => blockUnblock()}
                                onCancel={() => setShowBlockModal(false)}
                            />
                        </>
                    ) : (
                        <></>
                    )}

                    {showDeleteModal ? (
                        <>
                            <ConfirmModal
                                message={`Are you sure you want to remove @${getOtherUsername()}?`}
                                onConfirm={() => deleteRelationship()}
                                onCancel={() => setShowDeleteModal(false)}
                            />
                        </>
                    ) : (
                        <></>
                    )}
                </div>
            </>
        );
    }

    // Request detail view
    if (view === 2 && relationship) {
        return (
            <>
                <div className="relationships-page">
                    <button
                        className="relationships-back-btn"
                        onClick={() => {
                            setView(0);
                            setLastSelectedRequest(null);
                            setRelationship(null);
                        }}
                    >
                        ← Back
                    </button>

                    <div className="relationships-card">
                        <h2 className="relationships-title">Friend Request</h2>
                        <h3 className="relationships-username">
                            @{relationship.requester_username}
                        </h3>
                        <p className="relationships-subtitle">sent you a friend request</p>

                        {error !== '' ? (
                            <>
                                <p className="relationships-error">{error}</p>
                            </>
                        ) : (
                            <></>
                        )}

                        <div className="relationships-btn-row">
                            <button
                                className="relationships-profile-btn"
                                onClick={() => {
                                    setLastSelectedProfileId(relationship.requester_id);
                                    navigate('/profile');
                                }}
                            >
                                View Profile
                            </button>
                            <button
                                className="relationships-accept-btn"
                                onClick={() => acceptRequest()}
                                disabled={actionLoading}
                            >
                                {actionLoading ? '...' : 'Accept'}
                            </button>
                            <button
                                className="relationships-decline-btn"
                                onClick={() => declineRequest()}
                                disabled={actionLoading}
                            >
                                {actionLoading ? '...' : 'Decline'}
                            </button>
                        </div>
                    </div>
                </div>
            </>
        );
    }

    return (
        <>
            <div className="relationships-page">
                <p className="relationships-empty">
                    Select a friend or request from the sidebar
                </p>
            </div>
        </>
    );
}

export default RelationshipPage;
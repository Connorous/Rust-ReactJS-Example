import { useState, useEffect, useContext, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { Relationship } from '../types/relationship';
import type { Message } from '../types/message';
import type { ApiResponse } from '../types/api';
import MessageList from './MessageList';
import MessageInput from './MessageInput';
import './directMessages.css';

function DirectMessages() {
    const { sessionUser, accessToken, API_URL } = useContext(AuthContext);
    const { lastSelectedDM, setLastSelectedDM, setLastSelectedProfileId } = useContext(UIContext);
    const navigate = useNavigate();

    const [relationship, setRelationship] = useState<Relationship | null>(null);
    const [messages, setMessages] = useState<Message[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [otherUserId, setOtherUserId] = useState<number | null>(null);
    const [otherUsername, setOtherUsername] = useState<string>('');

    const [hasMore, setHasMore] = useState(false);
    const [loadingMore, setLoadingMore] = useState(false);

    const messagesEndRef = useRef<HTMLDivElement | null>(null);
    const messageListRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        if (lastSelectedDM !== null) {
            fetchRelationship(lastSelectedDM);
        }
    }, [lastSelectedDM]);

    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    // WS events
    useWSEvent('message_created', (data: any) => {
        if (data.relationship_id === relationship?.id) {
            setMessages(prev => [...prev, data]);
        }
    });

    useWSEvent('messages_refresh', (data: any) => {
        if (data.relationship_id === relationship?.id) {
            getMessages(relationship!.id);
        }
    });

    useWSEvent('relationship_updated', (data: any) => {
        if (relationship && data.relationship_id === relationship.id) {
            fetchRelationship(lastSelectedDM!);
        }
    });

    function scrollToBottom() {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }

    async function fetchRelationship(userId: number) {
        setLoading(true);
        setError('');

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

                // Work out other user id and username
                const otherId = data.data.requester_id === sessionUser?.id
                    ? data.data.receiver_id
                    : data.data.requester_id;

                setOtherUserId(otherId);
                fetchOtherUsername(otherId);

                // Only fetch messages if accepted or blocker
                if (canReadMessages(data.data)) {
                    getMessages(data.data.id);
                }
            } else {
                setError('Could not load conversation');
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function fetchOtherUsername(userId: number) {
        try {
            const res = await fetch(`${API_URL}users/user/get`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ id: userId }),
            });

            const data = await res.json();

            if (data.success === true) {
                setOtherUsername(data.data.username);
            }
        } catch (e) {
            console.log('Failed to fetch username');
        }
    }

    async function getMessages(relationshipId: number, beforeMessageId?: number) {
        try {
            const res = await fetch(`${API_URL}direct-messages/list`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ relationship_id: relationshipId, before_message_id: beforeMessageId ?? null }),
            });

            const data: ApiResponse<Message[]> = await res.json();

            if (data.success === true) {
                if (beforeMessageId) {
                    const list = messageListRef.current;
                    const scrollHeightBefore = list?.scrollHeight ?? 0;

                    setMessages(prev => [...data.data!, ...prev]);

                    requestAnimationFrame(() => { if (list) { list.scrollTop = list.scrollHeight - scrollHeightBefore;}});

                    if (data.data!.length < 100) {
                        setHasMore(false);
                    }
                }
                else {
                    setMessages(data.data!);
                    setHasMore(data.data!.length === 100);
                }
            } else if (data.empty === true) {
                setMessages([]);
            }
        } catch (e) {
            console.log('Failed to fetch messages');
        }
    }

    async function loadMoreMessages() {
        if (!hasMore || loadingMore || messages.length === 0) {
            return;
        }

        setLoadingMore(true);

        var oldestId = messages[0].id;
        await getMessages(relationship!.id, oldestId);
    }

    function canReadMessages(rel: Relationship): boolean {
        if (rel.status_id === 2) return true;  // accepted
        if (rel.status_id === 1) return false;  // pending
        if (rel.status_id === 4) return false;  // declined
        if (rel.status_id === 3) {
            // only blocker can read
            return rel.blocked_by === sessionUser?.id;
        }
        return false;
    }

    function canSendMessages(rel: Relationship): boolean {
        return rel.status_id === 2;  // accepted only
    }

    function handleUsernameClick() {
        if (otherUserId !== null) {
            setLastSelectedProfileId(otherUserId);
            navigate('/profile');
        }
    }

    if (!lastSelectedDM) {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-empty">Select a conversation from the sidebar</p>
                </div>
            </>
        );
    }

    if (loading) {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-loading">Loading...</p>
                </div>
            </>
        );
    }

    if (error !== '') {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-error">{error}</p>
                </div>
            </>
        );
    }

    if (!relationship) {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-empty">No conversation found</p>
                </div>
            </>
        );
    }

    // Declined
    if (relationship.status_id === 4) {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-restricted">This relationship has been declined.</p>
                </div>
            </>
        );
    }

    // Blocked and not the blocker
    if (relationship.status_id === 3 && relationship.blocked_by !== sessionUser?.id) {
        return (
            <>
                <div className="dm-page">
                    <p className="dm-restricted">You have been blocked by this user.</p>
                </div>
            </>
        );
    }

    return (
        <>
            <div className="dm-page">
                <div className="dm-header">
                    <button
                        className="dm-username-btn"
                        onClick={() => handleUsernameClick()}
                    >
                        @{otherUsername}
                    </button>
                    {relationship.status_id === 3 && (
                        <span className="dm-blocked-tag">Blocked</span>
                    )}
                    {relationship.status_id === 1 && (
                        <span className="dm-pending-tag">Pending</span>
                    )}
                </div>

                <MessageList
                    messages={messages}
                    sessionUser={sessionUser!}
                    relationshipId={relationship.id}
                    getMessages={() => getMessages(relationship.id)}
                    messagesEndRef={messagesEndRef}
                    messageListRef={messageListRef}
                    hasMore={hasMore}
                    loadingMore={loadingMore}
                    onScrollTop={() => loadMoreMessages()}
                />

                {canSendMessages(relationship) ? (
                    <>
                        <MessageInput
                            relationshipId={relationship.id}
                            onMessageSent={() => getMessages(relationship.id)}
                        />
                    </>
                ) : (
                    <>
                        {relationship.status_id === 1 ? (
                            <p className="dm-pending-msg">
                                Friend request pending — you can message once accepted.
                            </p>
                        ) : (
                            <></>
                        )}
                    </>
                )}
            </div>
        </>
    );
}

export default DirectMessages;
import { useState, useEffect, useContext, useRef } from 'react';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import { useWSEvent } from '../hooks/useWSEvent';
import type { Message } from '../types/message';
import type { ApiResponse } from '../types/api';
import type { Group, GroupPermission } from '../types/group';
import type { User } from '../types/user';
import GroupMessageList from './GroupMessageList';
import GroupMessageInput from './GroupMessageInput';
import GroupInfoPanel from './GroupInfoPanel';
import './chatGroups.css';

function ChatGroup() {
    const { sessionUser, accessToken, API_URL, isDesktop } = useContext(AuthContext);
    const { lastSelectedGroup } = useContext(UIContext);

    const [group, setGroup] = useState<Group | null>(null);
    const [groupPermission, setGroupPermission] = useState<GroupPermission | null>(null);
    const [messages, setMessages] = useState<Message[]>([]);
    const [senders, setSenders] = useState<User[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [showInfoPanel, setShowInfoPanel] = useState(false);

    const [hasMore, setHasMore] = useState(false);
    const [loadingMore, setLoadingMore] = useState(false);

    const messagesEndRef = useRef<HTMLDivElement | null>(null);
    const messageListRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        if (lastSelectedGroup !== null) {
            fetchGroup(lastSelectedGroup);
            fetchGroupPermission(lastSelectedGroup);
        }
    }, [lastSelectedGroup]);

    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    useWSEvent('message_created', (data: any) => {
        if (data.group_id === lastSelectedGroup) {
            setMessages(prev => [...prev, data]);

            if (!senderInList(data.sender_id)) {
                fetchMessageSenders(lastSelectedGroup!);
            }
        }
    });

    useWSEvent('messages_refresh', (data: any) => {
        if (data.group_id === lastSelectedGroup) {
            fetchMessages(lastSelectedGroup!);
        }
    });

    useWSEvent('group_updated', (data: any) => {
        if (data.group_id === lastSelectedGroup) {
            fetchGroup(lastSelectedGroup!);
        }
    });

    useWSEvent('group_permission_updated', (data: any) => {
        if (data.group_id === lastSelectedGroup && data.user_id === sessionUser?.id) {
            fetchGroupPermission(lastSelectedGroup!);
        }
    });

    useWSEvent('group_permission_deleted', (data: any) => {
        if (data.group_id === lastSelectedGroup && data.user_id === sessionUser?.id) {
            setGroup(null);
            setGroupPermission(null);
            setMessages([]);
        }
    });

    function scrollToBottom() {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }

    function senderInList(senderId: number): boolean {
        var found: boolean = false;

        for (var i = 0; i < senders.length; i++) {
            if (senders[i].id === senderId) {
                found = true;
                break;
            }
        }

        return found;
    }

    function getSenderUsername(senderId: number): string {
        var username: string = 'Unknown';

        for (var i = 0; i < senders.length; i++) {
            if (senders[i].id === senderId) {
                username = senders[i].username;
                break;
            }
        }

        return username;
    }

    function canSendMessages(): boolean {
        if (!groupPermission) return false;
        if (sessionUser?.user_type_id === 4) return false;  // viewer globally
        if (groupPermission.permission_type_id >= 4) return false;  // viewer or blocked in group
        return true;
    }

    function canManageGroup(): boolean {
        if (!groupPermission) return false;
        if (sessionUser?.user_type_id !== undefined && sessionUser.user_type_id <= 2) return true;  // admin
        if (groupPermission.permission_type_id <= 2) return true;  // owner or moderator
        return false;
    }

    function canDeleteMessage(senderId: number): boolean {
        if (!groupPermission) return false;
        if (senderId === sessionUser?.id) return true;  // own message
        if (sessionUser?.user_type_id !== undefined && sessionUser.user_type_id <= 2) return true;  // admin
        if (groupPermission.permission_type_id <= 2) return true;  // owner or moderator
        return false;
    }

    async function fetchGroup(groupId: number) {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}groups/group/get`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: groupId }),
            });

            const data: ApiResponse<Group> = await res.json();

            if (data.success === true) {
                setGroup(data.data!);
                fetchMessages(groupId);
                fetchMessageSenders(groupId);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function fetchGroupPermission(groupId: number) {
        try {
            const res = await fetch(`${API_URL}groups/permissions`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: groupId }),
            });

            const data: ApiResponse<GroupPermission[]> = await res.json();

            if (data.success === true) {
                var userPermission: GroupPermission | null = null;

                for (var i = 0; i < data.data!.length; i++) {
                    if (data.data![i].user_id === sessionUser?.id) {
                        userPermission = data.data![i];
                        break;
                    }
                }

                setGroupPermission(userPermission);
            }
        } catch (e) {
            console.log('Failed to fetch group permission');
        }
    }

    async function fetchMessages(groupId: number, beforeMessageId?: number) {
        if (!hasMore || loadingMore || messages.length === 0) {
            return;
        }

        try {
            const res = await fetch(`${API_URL}groups/messages`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: groupId, before_message_id: beforeMessageId }),
            });

            const data: ApiResponse<Message[]> = await res.json();

            if (data.success === true) {
                setMessages(data.data!);
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
        await fetchMessages(group!.id, oldestId);
    }

    async function fetchMessageSenders(groupId: number) {
        try {
            const res = await fetch(`${API_URL}groups/list-users-sent-group-messages`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: groupId }),
            });

            const data: ApiResponse<User[]> = await res.json();

            if (data.success === true) {
                setSenders(data.data!);
            }
        } catch (e) {
            console.log('Failed to fetch message senders');
        }
    }

    if (!lastSelectedGroup) {
        return (
            <>
                <div className="group-page">
                    <p className="group-empty">Select a group from the sidebar</p>
                </div>
            </>
        );
    }

    if (loading) {
        return (
            <>
                <div className="group-page">
                    <p className="group-loading">Loading...</p>
                </div>
            </>
        );
    }

    if (error !== '') {
        return (
            <>
                <div className="group-page">
                    <p className="group-error">{error}</p>
                </div>
            </>
        );
    }

    if (!group || !groupPermission) {
        return (
            <>
                <div className="group-page">
                    <p className="group-empty">Group not found</p>
                </div>
            </>
        );
    }

    // Blocked in group
    if (groupPermission.permission_type_id === 7) {
        return (
            <>
                <div className="group-page">
                    <p className="group-restricted">You have been blocked from this group.</p>
                </div>
            </>
        );
    }

    return (
        <>
            <div className="group-page">
                <div className="group-header">
    <h2 className="group-name">{group.name}</h2>
    {canManageGroup() ? (
        <>
            <button
                className="group-info-btn"
                onClick={() => setShowInfoPanel(!showInfoPanel)}
            >
                {showInfoPanel ? '✕ Close Info' : 'Group Info ›'}
            </button>
        </>
    ) : (
        <></>
    )}
</div>

                <div className="group-body">
                    <div className="group-chat">
                        <GroupMessageList
                            messages={messages}
                            sessionUser={sessionUser!}
                            groupId={group.id}
                            messagesEndRef={messagesEndRef}
                            messageListRef={messageListRef}
                            hasMore={hasMore}
                            loadingMore={loadingMore}
                            onScrollTop={() => loadMoreMessages()}
                        />

                        {canSendMessages() ? (
                            <>
                                <GroupMessageInput
                                    groupId={group.id}
                                    onMessageSent={() => fetchMessages(group.id)}
                                />
                            </>
                        ) : (
                            <>
                                <p className="group-viewer-msg">
                                    You are a viewer in this group and cannot send messages.
                                </p>
                            </>
                        )}
                    </div>

                    {showInfoPanel ? (
                        <>
                            <GroupInfoPanel
                                group={group}
                                groupPermission={groupPermission}
                                sessionUser={sessionUser!}
                                onGroupUpdated={() => fetchGroup(group.id)}
                                onClose={() => setShowInfoPanel(false)}
                            />
                        </>
                    ) : (
                        <></>
                    )}
                </div>
            </div>
        </>
    );
}

export default ChatGroup;
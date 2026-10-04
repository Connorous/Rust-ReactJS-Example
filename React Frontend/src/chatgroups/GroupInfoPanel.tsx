import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import { useWSEvent } from '../hooks/useWSEvent';
import ConfirmModal from '../shared/ConfirmModel';
import GroupMemberList from './GroupMemberList';
import type { Group, GroupPermissionRow, GroupMember } from '../types/group';
import type { User } from '../types/user';
import type { Message } from '../types/message';
import type { ApiResponse } from '../types/api';
import './chatGroups.css';

interface GroupInfoPanelProps {
    group: Group;
    groupPermission: GroupPermissionRow;
    sessionUser: User;
    onGroupUpdated: () => void;
    onClose: () => void;
    scrollToMessage: (id: number) => void;
}

function GroupInfoPanel({
    group,
    groupPermission,
    sessionUser,
    onGroupUpdated,
    onClose,
    scrollToMessage,
}: GroupInfoPanelProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    var [groupName, setGroupName] = useState(group.name);
    var gn = groupName;

    const [members, setMembers] = useState<GroupMember[]>([]);
    const [permissions, setPermissions] = useState<GroupPermissionRow[]>([]);
    const [nonMembers, setNonMembers] = useState<User[]>([]);

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;

    var [messageSearch, setMessageSearch] = useState('');
    var ms = messageSearch;

    const [nameError, setNameError] = useState('');
    const [nameSuccess, setNameSuccess] = useState('');
    const [nameLoading, setNameLoading] = useState(false);
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [, setDeleteLoading] = useState(false);
    const [error, setError] = useState('');

    // Collapsible sections — details and members open by default, search closed
    const [detailsExpanded, setDetailsExpanded] = useState(true);
    const [membersExpanded, setMembersExpanded] = useState(true);
    const [searchExpanded, setSearchExpanded] = useState(false);

    // Message search results
    const [searchResults, setSearchResults] = useState<Message[]>([]);
    const [searchLoading, setSearchLoading] = useState(false);

    const searchTimeoutRef = useState<ReturnType<typeof setTimeout> | null>(null);

    const isOwnerOrAdmin = sessionUser.user_type_id <= 2 || groupPermission.permission_type_id === 1;

    useEffect(() => {
        fetchMembers();
        fetchPermissions();
        fetchNonMembers();
    }, []);

    useWSEvent('group_permission_added', (data: any) => {
        if (data.group_id === group.id) {
            fetchMembers();
            fetchPermissions();
            fetchNonMembers();
        }
    });

    useWSEvent('group_permissions_refresh', (data: any) => {
        if (data.group_id === group.id) {
            fetchMembers();
            fetchPermissions();
            fetchNonMembers();
        }
    });

    async function fetchMembers() {
        try {
            const res = await fetch(`${API_URL}groups/list-group-members`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: group.id }),
            });

            const data: ApiResponse<GroupMember[]> = await res.json();

            if (data.success === true) {
                setMembers(data.data!);
            }
        } catch (e) {
            console.log('Failed to fetch members');
        }
    }

    async function fetchPermissions() {
        try {
            const res = await fetch(`${API_URL}groups/permissions`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: group.id }),
            });

            const data: ApiResponse<GroupPermissionRow[]> = await res.json();

            if (data.success === true) {
                setPermissions(data.data!);
            }
        } catch (e) {
            console.log('Failed to fetch permissions');
        }
    }

    async function fetchNonMembers() {
        try {
            const res = await fetch(`${API_URL}groups/list-non-group-members`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: group.id }),
            });

            const data: ApiResponse<User[]> = await res.json();

            if (data.success === true) {
                setNonMembers(data.data!);
            }
        } catch (e) {
            console.log('Failed to fetch non members');
        }
    }

    function getFilteredNonMembers(): User[] {
        var filtered: User[] = [];

        for (var i = 0; i < nonMembers.length; i++) {
            if (nonMembers[i].username.toLowerCase().includes(st.toLowerCase())) {
                filtered.push(nonMembers[i]);
            }
        }

        return filtered;
    }

    async function updateGroupName() {
        if (gn.trim() === '') {
            setNameError('Group name cannot be blank');
            return;
        }

        setNameLoading(true);
        setNameError('');
        setNameSuccess('');

        try {
            const res = await fetch(`${API_URL}groups/group`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    group_id: group.id,
                    name: gn,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setNameSuccess('Group name updated');
                onGroupUpdated();
            } else {
                setNameError(data.msg);
            }
        } catch (e) {
            setNameError('Cannot connect to server');
        }

        setNameLoading(false);
    }

    async function addMember(userId: number) {
        try {
            const res = await fetch(`${API_URL}groups/permission/new`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    group_id: group.id,
                    user_id: userId,
                    permission_type_id: 3,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                fetchMembers();
                fetchPermissions();
                fetchNonMembers();
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }
    }

    async function deleteGroup() {
        setDeleteLoading(true);

        try {
            const res = await fetch(`${API_URL}groups/group`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: group.id }),
            });

            const data = await res.json();

            if (data.success === true) {
                onClose();
                onGroupUpdated();
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setDeleteLoading(false);
        setShowDeleteModal(false);
    }

    function handleMessageSearchChange(value: string) {
        setMessageSearch(value);
        ms = value;

        if (searchTimeoutRef[0]) {
            clearTimeout(searchTimeoutRef[0]);
        }

        if (value.trim().length < 2) {
            setSearchResults([]);
            return;
        }

        // @ts-ignore
        searchTimeoutRef[0] = setTimeout(() => {
            searchMessages(value);
        }, 300);
    }

    function searchMessages(term: string) {
        // Frontend search — filters already loaded messages passed via prop
        // We'll need messages passed down — for now search via backend
        setSearchLoading(true);

        fetch(`${API_URL}groups/search-messages`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${accessToken}`,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                group_id: group.id,
                search_term: term,
            }),
        })
            .then(res => res.json())
            .then(data => {
                if (data.success === true) {
                    setSearchResults(data.data!);
                } else if (data.empty === true) {
                    setSearchResults([]);
                }
            })
            .catch(() => {
                console.log('Failed to search messages');
            })
            .finally(() => {
                setSearchLoading(false);
            });
    }

    function formatDate(dateStr: string): string {
        var date = new Date(dateStr);
        return date.toLocaleString();
    }

    return (
        <>
            <div className="group-info-panel">
                <div className="group-info-header">
                    <h3 className="group-info-title">Group Info</h3>
                    <button
                        className="group-info-close-btn"
                        onClick={() => onClose()}
                    >
                        ✕
                    </button>
                </div>

                {/* Group Details — collapsible */}
                {isOwnerOrAdmin ? (
                    <>
                        <div
                            className="group-info-section-header"
                            onClick={() => setDetailsExpanded(!detailsExpanded)}
                        >
                            <h4 className="group-info-section-title">Group Details</h4>
                            <span>{detailsExpanded ? '▲' : '▼'}</span>
                        </div>

                        {detailsExpanded ? (
                            <>
                                <div className="group-info-section">
                                    <input
                                        className="group-info-input"
                                        type="text"
                                        value={gn}
                                        onChange={(e) => {
                                            setGroupName(e.target.value);
                                            gn = e.target.value;
                                        }}
                                    />
                                    {nameError !== '' ? (
                                        <>
                                            <p className="group-info-error">{nameError}</p>
                                        </>
                                    ) : (
                                        <></>
                                    )}
                                    {nameSuccess !== '' ? (
                                        <>
                                            <p className="group-info-success">{nameSuccess}</p>
                                        </>
                                    ) : (
                                        <></>
                                    )}
                                    <button
                                        className="group-info-save-btn"
                                        onClick={() => updateGroupName()}
                                        disabled={nameLoading}
                                    >
                                        {nameLoading ? 'Saving...' : 'Save Name'}
                                    </button>

                                    <button
                                        className="group-delete-btn"
                                        onClick={() => setShowDeleteModal(true)}
                                    >
                                        Delete Group
                                    </button>
                                </div>
                            </>
                        ) : (
                            <></>
                        )}
                    </>
                ) : (
                    <></>
                )}

                {/* Members — collapsible */}
                <div
                    className="group-info-section-header"
                    onClick={() => setMembersExpanded(!membersExpanded)}
                >
                    <h4 className="group-info-section-title">Members</h4>
                    <span>{membersExpanded ? '▲' : '▼'}</span>
                </div>

                {membersExpanded ? (
                    <>
                        <div className="group-info-section">
                            <GroupMemberList
                                members={members}
                                permissions={permissions}
                                groupId={group.id}
                                sessionUser={sessionUser}
                                groupPermission={groupPermission}
                                onMembersUpdated={() => {
                                    fetchMembers();
                                    fetchPermissions();
                                    fetchNonMembers();
                                }}
                            />

                            {isOwnerOrAdmin ? (
                                <>
                                    <h4 className="group-info-section-title">Add Members</h4>
                                    <input
                                        className="group-info-input"
                                        type="text"
                                        placeholder="Search by username..."
                                        value={st}
                                        onChange={(e) => {
                                            setSearchTerm(e.target.value);
                                            st = e.target.value;
                                        }}
                                    />
                                    <div className="group-info-non-members">
                                        {getFilteredNonMembers().map(user => (
                                            <div key={user.id} className="group-non-member-row">
                                                <span className="group-non-member-name">@{user.username}</span>
                                                <button
                                                    className="group-add-btn"
                                                    onClick={() => addMember(user.id)}
                                                >
                                                    Add
                                                </button>
                                            </div>
                                        ))}
                                    </div>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>
                    </>
                ) : (
                    <></>
                )}

                {/* Message Search — collapsible, closed by default */}
                <div
                    className="group-info-section-header"
                    onClick={() => setSearchExpanded(!searchExpanded)}
                >
                    <h4 className="group-info-section-title">Message Search</h4>
                    <span>{searchExpanded ? '▲' : '▼'}</span>
                </div>

                {searchExpanded ? (
                    <>
                        <div className="group-info-section">
                            <input
                                className="group-info-input"
                                type="text"
                                placeholder="Search messages... (min 2 chars)"
                                value={ms}
                                onChange={(e) => handleMessageSearchChange(e.target.value)}
                            />

                            {searchLoading ? (
                                <>
                                    <p className="group-info-search-loading">Searching...</p>
                                </>
                            ) : (
                                <></>
                            )}

                            {searchResults.length === 0 && ms.trim().length >= 2 && !searchLoading ? (
                                <>
                                    <p className="group-info-empty">No messages found</p>
                                </>
                            ) : (
                                <></>
                            )}

                            <div className="group-info-search-results">
                                {searchResults.map(result => (
                                    <div key={result.id} className="group-info-search-result" onClick={() => scrollToMessage(result.id)}>
                                        <p className="group-info-search-message">{result.message}</p>
                                        <p className="group-info-search-time">{formatDate(result.created_at)}</p>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </>
                ) : (
                    <></>
                )}

                {error !== '' ? (
                    <>
                        <p className="group-info-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                {showDeleteModal ? (
                    <>
                        <ConfirmModal
                            message={`Are you sure you want to delete ${group.name}? This cannot be undone.`}
                            onConfirm={() => deleteGroup()}
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

export default GroupInfoPanel;
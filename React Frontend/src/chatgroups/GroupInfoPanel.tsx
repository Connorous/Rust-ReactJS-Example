import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import { useWSEvent } from '../hooks/useWSEvent';
import ConfirmModal from '../shared/ConfirmModel';
//import GroupMemberList from './GroupMemberViewHolder';
import type { Group, GroupPermission, GroupMember } from '../types/group';
import type { User } from '../types/user';
import type { ApiResponse } from '../types/api';
import './chatGroups.css';

interface GroupInfoPanelProps {
    group: Group;
    groupPermission: GroupPermission;
    sessionUser: User;
    onGroupUpdated: () => void;
    onClose: () => void;
}

function GroupInfoPanel({
    group,
    groupPermission,
    sessionUser,
    onGroupUpdated,
    onClose,
}: GroupInfoPanelProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    var [groupName, setGroupName] = useState(group.name);
    var gn = groupName;

    const [members, setMembers] = useState<GroupMember[]>([]);
    const [permissions, setPermissions] = useState<GroupPermission[]>([]);
    const [nonMembers, setNonMembers] = useState<User[]>([]);

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;

    const [nameError, setNameError] = useState('');
    const [nameSuccess, setNameSuccess] = useState('');
    const [nameLoading, setNameLoading] = useState(false);
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [deleteLoading, setDeleteLoading] = useState(false);
    const [error, setError] = useState('');

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

            const data: ApiResponse<GroupPermission[]> = await res.json();

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

                {isOwnerOrAdmin ? (
                    <>
                        <div className="group-info-section">
                            <h4 className="group-info-section-title">Group Name</h4>
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
                        </div>
                    </>
                ) : (
                    <></>
                )}

                <div className="group-info-section">
                    <h4 className="group-info-section-title">Members</h4>
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
                </div>

                <div className="group-info-section">
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
                </div>

                {error !== '' ? (
                    <>
                        <p className="group-info-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                {isOwnerOrAdmin ? (
                    <>
                        <button
                            className="group-delete-btn"
                            onClick={() => setShowDeleteModal(true)}
                        >
                            Delete Group
                        </button>
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
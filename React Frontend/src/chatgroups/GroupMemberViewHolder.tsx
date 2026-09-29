import type { GroupPermission } from '../types/group';
import type { User } from '../types/user';
import './chatGroups.css';

interface GroupMemberListProps {
    members: GroupPermission[];
    groupId: number;
    sessionUser: User;
    groupPermission: GroupPermission;
    onMembersUpdated: () => void;
}

function GroupMemberList({
    members,
    groupId,
    sessionUser,
    groupPermission,
    onMembersUpdated,
}: GroupMemberListProps) {
    if (members.length === 0) {
        return (
            <>
                <p className="group-info-empty">No members found</p>
            </>
        );
    }

    return (
        <>
            <div className="group-member-list">
                {members.map(member => (
                    <GroupMemberViewHolder
                        key={member.id}
                        member={member}
                        groupId={groupId}
                        sessionUser={sessionUser}
                        groupPermission={groupPermission}
                        onMembersUpdated={onMembersUpdated}
                    />
                ))}
            </div>
        </>
    );
}

export default GroupMemberList;
import { useState, useContext } from 'react';
import { AuthContext } from '../../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModal';
import type { GroupPermission } from '../../types/group';
import type { User } from '../../types/user';
import './chatGroups.css';

interface GroupMemberViewHolderProps {
    member: GroupPermission;
    groupId: number;
    sessionUser: User;
    groupPermission: GroupPermission;
    onMembersUpdated: () => void;
}

function GroupMemberViewHolder({
    member,
    groupId,
    sessionUser,
    groupPermission,
    onMembersUpdated,
}: GroupMemberViewHolderProps) {
    const { accessToken, API_URL, groupPermissionTypes } = useContext(AuthContext);

    const [selectedPermission, setSelectedPermission] = useState(member.permission_type_id);
    const [showRemoveModal, setShowRemoveModal] = useState(false);
    const [updateLoading, setUpdateLoading] = useState(false);
    const [removeLoading, setRemoveLoading] = useState(false);
    const [error, setError] = useState('');

    const isOwnerOrAdmin = sessionUser.user_type_id <= 2 || groupPermission.permission_type_id === 1;
    const isModerator = groupPermission.permission_type_id <= 2;
    const canManage = isOwnerOrAdmin || isModerator;
    const isSelf = member.user_id === sessionUser.id;

    function getPermissionName(permissionTypeId: number): string {
        var name: string = 'Unknown';

        for (var i = 0; i < groupPermissionTypes.length; i++) {
            if (groupPermissionTypes[i].id === permissionTypeId) {
                name = groupPermissionTypes[i].permission_type;
                break;
            }
        }

        return name;
    }

    async function updatePermission() {
        setUpdateLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}groups/permission`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    group_id: groupId,
                    user_id: member.user_id,
                    permission_type_id: selectedPermission,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                onMembersUpdated();
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setUpdateLoading(false);
    }

    async function removeMember() {
        setRemoveLoading(true);

        try {
            const res = await fetch(`${API_URL}groups/permission`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    group_id: groupId,
                    user_id: member.user_id,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                onMembersUpdated();
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setRemoveLoading(false);
        setShowRemoveModal(false);
    }

    return (
        <>
            <div className="group-member-row">
                <span className="group-member-name">
                    @{member.user_id}
                </span>

                {canManage && !isSelf ? (
                    <>
                        <select
                            className="group-member-select"
                            value={selectedPermission}
                            onChange={(e) => setSelectedPermission(Number(e.target.value))}
                        >
                            {groupPermissionTypes.map(type => (
                                <option key={type.id} value={type.id}>
                                    {type.permission_type}
                                </option>
                            ))}
                        </select>
                        <button
                            className="group-member-update-btn"
                            onClick={() => updatePermission()}
                            disabled={updateLoading}
                        >
                            {updateLoading ? '...' : 'Update'}
                        </button>
                        <button
                            className="group-member-remove-btn"
                            onClick={() => setShowRemoveModal(true)}
                        >
                            Remove
                        </button>
                    </>
                ) : (
                    <>
                        <span className="group-member-permission">
                            {getPermissionName(member.permission_type_id)}
                        </span>
                    </>
                )}

                {error !== '' ? (
                    <>
                        <p className="group-info-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                {showRemoveModal ? (
                    <>
                        <ConfirmModal
                            message="Are you sure you want to remove this member?"
                            onConfirm={() => removeMember()}
                            onCancel={() => setShowRemoveModal(false)}
                        />
                    </>
                ) : (
                    <></>
                )}
            </div>
        </>
    );
}

export default GroupMemberViewHolder;
import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModel';
import type { GroupPermissionRow, GroupMember } from '../types/group';
import type { User } from '../types/user';
import './chatGroups.css';

interface GroupMemberViewHolderProps {
    member: GroupMember;
    permission: GroupPermissionRow | null;
    groupId: number;
    sessionUser: User;
    groupPermission: GroupPermissionRow;
    onMembersUpdated: () => void;
}

function GroupMemberViewHolder({
    member,
    permission,
    groupId,
    sessionUser,
    groupPermission,
    onMembersUpdated,
}: GroupMemberViewHolderProps) {
    const { accessToken, API_URL, groupPermissionTypes } = useContext(AuthContext);

    // 0 = view, 1 = update
    const [mode, setMode] = useState(0);
    const [selectedPermission, setSelectedPermission] = useState(getMemberPermissionTypeId());
    var sp = selectedPermission;
    const [showRemoveModal, setShowRemoveModal] = useState(false);
    const [updateLoading, setUpdateLoading] = useState(false);
    const [, setRemoveLoading] = useState(false);
    const [error, setError] = useState('');

    const isOwnerOrAdmin = sessionUser.user_type_id <= 2 || groupPermission.permission_type_id === 1;
    const isModerator = groupPermission.permission_type_id <= 2;
    const canManage = isOwnerOrAdmin || isModerator;
    const isSelf = member.id === sessionUser.id;

    function getMemberPermissionTypeId(): number {
        if (!permission) return 3;
        return permission.permission_type_id;
    }

    function getMemberUpdatedByUsername(): string {
        if (!permission) return 'Unknown';
        return permission.updated_by_username;
    }

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
                    user_id: member.id,
                    permission_type_id: selectedPermission,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setMode(0);
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
                    user_id: member.id,
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
            <div className="group-member-item">

                {mode === 0 ? (
                    <>
                        <div className="group-member-row">
                            <span className="group-member-name">@{member.username}</span>
                            <span className="group-member-permission">
                                {getPermissionName(getMemberPermissionTypeId())}
                            </span>
                            {canManage && !isSelf ? (
                                <>
                                    <button
                                        className="group-member-update-btn"
                                        onClick={() => {
                                            setSelectedPermission(getMemberPermissionTypeId());
                                            sp = getMemberPermissionTypeId();
                                            setMode(1);
                                        }}
                                    >
                                        Update
                                    </button>
                                    <button
                                        className="group-member-remove-btn"
                                        onClick={() => setShowRemoveModal(true)}
                                    >
                                        Remove
                                    </button>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>
                        <span className="group-member-updated-by">
                            Updated by @{getMemberUpdatedByUsername()}
                        </span>
                    </>
                ) : (
                    <></>
                )}

                {mode === 1 ? (
                    <>
                        <div className="group-member-update-row">
                            <span className="group-member-name">@{member.username}</span>
                            <select
                                className="group-member-select"
                                value={sp}
                                onChange={(e) => {
                                    setSelectedPermission(Number(e.target.value));
                                    sp = Number(e.target.value);
                                }}
                            >
                                {groupPermissionTypes.map(type => (
                                    <option key={type.id} value={type.id}>
                                        {type.permission_type}
                                    </option>
                                ))}
                            </select>
                            {error !== '' ? (
                                <>
                                    <p className="group-info-error">{error}</p>
                                </>
                            ) : (
                                <></>
                            )}
                            <div className="group-member-update-btns">
                                <button
                                    className="group-member-save-btn"
                                    onClick={() => updatePermission()}
                                    disabled={updateLoading}
                                >
                                    {updateLoading ? 'Saving...' : 'Save'}
                                </button>
                                <button
                                    className="group-member-discard-btn"
                                    onClick={() => {
                                        setSelectedPermission(getMemberPermissionTypeId());
                                        sp = getMemberPermissionTypeId();
                                        setMode(0);
                                        setError('');
                                    }}
                                >
                                    Discard
                                </button>
                            </div>
                        </div>
                    </>
                ) : (
                    <></>
                )}

                {showRemoveModal ? (
                    <>
                        <ConfirmModal
                            message={`Are you sure you want to remove @${member.username} from this group?`}
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
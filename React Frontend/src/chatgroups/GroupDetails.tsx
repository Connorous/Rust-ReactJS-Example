import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { Group, GroupPermission } from '../types/group';
import type { ApiResponse } from '../types/api';
import './chatGroups.css';

interface GroupDetailsProps {
    selectedGroupId: number;
    onJoinGroup: (groupId: number) => void;
    onOpenGroup: (groupId: number) => void;
    onBack: () => void;
}

function GroupDetails({ selectedGroupId, onOpenGroup, onBack }: GroupDetailsProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    const [group, setGroup] = useState<Group | null>(null);
    const [permission, setPermission] = useState<GroupPermission | null>(null);
    const [memberCount, setMemberCount] = useState<number>(0);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [requestLoading, setRequestLoading] = useState(false);
    const [requestSuccess, setRequestSuccess] = useState('');
    const [requestError, setRequestError] = useState('');

    useEffect(() => {
        fetchGroupDetails();
        fetchUserPermission();
    }, [selectedGroupId]);

    async function fetchGroupDetails() {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}groups/get-public-group-details`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: selectedGroupId }),
            });

            const data = await res.json();

            if (data.success === true) {
                setGroup(data.data);
                setMemberCount(data.member_count);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function fetchUserPermission() {
        try {
            const res = await fetch(`${API_URL}groups/get-user-permission`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: selectedGroupId }),
            });

            const data: ApiResponse<GroupPermission> = await res.json();

            if (data.success === true) {
                setPermission(data.data!);
            } else {
                setPermission(null);
            }
        } catch (e) {
            console.log('Failed to fetch permission');
        }
    }

    async function sendJoinRequest() {
        setRequestLoading(true);
        setRequestError('');
        setRequestSuccess('');

        try {
            const res = await fetch(`${API_URL}groups/request-to-join-group`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ group_id: selectedGroupId }),
            });

            const data = await res.json();

            if (data.success === true) {
                setRequestSuccess('Join request sent successfully');
                fetchUserPermission();
            } else {
                setRequestError(data.msg);
            }
        } catch (e) {
            setRequestError('Cannot connect to server');
        }

        setRequestLoading(false);
    }

    function getPermissionMessage(): string {
        if (!permission) return '';
        if (permission.permission_type_id === 5) return 'Your request is pending approval';
        if (permission.permission_type_id === 6) return 'Your request was declined';
        if (permission.permission_type_id === 7) return 'You are blocked from this group';
        return '';
    }

    function isMember(): boolean {
        if (!permission) return false;
        return permission.permission_type_id <= 4;
    }

    if (loading) {
        return (
            <>
                <div className="group-details-page">
                    <p className="group-details-loading">Loading...</p>
                </div>
            </>
        );
    }

    if (error !== '') {
        return (
            <>
                <div className="group-details-page">
                    <p className="group-details-error">{error}</p>
                    <button
                        className="group-details-back-btn"
                        onClick={() => onBack()}
                    >
                        ← Back
                    </button>
                </div>
            </>
        );
    }

    if (!group) {
        return (
            <>
                <div className="group-details-page">
                    <p className="group-details-empty">Group not found</p>
                    <button
                        className="group-details-back-btn"
                        onClick={() => onBack()}
                    >
                        ← Back
                    </button>
                </div>
            </>
        );
    }

    return (
        <>
            <div className="group-details-page">
                <button
                    className="group-details-back-btn"
                    onClick={() => onBack()}
                >
                    ← Back
                </button>

                <div className="group-details-card">
                    <h2 className="group-details-name">{group.name}</h2>
                    <p className="group-details-members">{memberCount} members</p>
                    {group.is_public ? (
                        <>
                            <span className="group-details-tag">Public</span>
                        </>
                    ) : (
                        <></>
                    )}

                    {requestSuccess !== '' ? (
                        <>
                            <p className="group-details-success">{requestSuccess}</p>
                        </>
                    ) : (
                        <></>
                    )}

                    {requestError !== '' ? (
                        <>
                            <p className="group-details-error">{requestError}</p>
                        </>
                    ) : (
                        <></>
                    )}

                    {isMember() ? (
                        <>
                            <button
                                className="group-details-open-btn"
                                onClick={() => onOpenGroup(selectedGroupId)}
                            >
                                Open Group
                            </button>
                        </>
                    ) : (
                        <></>
                    )}

                    {!permission ? (
                        <>
                            <button
                                className="group-details-join-btn"
                                onClick={() => sendJoinRequest()}
                                disabled={requestLoading}
                            >
                                {requestLoading ? 'Sending...' : 'Send Join Request'}
                            </button>
                        </>
                    ) : (
                        <></>
                    )}

                    {permission && !isMember() ? (
                        <>
                            <p className="group-details-permission-msg">
                                {getPermissionMessage()}
                            </p>
                        </>
                    ) : (
                        <></>
                    )}
                </div>
            </div>
        </>
    );
}

export default GroupDetails;

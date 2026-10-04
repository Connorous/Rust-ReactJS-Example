import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModel';
import type { Relationship } from '../types/relationship';
import './manage-users.css';

interface UserRelationshipDetailProps {
    relationship: Relationship;
    onBack: () => void;
    onRelationshipUpdated: (rel: Relationship) => void;
    onRelationshipDeleted: () => void;
}

function UserRelationshipDetail({
    relationship,
    onBack,
    onRelationshipUpdated,
    onRelationshipDeleted,
}: UserRelationshipDetailProps) {
    const { accessToken, API_URL, relationshipStatuses } = useContext(AuthContext);

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [, setDeleteLoading] = useState(false);

    function getStatusName(statusId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < relationshipStatuses.length; i++) {
            if (relationshipStatuses[i].id === statusId) {
                name = relationshipStatuses[i].status;
                break;
            }
        }
        return name;
    }

    function getStatusClass(statusId: number): string {
        if (statusId === 2) return 'badge badge-green';
        if (statusId === 3) return 'badge badge-red';
        if (statusId === 1) return 'badge badge-blue';
        return 'badge badge-gray';
    }

    async function declineRelationship() {
        setLoading(true);
        setError('');

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
                const updated: Relationship = {
                    ...relationship,
                    status_id: 4,
                };
                onRelationshipUpdated(updated);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function deleteRelationship() {
        setDeleteLoading(true);

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
                onRelationshipDeleted();
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
            <div className="user-detail-page">
                <button
                    className="manage-back-btn"
                    onClick={() => onBack()}
                >
                    ← Back
                </button>

                <div className="manage-card">
                    <h3 className="manage-section-title">Relationship Detail</h3>

                    <div className="user-info-row">
                        <span className="user-info-label">Requester</span>
                        <span className="user-info-value">@{relationship.requester_username}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Receiver</span>
                        <span className="user-info-value">@{relationship.receiver_username}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Status</span>
                        <span className={getStatusClass(relationship.status_id)}>
                            {getStatusName(relationship.status_id)}
                        </span>
                    </div>
                    {relationship.status_id === 3 ? (
                        <div className="user-info-row">
                            <span className="user-info-label">Blocked by</span>
                            <span className="user-info-value">
                                @{relationship.blocked_by === relationship.requester_id
                                    ? relationship.requester_username
                                    : relationship.receiver_username}
                            </span>
                        </div>
                    ) : (<></>)}
                    {relationship.status_id === 4 ? (
                        <div className="user-info-row">
                            <span className="user-info-label">Declined by</span>
                            <span className="user-info-value">
                                @{relationship.declined_by === relationship.requester_id
                                    ? relationship.requester_username
                                    : relationship.receiver_username}
                            </span>
                        </div>
                    ) : (<></>)}

                    {error !== '' ? (
                        <p className="manage-error">{error}</p>
                    ) : (<></>)}

                    <div className="manage-btn-row">
                        {relationship.status_id !== 4 ? (
                            <>
                                <button
                                    className="manage-decline-btn"
                                    onClick={() => declineRelationship()}
                                    disabled={loading}
                                >
                                    {loading ? '...' : 'Decline'}
                                </button>
                            </>
                        ) : (<></>)}
                        <button
                            className="manage-delete-btn"
                            onClick={() => setShowDeleteModal(true)}
                        >
                            Delete
                        </button>
                    </div>
                </div>

                {showDeleteModal ? (
                    <>
                        <ConfirmModal
                            message="Are you sure you want to delete this relationship?"
                            onConfirm={() => deleteRelationship()}
                            onCancel={() => setShowDeleteModal(false)}
                        />
                    </>
                ) : (<></>)}
            </div>
        </>
    );
}

export default UserRelationshipDetail;
import { useState, useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import UserInfo from './UserInfo';
import UserRelationshipList from './UserRelationshipList';
import UserRelationshipDetail from './UserRelationship';
import UserGroupList from './UserGroupList';
import type { UserRow, UserInformation } from '../types/user';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import './manage-users.css';

interface UserDetailsProps {
    selectedUser: UserRow;
    onBack: () => void;
    onUserUpdated: (user: UserRow) => void;
    onUserDeleted: () => void;
}

function UserDetails({ selectedUser, onBack, onUserUpdated, onUserDeleted }: UserDetailsProps) {
    const { accessToken, API_URL, userTypes, accountStatuses } = useContext(AuthContext);
    const { setLastSelectedGroup } = useContext(UIContext);
    const navigate = useNavigate();

    // 0 = info, 1 = relationships, 2 = groups, 3 = relationship detail
    const [innerView, setInnerView] = useState(0);
    const [selectedRelationship, setSelectedRelationship] = useState<Relationship | null>(null);
    const [userInformation, setUserInformation] = useState<UserInformation | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    useEffect(() => {
        fetchUserInformation(selectedUser.id);
    }, [selectedUser.id]);

    async function fetchUserInformation(userId: number) {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}users/admin/user/get`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_id: userId }),
            });

            const data: ApiResponse<UserInformation> = await res.json();

            if (data.success === true) {
                setUserInformation(data.data!);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    function getUserTypeName(userTypeId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < userTypes.length; i++) {
            if (userTypes[i].id === userTypeId) {
                name = userTypes[i].type;
                break;
            }
        }
        return name;
    }

    function getAccountStatusName(accountStatusId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < accountStatuses.length; i++) {
            if (accountStatuses[i].id === accountStatusId) {
                name = accountStatuses[i].status;
                break;
            }
        }
        return name;
    }

    function getAccountStatusClass(accountStatusId: number): string {
        if (accountStatusId === 1) return 'badge badge-green';
        if (accountStatusId === 2) return 'badge badge-red';
        return 'badge badge-gray';
    }

    function handleSelectGroup(groupId: number) {
        setLastSelectedGroup(groupId);
        navigate('/chat-groups');
    }

    function handleSelectRelationship(rel: Relationship) {
        setSelectedRelationship(rel);
        setInnerView(3);
    }

    function handleUserUpdated(updatedInfo: UserInformation) {
        setUserInformation(updatedInfo);
        // Update the UserRow passed up to ManageUsers
        const updatedRow: UserRow = {
            ...selectedUser,
            username: updatedInfo.username,
            email: updatedInfo.email,
            name: updatedInfo.name,
            user_type_id: updatedInfo.user_type_id,
            account_status_id: updatedInfo.account_status_id,
        };
        onUserUpdated(updatedRow);
    }

    return (
        <>
            <div className="user-details-page">

                {/* Always visible header — uses UserRow */}
                <div className="user-details-header">
                    <button
                        className="manage-back-btn"
                        onClick={() => onBack()}
                    >
                        ← Back
                    </button>

                    <div className="user-details-info">
                        <div className="user-details-name-row">
                            <h2 className="user-details-name">{selectedUser.name}</h2>
                            <span className="user-details-username">@{selectedUser.username}</span>
                            <span className={selectedUser.is_online ? 'online-dot online' : 'online-dot offline'} />
                        </div>
                        <p className="user-details-email">{selectedUser.email}</p>
                        <div className="user-details-tags">
                            <span className={selectedUser.user_type_id <= 2 ? 'badge badge-blue' : 'badge badge-gray'}>
                                {getUserTypeName(selectedUser.user_type_id)}
                            </span>
                            <span className={getAccountStatusClass(selectedUser.account_status_id)}>
                                {getAccountStatusName(selectedUser.account_status_id)}
                            </span>
                        </div>
                    </div>
                </div>

                {/* Nav buttons */}
                <div className="user-details-nav">
                    <button
                        className={innerView === 0 ? 'user-details-nav-btn active' : 'user-details-nav-btn'}
                        onClick={() => setInnerView(0)}
                    >
                        User Info
                    </button>
                    <button
                        className={innerView === 1 || innerView === 3 ? 'user-details-nav-btn active' : 'user-details-nav-btn'}
                        onClick={() => setInnerView(1)}
                    >
                        Relationships
                    </button>
                    <button
                        className={innerView === 2 ? 'user-details-nav-btn active' : 'user-details-nav-btn'}
                        onClick={() => setInnerView(2)}
                    >
                        Groups
                    </button>
                </div>

                {/* Inner views */}
                {innerView === 0 ? (
                    <>
                        {loading ? (
                            <>
                                <p className="manage-empty">Loading...</p>
                            </>
                        ) : (
                            <></>
                        )}
                        {error !== '' ? (
                            <>
                                <p className="manage-error">{error}</p>
                            </>
                        ) : (
                            <></>
                        )}
                        {userInformation !== null ? (
                            <>
                                <UserInfo
                                    userInformation={userInformation}
                                    onUserUpdated={(info) => handleUserUpdated(info)}
                                    onUserDeleted={() => onUserDeleted()}
                                />
                            </>
                        ) : (
                            <></>
                        )}
                    </>
                ) : (
                    <></>
                )}

                {innerView === 1 ? (
                    <>
                        <UserRelationshipList
                            selectedUser={selectedUser}
                            onSelectRelationship={(rel) => handleSelectRelationship(rel)}
                        />
                    </>
                ) : (
                    <></>
                )}

                {innerView === 2 ? (
                    <>
                        <UserGroupList
                            selectedUser={selectedUser}
                            onSelectGroup={(groupId) => handleSelectGroup(groupId)}
                        />
                    </>
                ) : (
                    <></>
                )}

                {innerView === 3 && selectedRelationship !== null ? (
                    <>
                        <UserRelationshipDetail
                            relationship={selectedRelationship}
                            onBack={() => setInnerView(1)}
                            onRelationshipUpdated={(rel: Relationship) => setSelectedRelationship(rel)}
                            onRelationshipDeleted={() => {
                                setSelectedRelationship(null);
                                setInnerView(1);
                            }}
                        />
                    </>
                ) : (
                    <></>
                )}

            </div>
        </>
    );
}

export default UserDetails;
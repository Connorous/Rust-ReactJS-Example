import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModel';
import type { UserInformation } from '../types/user';
import './manage-users.css';

interface UserInfoProps {
    userInformation: UserInformation;
    onUserUpdated: (user: UserInformation) => void;
    onUserDeleted: () => void;
}

function UserInfo({ userInformation, onUserUpdated, onUserDeleted }: UserInfoProps) {
    const { accessToken, API_URL, userTypes, accountStatuses, userStatuses, showNameChoices, themes } = useContext(AuthContext);

    var [username, setUsername] = useState(userInformation.username);
    var un = username;
    var [email, setEmail] = useState(userInformation.email);
    var em = email;
    var [name, setName] = useState(userInformation.name);
    var nm = name;
    var [userTypeId, setUserTypeId] = useState(userInformation.user_type_id);
    var ut = userTypeId;
    var [accountStatusId, setAccountStatusId] = useState(userInformation.account_status_id);
    var as_ = accountStatusId;

    const [error, setError] = useState('');
    const [success, setSuccess] = useState('');
    const [loading, setLoading] = useState(false);
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [, setDeleteLoading] = useState(false);

    function getThemeName(themeId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < themes.length; i++) {
            if (themes[i].id === themeId) {
                name = themes[i].theme;
                break;
            }
        }
        return name;
    }

    function getShowNameChoiceName(choiceId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < showNameChoices.length; i++) {
            if (showNameChoices[i].id === choiceId) {
                name = showNameChoices[i].choice;
                break;
            }
        }
        return name;
    }

    function getUserStatusName(statusId: number | null): string {
        if (statusId === null) return 'Unknown';
        var name: string = 'Unknown';
        for (var i = 0; i < userStatuses.length; i++) {
            if (userStatuses[i].id === statusId) {
                name = userStatuses[i].status;
                break;
            }
        }
        return name;
    }

    function formatDate(dateStr: string): string {
        var date = new Date(dateStr);
        return date.toLocaleString();
    }

    async function updateUser() {
        setLoading(true);
        setError('');
        setSuccess('');

        try {
            const res = await fetch(`${API_URL}users/user`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    id: userInformation.id,
                    username: un,
                    email: em,
                    name: nm,
                    user_type_id: ut,
                    account_status_id: as_,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setSuccess('User updated successfully');
                const updated: UserInformation = {
                    ...userInformation,
                    username: un,
                    email: em,
                    name: nm,
                    user_type_id: ut,
                    account_status_id: as_,
                };
                onUserUpdated(updated);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function deleteUser() {
        setDeleteLoading(true);

        try {
            const res = await fetch(`${API_URL}users/user`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ id: userInformation.id }),
            });

            const data = await res.json();

            if (data.success === true) {
                onUserDeleted();
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
            <div className="user-info-page">

                {/* Read only display */}
                <div className="manage-card">
                    <h3 className="manage-section-title">Profile Details</h3>

                    <div className="user-info-row">
                        <span className="user-info-label">Bio</span>
                        <span className="user-info-value">{userInformation.bio_info ?? 'No bio'}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Show name as</span>
                        <span className="user-info-value">{getShowNameChoiceName(userInformation.show_name_choice_id)}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Theme</span>
                        <span className="user-info-value">{getThemeName(userInformation.theme_id)}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Dark mode</span>
                        <span className="user-info-value">{userInformation.theme_dark_mode ? 'On' : 'Off'}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Status</span>
                        <span className="user-info-value">{getUserStatusName(userInformation.status_id)}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Created at</span>
                        <span className="user-info-value">{formatDate(userInformation.created_at)}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Updated at</span>
                        <span className="user-info-value">{formatDate(userInformation.updated_at)}</span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Created by</span>
                        <span className="user-info-value">
                            {userInformation.created_by_username !== null
                                ? `@${userInformation.created_by_username}`
                                : 'System'}
                        </span>
                    </div>
                    <div className="user-info-row">
                        <span className="user-info-label">Updated by</span>
                        <span className="user-info-value">
                            {userInformation.updated_by_username !== null
                                ? `@${userInformation.updated_by_username}`
                                : 'System'}
                        </span>
                    </div>

                    <h3 className="manage-section-title">Light Mode Colours</h3>
                    <div className="user-info-colours">
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Primary</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_primary_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_primary_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Secondary</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_secondary_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_secondary_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Accent</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_accent_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_accent_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Sent</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_sent_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_sent_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Received</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_received_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_received_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Text on dark</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_dark_text_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_dark_text_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Text on light</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.light_theme_light_text_colour }} />
                            <span className="user-info-value">{userInformation.light_theme_light_text_colour}</span>
                        </div>
                    </div>

                    <h3 className="manage-section-title">Dark Mode Colours</h3>
                    <div className="user-info-colours">
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Primary</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_primary_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_primary_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Secondary</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_secondary_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_secondary_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Accent</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_accent_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_accent_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Sent</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_sent_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_sent_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Received</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_received_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_received_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Text on dark</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_dark_text_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_dark_text_colour}</span>
                        </div>
                        <div className="user-info-colour-row">
                            <span className="user-info-label">Text on light</span>
                            <div className="user-info-colour-swatch" style={{ backgroundColor: userInformation.dark_theme_light_text_colour }} />
                            <span className="user-info-value">{userInformation.dark_theme_light_text_colour}</span>
                        </div>
                    </div>
                </div>

                {/* Edit form */}
                <div className="manage-card">
                    <h3 className="manage-section-title">Edit User</h3>

                    <div className="manage-field">
                        <label className="manage-label">Username</label>
                        <input
                            className="manage-input"
                            type="text"
                            value={un}
                            onChange={(e) => {
                                setUsername(e.target.value);
                                un = e.target.value;
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        <label className="manage-label">Email</label>
                        <input
                            className="manage-input"
                            type="email"
                            value={em}
                            onChange={(e) => {
                                setEmail(e.target.value);
                                em = e.target.value;
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        <label className="manage-label">Name</label>
                        <input
                            className="manage-input"
                            type="text"
                            value={nm}
                            onChange={(e) => {
                                setName(e.target.value);
                                nm = e.target.value;
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        <label className="manage-label">User Type</label>
                        <select
                            className="manage-select"
                            value={ut}
                            onChange={(e) => {
                                setUserTypeId(Number(e.target.value));
                                ut = Number(e.target.value);
                            }}
                        >
                            {userTypes.map(type => (
                                <option key={type.id} value={type.id}>
                                    {type.type}
                                </option>
                            ))}
                        </select>
                    </div>

                    <div className="manage-field">
                        <label className="manage-label">Account Status</label>
                        <select
                            className="manage-select"
                            value={as_}
                            onChange={(e) => {
                                setAccountStatusId(Number(e.target.value));
                                as_ = Number(e.target.value);
                            }}
                        >
                            {accountStatuses.map(status => (
                                <option key={status.id} value={status.id}>
                                    {status.status}
                                </option>
                            ))}
                        </select>
                    </div>

                    {error !== '' ? (
                        <p className="manage-error">{error}</p>
                    ) : (<></>)}

                    {success !== '' ? (
                        <p className="manage-success">{success}</p>
                    ) : (<></>)}

                    <div className="manage-btn-row">
                        <button
                            className="manage-save-btn"
                            onClick={() => updateUser()}
                            disabled={loading}
                        >
                            {loading ? 'Saving...' : 'Save Changes'}
                        </button>
                        <button
                            className="manage-delete-btn"
                            onClick={() => setShowDeleteModal(true)}
                        >
                            Delete User
                        </button>
                    </div>
                </div>

                {showDeleteModal ? (
                    <>
                        <ConfirmModal
                            message={`Are you sure you want to delete @${userInformation.username}? This cannot be undone.`}
                            onConfirm={() => deleteUser()}
                            onCancel={() => setShowDeleteModal(false)}
                        />
                    </>
                ) : (<></>)}
            </div>
        </>
    );
}

export default UserInfo;
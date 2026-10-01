import { useState, useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import type { User } from '../types/user';
import type { Relationship } from '../types/relationship';
import type { ApiResponse } from '../types/api';
import './profile.css';

function Profile() {
    const {
        sessionUser,
        setSessionUser,
        accessToken,
        themes,
        showNameChoices,
        API_URL,
    } = useContext(AuthContext);

    const { lastSelectedProfileId } = useContext(UIContext);

    const navigate = useNavigate();

    const isOwnProfile = lastSelectedProfileId === null || lastSelectedProfileId === sessionUser?.id;

    const [profileUser, setProfileUser] = useState<User | null>(null);
    const [relationship, setRelationship] = useState<Relationship | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    // Profile fields
     const [username, setUsername] = useState('');
    const [name, setName] = useState('');
    const [bio, setBio] = useState('');
    const [showNameChoiceId, setShowNameChoiceId] = useState<number>(1);
    const [themeId, setThemeId] = useState<number>(1);
    const [darkMode, setDarkMode] = useState(false);


    // Light colours
    const [lightPrimary, setLightPrimary] = useState('');
    const [lightSecondary, setLightSecondary] = useState('');
    const [lightAccent, setLightAccent] = useState('');
    const [lightSent, setLightSent] = useState('');
    const [lightReceived, setLightReceived] = useState('');
    const [lightTextOnDark, setLightTextOnDark] = useState('');
    const [lightTextOnLight, setLightTextOnLight] = useState('');

    // Dark colours
    const [darkPrimary, setDarkPrimary] = useState('');
    const [darkSecondary, setDarkSecondary] = useState('');
    const [darkAccent, setDarkAccent] = useState('');
    const [darkSent, setDarkSent] = useState('');
    const [darkReceived, setDarkReceived] = useState('');
    const [darkTextOnDark, setDarkTextOnDark] = useState('');
    const [darkTextOnLight, setDarkTextOnLight] = useState('');

    const [updateError, setUpdateError] = useState('');
    const [updateSuccess, setUpdateSuccess] = useState('');
    const [updateLoading, setUpdateLoading] = useState(false);

    var us: string = username;
    var nm: string = name;
    var bi: string = bio;
    var snc: number = showNameChoiceId;
    var th: number = themeId;
    var dm: boolean = darkMode;

    var lp: string = lightPrimary;
    var ls: string = lightSecondary;
    var la: string = lightAccent;
    var lsn: string = lightSent;
    var lr: string = lightReceived;
    var ltd: string = lightTextOnDark;
    var ltl: string = lightTextOnLight;

    var dp: string = darkPrimary;
    var ds: string = darkSecondary;
    var da: string = darkAccent;
    var dsn: string = darkSent;
    var dr: string = darkReceived;
    var dtd: string = darkTextOnDark;
    var dtl: string = darkTextOnLight;

    useEffect(() => {
        if (isOwnProfile) {
            setProfileUser(sessionUser);
            populateFieldsWithCurrentUserDetails(sessionUser!);
        } else if (lastSelectedProfileId !== null) {
            fetchUserProfile(lastSelectedProfileId);
            fetchRelationship(lastSelectedProfileId);
        }
    }, [lastSelectedProfileId]);

    useEffect(() => {
        if (isOwnProfile && sessionUser) {
            setProfileUser(sessionUser);
            if (!updateLoading) {
                populateFieldsWithCurrentUserDetails(sessionUser);
            }
        }
    }, [sessionUser]);

    function populateFieldsWithCurrentUserDetails(user: User) {
        setUsername(user.username);
        setName(user.name);
        setBio(user.bio_info ?? '');
        setThemeId(user.theme_id);
        setDarkMode(user.theme_dark_mode);
        setShowNameChoiceId(user.show_name_choice_id);
        setLightPrimary(user.light_theme_primary_colour);
        setLightSecondary(user.light_theme_secondary_colour);
        setLightAccent(user.light_theme_accent_colour);
        setLightSent(user.light_theme_sent_colour);
        setLightReceived(user.light_theme_received_colour);
        setLightTextOnDark(user.light_theme_dark_text_colour);
        setLightTextOnLight(user.light_theme_light_text_colour);
        setDarkPrimary(user.dark_theme_primary_colour);
        setDarkSecondary(user.dark_theme_secondary_colour);
        setDarkAccent(user.dark_theme_accent_colour);
        setDarkSent(user.dark_theme_sent_colour);
        setDarkReceived(user.dark_theme_received_colour);
        setDarkTextOnDark(user.dark_theme_dark_text_colour);
        setDarkTextOnLight(user.dark_theme_light_text_colour);
    }

    function reset() {
        if (sessionUser) {
            populateFieldsWithCurrentUserDetails(sessionUser);
        }
        setUpdateError('');
        setUpdateSuccess('');
    }

    async function fetchUserProfile(id: number) {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}users/user/get`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ id: id }),
            });

            const data: ApiResponse<User> = await res.json();

            if (data.success === true) {
                setProfileUser(data.data!);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function fetchRelationship(id: number) {
        try {
            const res = await fetch(`${API_URL}relationships/get/relationship`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_id: id }),
            });

            const data: ApiResponse<Relationship> = await res.json();

            if (data.success === true) {
                setRelationship(data.data ?? null);
            } else {
                setRelationship(null);
            }
        } catch (e) {
            console.log('Failed to fetch relationship');
        }
    }

    function canViewProfile(): boolean {
        if (!relationship) return false;

        // Declined — cannot view regardless of who declined
        if (relationship.status_id === 4 && relationship.declined_by !== sessionUser?.id) return false;

        // Blocked by the other user — cannot view
        if (relationship.status_id === 3 && relationship.blocked_by !== sessionUser?.id) return false;

        // You blocked them — can still view
        if (relationship.status_id === 3 && relationship.blocked_by === sessionUser?.id) return true;

        // Pending or accepted — can view
        if (relationship.status_id === 1 || relationship.status_id === 2) return true;

        return false;
    }

    function isFriend(): boolean {
        return relationship?.status_id === 2;
    }

    function shouldShowRealName(): boolean {
        if (!profileUser) return false;
        if (profileUser.show_name_choice_id === 3) return true;
        if (profileUser.show_name_choice_id === 2 && isFriend()) return true;
        return false;
    }

    function validateColours(): boolean {
        const hexRegex = /^#[0-9A-Fa-f]{6}$/;
        const colours = [
            lightPrimary, lightSecondary, lightAccent,
            lightSent, lightReceived, lightTextOnDark, lightTextOnLight,
            darkPrimary, darkSecondary, darkAccent,
            darkSent, darkReceived, darkTextOnDark, darkTextOnLight,
        ];

        for (let i = 0; i < colours.length; i++) {
            if (colours[i].trim() !== '' && !hexRegex.test(colours[i])) {
                setUpdateError('Colours must be valid hex codes e.g. #ffffff');
                return false;
            }
        }

        return true;
    }

    function validateName() {
        if (username === '' || name === '') {
            setUpdateError('Username or Name Cannot be Empty');
            return false;
        }
        
        return true;
    }

    async function updateProfile() {
        if (validateColours() === false) return;
        if (validateName() === false) return;

        setUpdateLoading(true);
        setUpdateError('');
        setUpdateSuccess('');

        try {
            const res = await fetch(`${API_URL}users/profile`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    username: username,
                    name: name,
                    bio_info: bio,
                    theme_id: themeId,
                    theme_dark_mode: darkMode,
                    show_name_choice_id: showNameChoiceId,
                    light_theme_primary_colour: lightPrimary,
                    light_theme_secondary_colour: lightSecondary,
                    light_theme_accent_colour: lightAccent,
                    light_theme_sent_colour: lightSent,
                    light_theme_received_colour: lightReceived,
                    light_theme_dark_text_colour: lightTextOnDark,
                    light_theme_light_text_colour: lightTextOnLight,
                    dark_theme_primary_colour: darkPrimary,
                    dark_theme_secondary_colour: darkSecondary,
                    dark_theme_accent_colour: darkAccent,
                    dark_theme_sent_colour: darkSent,
                    dark_theme_received_colour: darkReceived,
                    dark_theme_dark_text_colour: darkTextOnDark,
                    dark_theme_light_text_colour: darkTextOnLight,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                if (sessionUser) {
                    const updatedUser: User = {
                        ...sessionUser,
                        username: username,
                        name: name,
                        bio_info: bio || null,
                        theme_id: themeId,
                        theme_dark_mode: darkMode,
                        show_name_choice_id: showNameChoiceId,
                        light_theme_primary_colour: lightPrimary,
                        light_theme_secondary_colour: lightSecondary,
                        light_theme_accent_colour: lightAccent,
                        light_theme_sent_colour: lightSent,
                        light_theme_received_colour: lightReceived,
                        light_theme_dark_text_colour: lightTextOnDark,
                        light_theme_light_text_colour: lightTextOnLight,
                        dark_theme_primary_colour: darkPrimary,
                        dark_theme_secondary_colour: darkSecondary,
                        dark_theme_accent_colour: darkAccent,
                        dark_theme_sent_colour: darkSent,
                        dark_theme_received_colour: darkReceived,
                        dark_theme_dark_text_colour: darkTextOnDark,
                        dark_theme_light_text_colour: darkTextOnLight,
                    };
                    setSessionUser(updatedUser);
                }
                setUpdateSuccess('Profile updated successfully');
            } else {
                setUpdateError(data.msg);
            }
        } catch (e) {
            setUpdateError('Cannot connect to server');
        }

        setUpdateLoading(false);
    }

    if (loading) {
        return (
            <>
                <div className="profile-page">
                    <p className="profile-loading">Loading...</p>
                </div>
            </>
        );
    }

    if (error !== '') {
        return (
            <>
                <div className="profile-page">
                    <p className="profile-error">{error}</p>
                </div>
            </>
        );
    }

    if (!profileUser) {
        return (
            <>
                <div className="profile-page">
                    <p className="profile-error">No profile selected</p>
                </div>
            </>
        );
    }

    // Viewing another user's profile
    if (!isOwnProfile) {
        if (!relationship) {
            return (
                <>
                    <div className="profile-page">
                        <p className="profile-restricted">
                            You do not have a relationship with this user.
                        </p>
                    </div>
                </>
            );
        }

        if (!canViewProfile()) {
            return (
                <>
                    <div className="profile-page">
                        <p className="profile-restricted">
                            {relationship.status_id === 4
                                ? 'This relationship has been declined.'
                                : 'You cannot view this user\'s profile as they have blocked you.'
                            }
                        </p>
                    </div>
                </>
            );
        }

        return (
            <>
                <div className="profile-page">
                    <div className="profile-header">
                        <div className="profile-avatar">
                            {profileUser.username}
                        </div>
                        <div className="profile-header-info">
                            <h1 className="profile-name">@{profileUser.username}</h1>
                            {shouldShowRealName() ? (
                                <>
                                    <p className="profile-realname">{profileUser.name}</p>
                                </>
                            ) : (
                                <></>
                            )}
                            <div className="profile-status">
                                <div className={profileUser.is_online ? 'profile-status-dot online' : 'profile-status-dot offline'} />
                                <span>{profileUser.is_online ? 'Online' : 'Offline'}</span>
                            </div>
                        </div>
                    </div>

                    {profileUser.bio_info ? (
                        <>
                            <div className="profile-section">
                                <h3 className="profile-section-title">About</h3>
                                <p className="profile-bio">{profileUser.bio_info}</p>
                            </div>
                        </>
                    ) : (
                        <></>
                    )}
                </div>
            </>
        );
    }

    // Own profile
    return (
        <>
            <div className="profile-page">
                <div className="profile-header">
                    <div className="profile-avatar">
                        {sessionUser!.username}
                    </div>
                    <div className="profile-header-info">
                        <h1 className="profile-name">@{sessionUser!.username}</h1>
                        <p className="profile-realname">{sessionUser!.name}</p>
                        <p className="profile-email">{sessionUser!.email}</p>
                        <div className="profile-status">
                            <div className={sessionUser!.is_online ? 'profile-status-dot online' : 'profile-status-dot offline'} />
                            <span>{sessionUser!.is_online ? 'Online' : 'Offline'}</span>
                        </div>
                    </div>
                </div>

                <div className="profile-section">
                        <label className="profile-label">Username</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#ffffff"
                            value={us}
                            onChange={(e) => setUsername(e.target.value)}
                        />
                </div>

                <div className="profile-section">
                        <label className="profile-label">Name</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#ffffff"
                            value={nm}
                            onChange={(e) => setName(e.target.value)}
                        />
                </div>

                <div className="profile-section">
                    <h3 className="profile-section-title">Bio</h3>
                    <textarea
                        className="profile-textarea"
                        placeholder="Tell people about yourself..."
                        value={bi}
                        onChange={(e) => setBio(e.target.value)}
                        rows={3}
                    />
                </div>

                <div className="profile-section">
                    <h3 className="profile-section-title">Show Real Name</h3>
                    <select
                        className="profile-select"
                        value={snc}
                        onChange={(e) => setShowNameChoiceId(Number(e.target.value))}
                    >
                        {showNameChoices.map(choice => (
                            <option key={choice.id} value={choice.id}>
                                {choice.choice}
                            </option>
                        ))}
                    </select>
                </div>

                <div className="profile-section">
                    <h3 className="profile-section-title">Theme</h3>
                    <select
                        className="profile-select"
                        value={th}
                        onChange={(e) => setThemeId(Number(e.target.value))}
                    >
                        {themes.map(theme => (
                            <option key={theme.id} value={theme.id}>
                                {theme.theme}
                            </option>
                        ))}
                    </select>

                    <div className="profile-toggle-row">
                        <label className="profile-label">Dark Mode</label>
                        <input
                            type="checkbox"
                            checked={dm}
                            onChange={(e) => setDarkMode(e.target.checked)}
                        />
                    </div>
                </div>

                <div className="profile-section">
                    <h3 className="profile-section-title">Light Mode Colours</h3>

                    <div className="profile-colour-row">
                        <label className="profile-label">Primary Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#ffffff"
                            value={lp}
                            onChange={(e) => setLightPrimary(e.target.value)}
                        />
                        {lightPrimary !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightPrimary }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Secondary Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#f0f0f0"
                            value={ls}
                            onChange={(e) => setLightSecondary(e.target.value)}
                        />
                        {lightSecondary !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightSecondary }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Accent</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#0066ff"
                            value={la}
                            onChange={(e) => setLightAccent(e.target.value)}
                        />
                        {lightAccent !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightAccent }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Sent Message Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#0066ff"
                            value={lsn}
                            onChange={(e) => setLightSent(e.target.value)}
                        />
                        {lightSent !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightSent }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Received Message Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#e0e0e0"
                            value={lr}
                            onChange={(e) => setLightReceived(e.target.value)}
                        />
                        {lightReceived !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightReceived }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Text on Light Backgrounds</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#000000"
                            value={ltl}
                            onChange={(e) => setLightTextOnLight(e.target.value)}
                        />
                        {lightTextOnLight !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightTextOnLight }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Text on Dark Backgrounds</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#ffffff"
                            value={ltd}
                            onChange={(e) => setLightTextOnDark(e.target.value)}
                        />
                        {lightTextOnDark !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: lightTextOnDark }} />
                        ) : (
                            <></>
                        )}
                    </div>
                </div>

                <div className="profile-section">
                    <h3 className="profile-section-title">Dark Mode Colours</h3>

                    <div className="profile-colour-row">
                        <label className="profile-label">Primary Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#1a1a1a"
                            value={dp}
                            onChange={(e) => setDarkPrimary(e.target.value)}
                        />
                        {darkPrimary !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkPrimary }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Secondary Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#2d2d2d"
                            value={ds}
                            onChange={(e) => setDarkSecondary(e.target.value)}
                        />
                        {darkSecondary !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkSecondary }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Accent</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#0066ff"
                            value={da}
                            onChange={(e) => setDarkAccent(e.target.value)}
                        />
                        {darkAccent !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkAccent }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Sent Message Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#0066ff"
                            value={dsn}
                            onChange={(e) => setDarkSent(e.target.value)}
                        />
                        {darkSent !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkSent }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Received Message Background</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#3d3d3d"
                            value={dr}
                            onChange={(e) => setDarkReceived(e.target.value)}
                        />
                        {darkReceived !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkReceived }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Text on Light Backgrounds</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#000000"
                            value={dtl}
                            onChange={(e) => setDarkTextOnLight(e.target.value)}
                        />
                        {darkTextOnLight !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkTextOnLight }} />
                        ) : (
                            <></>
                        )}
                    </div>

                    <div className="profile-colour-row">
                        <label className="profile-label">Text on Dark Backgrounds</label>
                        <input
                            className="profile-colour-input"
                            type="text"
                            placeholder="#ffffff"
                            value={dtd}
                            onChange={(e) => setDarkTextOnDark(e.target.value)}
                        />
                        {darkTextOnDark !== '' ? (
                            <div className="profile-colour-preview" style={{ backgroundColor: darkTextOnDark }} />
                        ) : (
                            <></>
                        )}
                    </div>
                </div>

                {updateError !== '' ? (
                    <>
                        <p className="profile-error">{updateError}</p>
                    </>
                ) : (
                    <></>
                )}

                {updateSuccess !== '' ? (
                    <>
                        <p className="profile-success">{updateSuccess}</p>
                    </>
                ) : (
                    <></>
                )}

                <div className="profile-btn-row">
                    <button
                        className="profile-save-btn"
                        onClick={() => updateProfile()}
                        disabled={updateLoading}
                    >
                        {updateLoading ? 'Saving...' : 'Save Changes'}
                    </button>
                    <button
                        className="profile-reset-btn"
                        onClick={() => reset()}
                    >
                        Reset
                    </button>
                </div>

                <button
                    className="profile-logout-btn"
                    onClick={() => navigate('/logout')}
                >
                    Logout
                </button>

            </div>
        </>
    );
}

export default Profile;
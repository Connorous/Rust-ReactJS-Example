import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import './manage-users.css';

interface NewUserProps {
    onBack: () => void;
    onUserCreated: () => void;
}

function NewUser({ onBack, onUserCreated }: NewUserProps) {
    const { accessToken, API_URL, userTypes } = useContext(AuthContext);

    var [username, setUsername] = useState('');
    var un = username;
    var [email, setEmail] = useState('');
    var em = email;
    var [name, setName] = useState('');
    var nm = name;
    var [password, setPassword] = useState('');
    var pw = password;
    var [confirmPassword, setConfirmPassword] = useState('');
    var cpw = confirmPassword;
    var [userTypeId, setUserTypeId] = useState(3);
    var ut = userTypeId;

    const [usernameError, setUsernameError] = useState('');
    const [emailError, setEmailError] = useState('');
    const [nameError, setNameError] = useState('');
    const [passwordError, setPasswordError] = useState('');
    const [confirmPasswordError, setConfirmPasswordError] = useState('');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);

    function validate(): boolean {
        var valid = true;

        if (un.trim() === '') {
            setUsernameError('Username cannot be blank');
            valid = false;
        } else {
            setUsernameError('');
        }

        if (em.trim() === '') {
            setEmailError('Email cannot be blank');
            valid = false;
        } else {
            setEmailError('');
        }

        if (nm.trim() === '') {
            setNameError('Name cannot be blank');
            valid = false;
        } else {
            setNameError('');
        }

        if (pw.trim() === '') {
            setPasswordError('Password cannot be blank');
            valid = false;
        } else {
            setPasswordError('');
        }

        if (cpw.trim() === '') {
            setConfirmPasswordError('Confirm password cannot be blank');
            valid = false;
        } else if (cpw !== pw) {
            setConfirmPasswordError('Passwords do not match');
            valid = false;
        } else {
            setConfirmPasswordError('');
        }

        return valid;
    }

    async function createUser() {
        if (validate() === false) return;

        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}users/user/new`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    username: un,
                    email: em,
                    name: nm,
                    password: pw,
                    user_type_id: ut,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                onUserCreated();
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    return (
        <>
            <div className="new-user-page">
                <button
                    className="manage-back-btn"
                    onClick={() => onBack()}
                >
                    ← Back
                </button>

                <h2 className="manage-title">New User</h2>

                <div className="manage-card">
                    <div className="manage-field">
                        {usernameError !== '' ? (
                            <p className="manage-field-error">{usernameError}</p>
                        ) : (<></>)}
                        <label className="manage-label">Username</label>
                        <input
                            className="manage-input"
                            type="text"
                            placeholder="Enter username"
                            value={un}
                            onChange={(e) => {
                                setUsername(e.target.value);
                                un = e.target.value;
                                if (e.target.value.trim() !== '') setUsernameError('');
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        {emailError !== '' ? (
                            <p className="manage-field-error">{emailError}</p>
                        ) : (<></>)}
                        <label className="manage-label">Email</label>
                        <input
                            className="manage-input"
                            type="email"
                            placeholder="Enter email"
                            value={em}
                            onChange={(e) => {
                                setEmail(e.target.value);
                                em = e.target.value;
                                if (e.target.value.trim() !== '') setEmailError('');
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        {nameError !== '' ? (
                            <p className="manage-field-error">{nameError}</p>
                        ) : (<></>)}
                        <label className="manage-label">Name</label>
                        <input
                            className="manage-input"
                            type="text"
                            placeholder="Enter name"
                            value={nm}
                            onChange={(e) => {
                                setName(e.target.value);
                                nm = e.target.value;
                                if (e.target.value.trim() !== '') setNameError('');
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        {passwordError !== '' ? (
                            <p className="manage-field-error">{passwordError}</p>
                        ) : (<></>)}
                        <label className="manage-label">Password</label>
                        <input
                            className="manage-input"
                            type="password"
                            placeholder="Enter password"
                            value={pw}
                            onChange={(e) => {
                                setPassword(e.target.value);
                                pw = e.target.value;
                                if (e.target.value.trim() !== '') setPasswordError('');
                            }}
                        />
                    </div>

                    <div className="manage-field">
                        {confirmPasswordError !== '' ? (
                            <p className="manage-field-error">{confirmPasswordError}</p>
                        ) : (<></>)}
                        <label className="manage-label">Confirm Password</label>
                        <input
                            className="manage-input"
                            type="password"
                            placeholder="Confirm password"
                            value={cpw}
                            onChange={(e) => {
                                setConfirmPassword(e.target.value);
                                cpw = e.target.value;
                                if (e.target.value.trim() !== '') setConfirmPasswordError('');
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

                    {error !== '' ? (
                        <p className="manage-error">{error}</p>
                    ) : (<></>)}

                    <div className="manage-btn-row">
                        <button
                            className="manage-save-btn"
                            onClick={() => createUser()}
                            disabled={loading}
                        >
                            {loading ? 'Creating...' : 'Create User'}
                        </button>
                        <button
                            className="manage-cancel-btn"
                            onClick={() => onBack()}
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
}

export default NewUser;
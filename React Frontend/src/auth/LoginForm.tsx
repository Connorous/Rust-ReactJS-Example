import { useState, useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import type { LoginResponse } from '../types/api';
import './login.css';

function Login() {
    const {
        accessToken,
        setAccessToken,
        setSessionUser,
        setThemes,
        setShowNameChoices,
        setUserTypes,
        setUserStatuses,
        setAccountStatuses,
        setRelationshipStatuses,
        setGroupPermissionTypes,
        API_URL,
        SECRET,
    } = useContext(AuthContext);

    const navigate = useNavigate();

    // 0 = login, 1 = register, 2 = reset password
    const [view, setView] = useState(0);

    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [name, setName] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');

    const [usernameError, setUsernameError] = useState('');
    const [emailError, setEmailError] = useState('');
    const [nameError, setNameError] = useState('');
    const [passwordError, setPasswordError] = useState('');
    const [confirmPasswordError, setConfirmPasswordError] = useState('');

    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);


    var un: string = username;
    var em: string = email;
    var nm: string = name;
    var ps: string = password;
    var cps: string = confirmPassword;

    useEffect(() => {
        if (accessToken) {
            navigate('/direct-messages');
        }
    }, [accessToken]);

    function clearFields() {
        setUsername('');
        setEmail('');
        setName('');
        setPassword('');
        setConfirmPassword('');
        setError('');
        setUsernameError('');
        setEmailError('');
        setNameError('');
        setPasswordError('');
        setConfirmPasswordError('');
    }

    function switchView(v: number) {
        clearFields();
        setView(v);
    }

    function validateLogin(): boolean {
        let valid = true;

        if (username.trim() === '') {
            setUsernameError('Username cannot be blank');
            valid = false;
        } else {
            setUsernameError('');
        }

        if (password.trim() === '') {
            setPasswordError('Password cannot be blank');
            valid = false;
        } else {
            setPasswordError('');
        }

        return valid;
    }

    function validateRegister(): boolean {
        let valid = true;

        if (username.trim() === '') {
            setUsernameError('Username cannot be blank');
            valid = false;
        } else {
            setUsernameError('');
        }

        if (email.trim() === '') {
            setEmailError('Email cannot be blank');
            valid = false;
        } else {
            setEmailError('');
        }

        if (name.trim() === '') {
            setNameError('Name cannot be blank');
            valid = false;
        } else {
            setNameError('');
        }

        if (password.trim() === '') {
            setPasswordError('Password cannot be blank');
            valid = false;
        } else {
            setPasswordError('');
        }

        if (confirmPassword.trim() === '') {
            setConfirmPasswordError('Confirm Password cannot be blank');
            valid = false;
        } else if (confirmPassword !== password) {
            setConfirmPasswordError('Passwords do not match');
            valid = false;
        } else {
            setConfirmPasswordError('');
        }

        return valid;
    }

    function validateResetPassword(): boolean {
        let valid = true;

        if (username.trim() === '') {
            setUsernameError('Username cannot be blank');
            valid = false;
        } else {
            setUsernameError('');
        }

        if (email.trim() === '') {
            setEmailError('Email cannot be blank');
            valid = false;
        } else {
            setEmailError('');
        }

        if (password.trim() === '') {
            setPasswordError('Password cannot be blank');
            valid = false;
        } else {
            setPasswordError('');
        }

        if (confirmPassword.trim() === '') {
            setConfirmPasswordError('Confirm Password cannot be blank');
            valid = false;
        } else if (confirmPassword !== password) {
            setConfirmPasswordError('Passwords do Not match, Confirm Password Must match Password');
            valid = false;
        } else {
            setConfirmPasswordError('');
        }

        return valid;
    }


    async function login() {
        if (!validateLogin()) return;

        setLoading(true);

        const settings = {
            method: 'POST',
            headers: {
                Authorization: SECRET,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username: username,
                password: password,
            }),
        };

        try {
            const res = await fetch(`${API_URL}login/login`, settings);
            const data: LoginResponse = await res.json();

            if (data.success === true) {
                setAccessToken(data.access_token!);
                setSessionUser(data.user!);
                setThemes(data.themes!);
                setShowNameChoices(data.show_name_choices!);
                setUserTypes(data.user_types!);
                setUserStatuses(data.user_statuses!);
                setAccountStatuses(data.account_statuses!);
                setRelationshipStatuses(data.relationship_statuses!)
                setGroupPermissionTypes(data.group_permission_types!)
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function register() {
        if (!validateRegister()) return;

        setLoading(true);

        const settings = {
            method: 'POST',
            headers: {
                Authorization: SECRET,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username: username,
                email: email,
                name: name,
                password: password,
            }),
        };

        try {
            const res = await fetch(`${API_URL}login/register`, settings);
            const data = await res.json();

            if (data.success === true) {
                switchView(0);
                setError('Registration successful, please login!');
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function resetPassword() {
        if (!validateResetPassword()) return;

        setLoading(true);

        const settings = {
            method: 'POST',
            headers: {
                Authorization: SECRET,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username: username,
                email: email,
                password: password,
            }),
        };

        try {
            const res = await fetch(`${API_URL}login/reset-password`, settings);
            const data = await res.json();

            if (data.success === true) {
                switchView(0);
                setError('Password reset successful, please login!');
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
            <div className="login-page">
                <div className="login-card">

                    {view === 0 ? (
                        <>
                            <h1 className="login-title">Login</h1>
                            <p className="login-subtitle">Welcome back</p>

                            <div className="login-field">
                                {usernameError !== '' ? (
                                    <span className="login-field-error">{usernameError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Username</label>
                                <input
                                    className="login-input"
                                    type="text"
                                    placeholder="Enter your username"
                                    value={un}
                                    onChange={(e) => {
                                        setUsername(e.target.value);
                                        if (e.target.value.trim() !== '') setUsernameError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {passwordError !== '' ? (
                                    <span className="login-field-error">{passwordError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Password</label>
                                <input
                                    className="login-input"
                                    type="password"
                                    placeholder="Enter your password"
                                    value={ps}
                                    onChange={(e) => {
                                        setPassword(e.target.value);
                                        if (e.target.value.trim() !== '') setPasswordError('');
                                    }}
                                />
                            </div>

                            {error !== '' ? (
                                <p className="login-error">{error}</p>
                            ) : (
                                <></>
                            )}

                            <button
                                className="login-btn-primary"
                                onClick={() => login()}
                                disabled={loading}
                            >
                                {loading ? 'Logging in...' : 'Login'}
                            </button>

                            <button
                                className="login-btn-secondary"
                                onClick={() => switchView(1)}
                            >
                                Don't have an account? Register
                            </button>

                            <button
                                className="login-btn-link"
                                onClick={() => switchView(2)}
                            >
                                Forgot your password?
                            </button>
                        </>
                    ) : (
                        <></>
                    )}

                    {view === 1 ? (
                        <>
                            <h1 className="login-title">Register</h1>
                            <p className="login-subtitle">Create an account</p>

                            <div className="login-field">
                                {usernameError !== '' ? (
                                    <span className="login-field-error">{usernameError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Username</label>
                                <input
                                    className="login-input"
                                    type="text"
                                    placeholder="Choose a username"
                                    value={un}
                                    onChange={(e) => {
                                        setUsername(e.target.value);
                                        if (e.target.value.trim() !== '') setUsernameError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {emailError !== '' ? (
                                    <span className="login-field-error">{emailError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Email</label>
                                <input
                                    className="login-input"
                                    type="email"
                                    placeholder="Enter your email"
                                    value={em}
                                    onChange={(e) => {
                                        setEmail(e.target.value);
                                        if (e.target.value.trim() !== '') setEmailError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {nameError !== '' ? (
                                    <span className="login-field-error">{nameError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Name</label>
                                <input
                                    className="login-input"
                                    type="text"
                                    placeholder="Enter your name"
                                    value={nm}
                                    onChange={(e) => {
                                        setName(e.target.value);
                                        if (e.target.value.trim() !== '') setNameError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {passwordError !== '' ? (
                                    <span className="login-field-error">{passwordError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Password</label>
                                <input
                                    className="login-input"
                                    type="password"
                                    placeholder="Choose a password"
                                    value={ps}
                                    onChange={(e) => {
                                        setPassword(e.target.value);
                                        if (e.target.value.trim() !== '') setPasswordError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {confirmPasswordError !== '' ? (
                                    <span className="login-field-error">{confirmPasswordError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Confirm Password</label>
                                <input
                                    className="login-input"
                                    type="password"
                                    placeholder="Confirm your password"
                                    value={cps}
                                    onChange={(e) => {
                                        setConfirmPassword(e.target.value);
                                        if (e.target.value.trim() !== '') setConfirmPasswordError('');
                                    }}
                                />
                            </div>

                            {error !== '' ? (
                                <p className="login-error">{error}</p>
                            ) : (
                                <></>
                            )}

                            <button
                                className="login-btn-primary"
                                onClick={() => register()}
                                disabled={loading}
                            >
                                {loading ? 'Registering...' : 'Register'}
                            </button>

                            <button
                                className="login-btn-secondary"
                                onClick={() => switchView(0)}
                            >
                                Already have an account? Login
                            </button>
                        </>
                    ) : (
                        <></>
                    )}

                    {view === 2 ? (
                        <>
                            <h1 className="login-title">Reset Password</h1>
                            <p className="login-subtitle">Enter your details to reset</p>

                            <div className="login-field">
                                {usernameError !== '' ? (
                                    <span className="login-field-error">{usernameError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Username</label>
                                <input
                                    className="login-input"
                                    type="text"
                                    placeholder="Enter your username"
                                    value={un}
                                    onChange={(e) => {
                                        setUsername(e.target.value);
                                        if (e.target.value.trim() !== '') setUsernameError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {emailError !== '' ? (
                                    <span className="login-field-error">{emailError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Email</label>
                                <input
                                    className="login-input"
                                    type="email"
                                    placeholder="Enter your email"
                                    value={em}
                                    onChange={(e) => {
                                        setEmail(e.target.value);
                                        if (e.target.value.trim() !== '') setEmailError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {passwordError !== '' ? (
                                    <span className="login-field-error">{passwordError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">New Password</label>
                                <input
                                    className="login-input"
                                    type="password"
                                    placeholder="Enter new password"
                                    value={ps}
                                    onChange={(e) => {
                                        setPassword(e.target.value);
                                        if (e.target.value.trim() !== '') setPasswordError('');
                                    }}
                                />
                            </div>

                            <div className="login-field">
                                {confirmPasswordError !== '' ? (
                                    <span className="login-field-error">{confirmPasswordError}</span>
                                ) : (
                                    <></>
                                )}
                                <label className="login-label">Confirm New Password</label>
                                <input
                                    className="login-input"
                                    type="password"
                                    placeholder="Confirm new password"
                                    value={cps}
                                    onChange={(e) => {
                                        setConfirmPassword(e.target.value);
                                        if (e.target.value.trim() !== '') setConfirmPasswordError('');
                                    }}
                                />
                            </div>

                            {error !== '' ? (
                                <p className="login-error">{error}</p>
                            ) : (
                                <></>
                            )}

                            <button
                                className="login-btn-primary"
                                onClick={() => resetPassword()}
                                disabled={loading}
                            >
                                {loading ? 'Resetting...' : 'Reset Password'}
                            </button>

                            <button
                                className="login-btn-secondary"
                                onClick={() => switchView(0)}
                            >
                                Back to Login
                            </button>
                        </>
                    ) : (
                        <></>
                    )}

                </div>
            </div>
        </>
    );
}

export default Login;
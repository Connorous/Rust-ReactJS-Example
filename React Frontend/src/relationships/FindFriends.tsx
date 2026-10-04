import { useState, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import { UIContext } from '../context/UIContext';
import type { User } from '../types/user';
import type { ApiResponse } from '../types/api';
import './relationships.css';

function FindFriends() {
    const { accessToken, API_URL } = useContext(AuthContext);
    const { setLastSelectedProfileId } = useContext(UIContext);
    const navigate = useNavigate();

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;

    const [results, setResults] = useState<User[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [successMessages, setSuccessMessages] = useState<Record<number, string>>({});
    const [errorMessages, setErrorMessages] = useState<Record<number, string>>({});
    const [sendingRequest, setSendingRequest] = useState<Record<number, boolean>>({});

    async function searchUsers() {
        if (st.trim().length < 2) {
            setError('Search term must be at least 2 characters');
            return;
        }

        setLoading(true);
        setError('');
        setResults([]);

        try {
            const res = await fetch(`${API_URL}relationships/search-non-relationships`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ search: st }),
            });

            const data: ApiResponse<User[]> = await res.json();

            if (data.success === true) {
                setResults(data.data!);
            } else if (data.empty === true) {
                setResults([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    async function sendFriendRequest(userId: number) {
        setSendingRequest(prev => ({ ...prev, [userId]: true }));
        setErrorMessages(prev => ({ ...prev, [userId]: '' }));
        setSuccessMessages(prev => ({ ...prev, [userId]: '' }));

        try {
            const res = await fetch(`${API_URL}relationships/relationship`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ receiver_id: userId }),
            });

            const data = await res.json();

            if (data.success === true) {
                setSuccessMessages(prev => ({ ...prev, [userId]: 'Friend request sent!' }));
                // Remove from results since they now have a relationship
                setResults(prev => {
                    var updated: User[] = [];
                    for (var i = 0; i < prev.length; i++) {
                        if (prev[i].id !== userId) {
                            updated.push(prev[i]);
                        }
                    }
                    return updated;
                });
            } else {
                setErrorMessages(prev => ({ ...prev, [userId]: data.msg }));
            }
        } catch (e) {
            setErrorMessages(prev => ({ ...prev, [userId]: 'Cannot connect to server' }));
        }

        setSendingRequest(prev => ({ ...prev, [userId]: false }));
    }

    function handleViewProfile(userId: number) {
        setLastSelectedProfileId(userId);
        navigate('/profile');
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            searchUsers();
        }
    }

    return (
        <>
            <div className="find-friends-page">
                <h2 className="find-friends-title">Find Friends</h2>

                <div className="find-friends-search">
                    <input
                        className="find-friends-input"
                        type="text"
                        placeholder="Search by username... (min 2 chars)"
                        value={st}
                        onChange={(e) => {
                            setSearchTerm(e.target.value);
                            st = e.target.value;
                            if (e.target.value.trim() !== '') setError('');
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                    />
                    <button
                        className="find-friends-search-btn"
                        onClick={() => searchUsers()}
                        disabled={loading}
                    >
                        {loading ? 'Searching...' : 'Search'}
                    </button>
                </div>

                {error !== '' ? (
                    <>
                        <p className="find-friends-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                {results.length === 0 && !loading && st.trim().length >= 2 ? (
                    <>
                        <p className="find-friends-empty">No users found</p>
                    </>
                ) : (
                    <></>
                )}

                <div className="find-friends-results">
                    {results.map(user => (
                        <div key={user.id} className="find-friends-result">
                            <div className="find-friends-result-info">
                                <button
                                    className="find-friends-username-btn"
                                    onClick={() => handleViewProfile(user.id)}
                                >
                                    @{user.username}
                                </button>
                                {successMessages[user.id] ? (
                                    <>
                                        <p className="find-friends-success">{successMessages[user.id]}</p>
                                    </>
                                ) : (
                                    <></>
                                )}
                                {errorMessages[user.id] ? (
                                    <>
                                        <p className="find-friends-error">{errorMessages[user.id]}</p>
                                    </>
                                ) : (
                                    <></>
                                )}
                            </div>
                            <button
                                className="find-friends-add-btn"
                                onClick={() => sendFriendRequest(user.id)}
                                disabled={sendingRequest[user.id]}
                            >
                                {sendingRequest[user.id] ? '...' : 'Add Friend'}
                            </button>
                        </div>
                    ))}
                </div>
            </div>
        </>
    );
}

export default FindFriends;
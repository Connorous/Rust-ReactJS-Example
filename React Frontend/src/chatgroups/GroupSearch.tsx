import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { Group } from '../types/group';
import type { ApiResponse } from '../types/api';
import './chatGroups.css';

interface GroupSearchProps {
    onSelectGroup: (groupId: number) => void;
}

function GroupSearch({ onSelectGroup }: GroupSearchProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;

    const [results, setResults] = useState<Group[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    async function searchGroups() {
        if (st.trim().length < 2) {
            setError('Search term must be at least 2 characters');
            return;
        }

        setLoading(true);
        setError('');
        setResults([]);

        try {
            const res = await fetch(`${API_URL}groups/search-public-groups`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ search_name: st }),
            });

            const data: ApiResponse<Group[]> = await res.json();

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

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            searchGroups();
        }
    }

    return (
        <>
            <div className="group-search-page">
                <h2 className="group-search-title">Find Groups</h2>

                <div className="group-search-bar">
                    <input
                        className="group-search-input"
                        type="text"
                        placeholder="Search groups... (min 2 chars)"
                        value={st}
                        onChange={(e) => {
                            setSearchTerm(e.target.value);
                            st = e.target.value;
                            if (e.target.value.trim() !== '') setError('');
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                    />
                    <button
                        className="group-search-btn"
                        onClick={() => searchGroups()}
                        disabled={loading}
                    >
                        {loading ? 'Searching...' : 'Search'}
                    </button>
                </div>

                {error !== '' ? (
                    <>
                        <p className="group-search-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                {results.length === 0 && !loading && st.trim().length >= 2 ? (
                    <>
                        <p className="group-search-empty">No groups found</p>
                    </>
                ) : (
                    <></>
                )}

                <div className="group-search-results">
                    {results.map(group => (
                        <div
                            key={group.id}
                            className="group-search-result"
                            onClick={() => onSelectGroup(group.id)}
                        >
                            <span className="group-search-result-name">{group.name}</span>
                            {group.is_public ? (
                                <>
                                    <span className="group-search-result-tag">Public</span>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>
                    ))}
                </div>
            </div>
        </>
    );
}

export default GroupSearch;

import { useState, useContext, useRef } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { Message } from '../types/message';
import type { ApiResponse } from '../types/api';
import './directMessages.css';

interface DMSearchPanelProps {
    relationshipId: number;
    onScrollToMessage: (messageId: number) => void;
    onClose: () => void;
}

function DirectMessageSearchPanel({ relationshipId, onScrollToMessage, onClose }: DMSearchPanelProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    var [searchTerm, setSearchTerm] = useState('');
    var st = searchTerm;

    const [searchResults, setSearchResults] = useState<Message[]>([]);
    const [searchLoading, setSearchLoading] = useState(false);

    const searchTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    function handleSearchChange(value: string) {
        setSearchTerm(value);
        st = value;

        if (searchTimeoutRef.current) {
            clearTimeout(searchTimeoutRef.current);
        }

        if (value.trim().length < 2) {
            setSearchResults([]);
            return;
        }

        searchTimeoutRef.current = setTimeout(() => {
            searchMessages(value);
        }, 300);
    }

    async function searchMessages(term: string) {
        setSearchLoading(true);

        try {
            const res = await fetch(`${API_URL}direct-messages/search`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationshipId,
                    search_term: term,
                }),
            });

            const data: ApiResponse<Message[]> = await res.json();

            if (data.success === true) {
                setSearchResults(data.data!);
            } else if (data.empty === true) {
                setSearchResults([]);
            }
        } catch (e) {
            console.log('Failed to search messages');
        }

        setSearchLoading(false);
    }

    function formatDate(dateStr: string): string {
        var date = new Date(dateStr);
        return date.toLocaleString();
    }

    return (
        <>
            <div className="dm-search-panel">
                <div className="dm-search-header">
                    <h3 className="dm-search-title">Search Messages</h3>
                    <button
                        className="dm-search-close-btn"
                        onClick={() => onClose()}
                    >
                        ✕
                    </button>
                </div>

                <div className="dm-search-section">
                    <input
                        className="dm-search-input"
                        type="text"
                        placeholder="Search messages... (min 2 chars)"
                        value={st}
                        onChange={(e) => handleSearchChange(e.target.value)}
                    />

                    {searchLoading ? (
                        <>
                            <p className="dm-search-loading">Searching...</p>
                        </>
                    ) : (
                        <></>
                    )}

                    {searchResults.length === 0 && st.trim().length >= 2 && !searchLoading ? (
                        <>
                            <p className="dm-search-empty">No messages found</p>
                        </>
                    ) : (
                        <></>
                    )}

                    <div className="dm-search-results">
                        {searchResults.map(result => (
                            <div
                                key={result.id}
                                className="dm-search-result"
                                onClick={() => onScrollToMessage(result.id)}
                            >
                                <p className="dm-search-message">{result.message}</p>
                                <p className="dm-search-time">{formatDate(result.created_at)}</p>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </>
    );
}

export default DirectMessageSearchPanel;
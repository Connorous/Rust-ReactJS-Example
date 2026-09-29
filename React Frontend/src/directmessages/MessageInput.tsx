import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import './directMessages.css';

interface MessageInputProps {
    relationshipId: number;
    onMessageSent: () => void;
}

function MessageInput({ relationshipId, onMessageSent }: MessageInputProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    var [message, setMessage] = useState('');
    var msg = message;

    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);

    async function sendMessage() {
        if (msg.trim() === '') {
            setError('Message cannot be blank');
            return;
        }

        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}direct-messages/message`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    relationship_id: relationshipId,
                    message: msg,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setMessage('');
                msg = '';
                onMessageSent();
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
            sendMessage();
        }
    }

    return (
        <>
            <div className="message-input-container">
                {error !== '' ? (
                    <>
                        <p className="message-input-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}
                <div className="message-input-row">
                    <input
                        className="message-input"
                        type="text"
                        placeholder="Type a message..."
                        value={msg}
                        onChange={(e) => {
                            setMessage(e.target.value);
                            msg = e.target.value;
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                        disabled={loading}
                    />
                    <button
                        className="message-send-btn"
                        onClick={() => sendMessage()}
                        disabled={loading}
                    >
                        {loading ? '...' : 'Send'}
                    </button>
                </div>
            </div>
        </>
    );
}

export default MessageInput;
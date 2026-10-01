import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModel';
import type { Message } from '../types/message';
import type { User } from '../types/user';
import './directMessages.css';

interface MessageViewHolderProps {
    message: Message;
    sessionUser: User;
    relationshipId: number;
    getMessages: () => void;
}

function MessageViewHolder({ message, sessionUser, relationshipId, getMessages }: MessageViewHolderProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    const isOwn = message.sender_id === sessionUser.id;

    // 0 = view, 1 = edit
    const [mode, setMode] = useState(0);
    var [content, setContent] = useState(message.message);
    var ct = content;

    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [updateError, setUpdateError] = useState('');
    const [updateLoading, setUpdateLoading] = useState(false);
    const [deleteLoading, setDeleteLoading] = useState(false);

    function formatDate(dateStr: string): string {
        const date = new Date(dateStr);
        return date.toLocaleString();
    }

    function isEdited(): boolean {
        return message.updated_at !== message.created_at;
    }

    async function updateMessage() {
        if (content.trim() === '') {
            setUpdateError('Message cannot be blank');
            return;
        }

        setUpdateLoading(true);
        setUpdateError('');

        try {
            const res = await fetch(`${API_URL}direct-messages/message`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    message_id: message.id,
                    relationship_id: relationshipId,
                    message: content,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setMode(0);
                getMessages();
            } else {
                setUpdateError(data.msg);
            }
        } catch (e) {
            setUpdateError('Cannot connect to server');
        }

        setUpdateLoading(false);
    }

    async function deleteMessage() {
        setDeleteLoading(true);

        try {
            const res = await fetch(`${API_URL}direct-messages/message`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    message_id: message.id,
                    relationship_id: relationshipId,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                getMessages();
            }
        } catch (e) {
            console.log('Failed to delete message');
        }

        setDeleteLoading(false);
        setShowDeleteModal(false);
    }

    return (
        <>
            <div className={isOwn ? 'message-holder message-own' : 'message-holder message-other'}>

                {mode === 0 ? (
                    <>
                        <div className="message-view">
                            <p className="message-text">{message.message}</p>
                            <div className="message-times">
                                <span className="message-time">
                                    Sent: {formatDate(message.created_at)}
                                </span>
                                {isEdited() ? (
                                    <>
                                        <span className="message-time">
                                            &nbsp;&nbsp;Edited: {formatDate(message.updated_at)}
                                        </span>
                                    </>
                                ) : (
                                    <></>
                                )}
                            </div>
                            {isOwn ? (
                                <>
                                    <div className="message-btn-row">
                                        <button
                                            className="message-edit-btn"
                                            onClick={() => {
                                                setContent(message.message);
                                                setMode(1);
                                            }}
                                        >
                                            Edit
                                        </button>
                                        <button
                                            className="message-delete-btn"
                                            onClick={() => setShowDeleteModal(true)}
                                        >
                                            Delete
                                        </button>
                                    </div>
                                </>
                            ) : (
                                <></>
                            )}
                        </div>
                    </>
                ) : (
                    <></>
                )}

                {mode === 1 ? (
                    <>
                        <div className="message-edit">
                            <input
                                className="message-edit-input"
                                type="text"
                                value={ct}
                                onChange={(e) => {
                                    setContent(e.target.value);
                                }}
                            />
                            {updateError !== '' ? (
                                <>
                                    <p className="message-error">{updateError}</p>
                                </>
                            ) : (
                                <></>
                            )}
                            <div className="message-btn-row">
                                <button
                                    className="message-save-btn"
                                    onClick={() => updateMessage()}
                                    disabled={updateLoading}
                                >
                                    {updateLoading ? 'Saving...' : 'Save'}
                                </button>
                                <button
                                    className="message-discard-btn"
                                    onClick={() => {
                                        setContent(message.message);
                                        setMode(0);
                                        setUpdateError('');
                                    }}
                                >
                                    Discard
                                </button>
                            </div>
                        </div>
                    </>
                ) : (
                    <></>
                )}

                {showDeleteModal ? (
                    <>
                        <ConfirmModal
                            message="Are you sure you want to delete this message?"
                            onConfirm={() => deleteMessage()}
                            onCancel={() => setShowDeleteModal(false)}
                        />
                    </>
                ) : (
                    <></>
                )}

            </div>
        </>
    );
}

export default MessageViewHolder;
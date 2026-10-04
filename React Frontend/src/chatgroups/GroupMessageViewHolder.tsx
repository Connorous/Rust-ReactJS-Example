import { useState, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import ConfirmModal from '../shared/ConfirmModel';
import type { Message } from '../types/message';
import type { User } from '../types/user';
import './chatGroups.css';

interface GroupMessageViewHolderProps {
    message: Message;
    sessionUser: User;
    groupId: number;
    senderUsername: string;
    canDelete: boolean;
    fetchMessages: () => void;
}

function GroupMessageViewHolder({
    message,
    sessionUser,
    groupId,
    senderUsername,
    canDelete,
    fetchMessages,
}: GroupMessageViewHolderProps) {
    const { accessToken, API_URL } = useContext(AuthContext);

    const isOwn = message.sender_id === sessionUser.id;

    var [editText, setEditText] = useState(message.message);
    var et = editText;

    // 0 = view, 1 = edit
    const [mode, setMode] = useState(0);
    const [showDeleteModal, setShowDeleteModal] = useState(false);
    const [updateError, setUpdateError] = useState('');
    const [updateLoading, setUpdateLoading] = useState(false);
    const [, setDeleteLoading] = useState(false);

    function formatDate(dateStr: string): string {
        var date = new Date(dateStr);
        return date.toLocaleString();
    }

    function isEdited(): boolean {
        return message.updated_at !== message.created_at;
    }

    async function saveEdit() {
        if (editText.trim() === '') {
            setUpdateError('Message cannot be blank');
            return;
        }

        setUpdateLoading(true);
        setUpdateError('');

        try {
            const res = await fetch(`${API_URL}groups/message`, {
                method: 'PUT',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    message_id: message.id,
                    group_id: groupId,
                    message: editText,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                setMode(0);
                fetchMessages();
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
            const res = await fetch(`${API_URL}groups/message`, {
                method: 'DELETE',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    message_id: message.id,
                    group_id: groupId,
                }),
            });

            const data = await res.json();

            if (data.success === true) {
                fetchMessages();
            }
        } catch (e) {
            console.log('Failed to delete message');
        }

        setDeleteLoading(false);
        setShowDeleteModal(false);
    }

    return (
        <>
            <div id={'message=${message.id}'} className={isOwn ? 'message-holder message-own' : 'message-holder message-other'}>

                {mode === 0 ? (
                    <>
                        <div className="message-view">
                            <p className="message-sender">@{senderUsername}</p>
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
                            <div className="message-btn-row">
                                {isOwn ? (
                                    <>
                                        <button
                                            className="message-edit-btn"
                                            onClick={() => {
                                                setEditText(message.message);
                                                et = message.message;
                                                setMode(1);
                                            }}
                                        >
                                            Edit
                                        </button>
                                    </>
                                ) : (
                                    <></>
                                )}
                                {canDelete ? (
                                    <>
                                        <button
                                            className="message-delete-btn"
                                            onClick={() => setShowDeleteModal(true)}
                                        >
                                            Delete
                                        </button>
                                    </>
                                ) : (
                                    <></>
                                )}
                            </div>
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
                                value={et}
                                onChange={(e) => {
                                    setEditText(e.target.value);
                                    et = e.target.value;
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
                                    onClick={() => saveEdit()}
                                    disabled={updateLoading}
                                >
                                    {updateLoading ? 'Saving...' : 'Save'}
                                </button>
                                <button
                                    className="message-discard-btn"
                                    onClick={() => {
                                        setEditText(message.message);
                                        et = message.message;
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

export default GroupMessageViewHolder;
import type { Message } from '../types/message';
import type { User } from '../types/user';
import MessageViewHolder from './MessageViewholder';
import './directMessages.css';

interface MessageListProps {
    messages: Message[];
    sessionUser: User;
    relationshipId: number;
    getMessages: () => void;
    messagesEndRef: React.RefObject<HTMLDivElement| null>;
}

function MessageList({ messages, sessionUser, relationshipId, getMessages: getMessages, messagesEndRef }: MessageListProps) {
    if (messages.length === 0) {
        return (
            <>
                <div className="message-list">
                    <p className="message-list-empty">No messages yet, say hello!</p>
                    <div ref={messagesEndRef} />
                </div>
            </>
        );
    }

    return (
        <>
            <div className="message-list">
                {messages.map(message => (
                    <MessageViewHolder
                        key={message.id}
                        message={message}
                        sessionUser={sessionUser}
                        relationshipId={relationshipId}
                        getMessages={getMessages}
                    />
                ))}
                <div ref={messagesEndRef} />
            </div>
        </>
    );
}

export default MessageList;
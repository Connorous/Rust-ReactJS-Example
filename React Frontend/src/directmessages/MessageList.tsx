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
    messageListRef: React.RefObject<HTMLDivElement| null>;
    hasMore: boolean;
    loadingMore: boolean;
    onScrollTop: () => void;
}

function MessageList({ messages, sessionUser, relationshipId, getMessages: getMessages, messagesEndRef, messageListRef, hasMore, loadingMore, onScrollTop  }: MessageListProps) {

    function handleScroll(e: React.UIEvent<HTMLDivElement>) {
        const div = e.currentTarget; 
        if (div.scrollTop < 50 && hasMore && !loadingMore) {
            onScrollTop();
        }
    }
    
    
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
            <div className="message-list" ref={messageListRef} onScroll={(e) => handleScroll(e)}>
                {loadingMore ? (<> <p className='message-list-loading'> Loading More Messages...</p></>) : (<></>)}
                {hasMore ? (<> <p className='message-list-end'> No More Messages</p></>) : (<></>)}

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
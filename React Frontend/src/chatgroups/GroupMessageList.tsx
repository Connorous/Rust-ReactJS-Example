import type { GroupPermission, GroupMember } from '../types/group';
import type { Message } from '../types/message';
import type { User } from '../types/user';
import GroupMessageViewHolder from './GroupMessageViewHolder';
import './chatGroups.css';

interface GroupMessageListProps {
    messages: Message[];
    groupId: number;
    sessionUser: User;
    fetchMessages: () => void;
    messagesEndRef: React.RefObject<HTMLDivElement| null>;
    messageListRef: React.RefObject<HTMLDivElement| null>;
    hasMore: boolean;
    loadingMore: boolean;
    onScrollTop: () => void;
}

function GroupMessageList({
    messages,
    groupId,
    sessionUser,
    fetchMessages,
    messagesEndRef, 
    messageListRef,
    hasMore, 
    loadingMore,
    onScrollTop
}: GroupMessageListProps) {
    function handleScroll(e: React.UIEvent<HTMLDivElement>) {
        const div = e.currentTarget; 
        if (div.scrollTop < 50 && hasMore && !loadingMore) {
            onScrollTop();
        }
    }
    
    
    if (messages.length === 0) {
        return (
            <>
                <div className="group-message-list">
                    <p className="group-message-list-empty">No messages yet, say hello!</p>
                    <div ref={messagesEndRef} />
                </div>
            </>
        );
    }

    return (
        <>
            <div className="group-message-list" ref={messageListRef} onScroll={(e) => handleScroll(e)}>
                {loadingMore ? (<> <p className='group-message-list-loading'> Loading More Messages...</p></>) : (<></>)}
                {hasMore ? (<> <p className='group-message-list-end'> No More Messages</p></>) : (<></>)}

                {messages.map(message => (
                    <GroupMessageViewHolder
                        key={message.id}
                        message={message}
                        sessionUser={sessionUser}
                        groupId={groupId}
                        fetchMessages={fetchMessages}
                    />
                ))}
                <div ref={messagesEndRef} />
            </div>
        </>
    );
}

export default GroupMessageList;
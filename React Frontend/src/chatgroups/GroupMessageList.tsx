import type { GroupPermission, GroupMember } from '../types/group';
import type { User } from '../types/user';
import GroupMemberViewHolder from './GroupMemberViewHolder';
import './chatGroups.css';

interface GroupMemberListProps {
    members: GroupMember[];
    permissions: GroupPermission[];
    groupId: number;
    sessionUser: User;
    groupPermission: GroupPermission;
    onMembersUpdated: () => void;
}

function GroupMemberList({
    members,
    permissions,
    groupId,
    sessionUser,
    groupPermission,
    onMembersUpdated,
}: GroupMemberListProps) {
    if (members.length === 0) {
        return (
            <>
                <p className="group-info-empty">No members found</p>
            </>
        );
    }

    return (
        <>
            <div className="group-member-list">
                {members.map(member => (
                    <GroupMemberViewHolder
                        key={member.id}
                        member={member}
                        permissions={permissions}
                        groupId={groupId}
                        sessionUser={sessionUser}
                        groupPermission={groupPermission}
                        onMembersUpdated={onMembersUpdated}
                    />
                ))}
            </div>
        </>
    );
}

export default GroupMemberList;
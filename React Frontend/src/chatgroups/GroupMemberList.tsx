import type { GroupPermissionRow, GroupMember } from '../types/group';
import type { User } from '../types/user';
import GroupMemberViewHolder from './GroupMemberViewHolder';
import './chatGroups.css';

interface GroupMemberListProps {
    members: GroupMember[];
    permissions: GroupPermissionRow[];
    groupId: number;
    sessionUser: User;
    groupPermission: GroupPermissionRow;
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
                {members.map(member => {
                    var memberPermission: GroupPermissionRow | null = null;

                    for (var i = 0; i < permissions.length; i++) {
                        if (permissions[i].user_id === member.id) {
                            memberPermission = permissions[i];
                            break;
                        }
                    }

                    return (
                        <GroupMemberViewHolder
                            key={member.id}
                            member={member}
                            permission={memberPermission}
                            groupId={groupId}
                            sessionUser={sessionUser}
                            groupPermission={groupPermission}
                            onMembersUpdated={onMembersUpdated}
                        />
                    );
                })}
            </div>
        </>
    );
}

export default GroupMemberList;
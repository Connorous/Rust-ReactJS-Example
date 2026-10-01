import { useState, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import FriendsList from '../relationships/FriendsList';
import RequestsList from '../relationships/RequestsList';
import DMList from '../directmessages/DMList';
import GroupList from '../chatgroups/GroupList';
import UserList from '../users/UserList';
import { UIContext } from '../context/UIContext';

function Sidebar() {
    const { sessionUser, isDesktop } = useContext(AuthContext);
    const { setLastSelectedProfileId } = useContext(UIContext);

    const navigate = useNavigate();

    const [collapsed, setCollapsed] = useState(false);
    const [activeNav, setActiveNav] = useState(5);

    // 0 = none
    // 1 = profile
    // 2 = find friends
    // 3 = friends
    // 4 = requests
    // 5 = dms
    // 6 = groups
    // 7 = admin

    function handleNavClick(nav: number, route: string) {
        if (nav === 1) {
            setLastSelectedProfileId(sessionUser?.id ?? null);
        }
        setActiveNav(nav);
        navigate(route);
      
        if (!isDesktop) {
            setCollapsed(true);
        }
    }

    if (!isDesktop && collapsed) {
        return (
            <>
                <button
                    className="sidebar-arrow"
                    onClick={() => setCollapsed(false)}
                >
                    →
                </button>
            </>
        );
    }

    if (isDesktop && collapsed) {
        return (
            <>
                <div className="sidebar-collapsed">
                    <button
                        className="sidebar-collapse-btn"
                        onClick={() => setCollapsed(false)}
                    >
                        →
                    </button>
                </div>
            </>
        );
    }

    return (
        <>
            <div className={isDesktop ? 'sidebar' : 'sidebar sidebar-mobile'}>

                <button
                    className="sidebar-collapse-btn"
                    onClick={() => setCollapsed(true)}
                >
                    ←
                </button>

                {/* Nav buttons — top 30% */}
                <div className="sidebar-nav">
                    <button
                        className={activeNav === 1 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(1, '/profile')}
                    >
                        👤 Profile
                    </button>
                    <button
                        className={activeNav === 2 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(2, '/find-friends')}
                    >
                        🔍 Find Friends
                    </button>
                    <button
                        className={activeNav === 3 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(3, '/relationships')}
                    >
                        👥 Friends
                    </button>
                    <button
                        className={activeNav === 4 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(4, '/relationships')}
                    >
                        📬 Requests
                    </button>
                    <button
                        className={activeNav === 5 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(5, '/direct-messages')}
                    >
                        💬 Direct Messages
                    </button>
                    <button
                        className={activeNav === 6 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                        onClick={() => handleNavClick(6, '/chat-groups')}
                    >
                        🗨️ Chat Groups
                    </button>

                    {sessionUser && sessionUser.user_type_id <= 2 ? (
                        <>
                            <button
                                className={activeNav === 7 ? 'sidebar-nav-btn active' : 'sidebar-nav-btn'}
                                onClick={() => handleNavClick(7, '/manage-users')}
                            >
                                ⚙️ Manage Users
                            </button>
                        </>
                    ) : (
                        <></>
                    )}
                </div>

                {/* List view — bottom 70% */}
                <div className="sidebar-list">
                    {activeNav === 3 ? (
                        <>
                            <FriendsList />
                        </>
                    ) : (
                        <></>
                    )}
                    {activeNav === 4 ? (
                        <>
                            <RequestsList />
                        </>
                    ) : (
                        <></>
                    )}
                    {activeNav === 5 ? (
                        <>
                            <DMList />
                        </>
                    ) : (
                        <></>
                    )}
                    {activeNav === 6 ? (
                        <>
                            <GroupList />
                        </>
                    ) : (
                        <></>
                    )}
                    {activeNav === 7 ? (
                        <>
                            <UserList />
                        </>
                    ) : (
                        <></>
                    )}
                </div>

            </div>
        </>
    );
}

export default Sidebar;
import "./layout.css";
import { useContext } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';
import LoginForm from '../auth/LoginForm';
import Logout from '../auth/Logout';
import TopBar from './TopBar';
import SideBar from './SideBar';
import ManageUsers from '../users/ManageUsers';
import DirectMessages from '../directMessages/DirectMessages';
import ChatGroups from '../chatGroups/ChatGroups';
import Relationships from '../relationships/Relationships';
import Profile from '../profile/Profile';

function Layout() {
    const { accessToken, sessionUser, isDesktop } = useContext(AuthContext);

    // Not logged in
    if (!accessToken || !sessionUser) {
        return (
            <Routes>
                <Route path="/login" element={<LoginForm />} />
                <Route path="/logout" element={<Logout />} />
                <Route path="*" element={<Navigate to="/login" />} />
            </Routes>
        );
    }

    // Blocked or suspended account
    if (sessionUser.user_type_id === 5 || sessionUser.account_status_id !== 1) {
        return (
            <div className="blocked-page">
                <p>
                    {sessionUser.account_status_id === 2
                        ? 'Your account has been suspended.'
                        : sessionUser.account_status_id === 3
                        ? 'Your account has been closed.'
                        : 'Your account has been blocked.'}
                </p>
                <Routes>
                    <Route path="/logout" element={<Logout />} />
                    <Route path="*" element={<Navigate to="/logout" />} />
                </Routes>
            </div>
        );
    }

    // Logged in — desktop layout
    if (isDesktop) {
        return (
            <div className="app-desktop">
                <TopBar />
                <div className="app-desktop-body">
                    <SideBar />
                    <div className="app-desktop-main">
                        <Routes>
                            <Route path="/" element={<Navigate to="/direct-messages" />} />
                            <Route path="/direct-messages" element={<DirectMessages />} />
                            <Route path="/chat-groups" element={<ChatGroups />} />
                            <Route path="/relationships" element={<Relationships />} />
                            <Route path="/profile" element={<Profile />} />
                            {sessionUser.user_type_id <= 2 && (
                                <Route path="/manage-users" element={<ManageUsers />} />
                            )}
                            <Route path="/logout" element={<Logout />} />
                            <Route path="*" element={<Navigate to="/direct-messages" />} />
                        </Routes>
                    </div>
                </div>
            </div>
        );
    }

    // Logged in — mobile layout
    return (
        <div className="app-mobile">
            <TopBar />
            <div className="app-mobile-main">
                <Routes>
                    <Route path="/" element={<Navigate to="/direct-messages" />} />
                    <Route path="/direct-messages" element={<DirectMessages />} />
                    <Route path="/chat-groups" element={<ChatGroups />} />
                    <Route path="/relationships" element={<Relationships />} />
                    <Route path="/profile" element={<Profile />} />
                    {sessionUser.user_type_id <= 2 && (
                        <Route path="/manage-users" element={<ManageUsers />} />
                    )}
                    <Route path="/logout" element={<Logout />} />
                    <Route path="*" element={<Navigate to="/direct-messages" />} />
                </Routes>
            </div>
        </div>
    );
}

export default Layout;
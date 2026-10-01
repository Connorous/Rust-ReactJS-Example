import { useEffect, useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthContext } from '../context/AuthContext';

function Logout() {
    const {
        accessToken,
        setAccessToken,
        setSessionUser,
        setShowNameChoices,
        setThemes,
        setUserTypes,
        setAccountStatuses,
        setRelationshipStatuses,
        setGroupPermissionTypes,
        clearRefreshTimeout,
        API_URL,
    } = useContext(AuthContext);

    const navigate = useNavigate();

    useEffect(() => {
    async function performLogout() {
        if (accessToken) {
            try {
                await fetch(`${API_URL}users/logout`, {
                    method: 'POST',
                    headers: {
                        Authorization: `Bearer ${accessToken}`,
                        'Content-Type': 'application/json',
                    },
                    credentials: 'include',
                });
            } catch (e) {
                console.log('Logout server call failed');
            }
        }

        clearRefreshTimeout();
        setAccessToken(null);
        setSessionUser(null);
        setThemes([]);
        setUserTypes([]);
        setAccountStatuses([]);
        setRelationshipStatuses([]);
        setGroupPermissionTypes([]);
        setShowNameChoices([]);

        navigate('/login');
    }

    performLogout();
}, []);

    return (<></>);
}

export default Logout;
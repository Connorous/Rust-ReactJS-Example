import { useState, useEffect, useRef } from 'react';
import { useMediaQuery } from 'react-responsive';
import { jwtDecode } from 'jwt-decode';
import { BrowserRouter as Router } from 'react-router-dom';
import { AuthContext } from './context/AuthContext.tsx';
import { WSContext } from './context/WSContext.tsx';
import Layout from './layout/Layout.tsx';
import type { User, Theme, UserType, AccountStatus, UserStatus, ShowNameChoice } from './types/user.ts';
import type { RelationshipStatus } from './types/relationship.ts';
import type { GroupPermissionType } from './types/group.ts';
import { UIProvider } from './context/UIContext.tsx';
//comment
interface JwtPayload {
    sub: string;
    user_id: number;
    user_type_id: number;
    account_status_id: number;
    exp: number;
}

const API_URL = import.meta.env.VITE_API_URL as string;
const SECRET = import.meta.env.VITE_SECRET as string;
const WS_URL = import.meta.env.VITE_WS_URL as string;

function App() {
    const isDesktop = useMediaQuery({ minWidth: 850 });

    // --- AUTH STATE ---
    const [accessToken, setAccessToken] = useState<string | null>(null);

    const [sessionUser, setSessionUser] = useState<User | null>(() => {
        try {
            return JSON.parse(localStorage.getItem('sessionUser') ?? 'null');
        } catch {
            return null;
        }
    });

    const [themes, setThemes] = useState<Theme[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('themes') ?? '[]');
        } catch {
            return [];
        }
    });

    const [userTypes, setUserTypes] = useState<UserType[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('userTypes') ?? '[]');
        } catch {
            return [];
        }
    });

    const [userStatuses, setUserStatuses] = useState<UserStatus[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('userStatuses') ?? '[]');
        } catch {
            return [];
        }
    });

    const [showNameChoices, setShowNameChoices] = useState<ShowNameChoice[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('showNameChoices') ?? '[]');
        } catch {
            return [];
        }
    });

    const [accountStatuses, setAccountStatuses] = useState<AccountStatus[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('accountStatuses') ?? '[]');
        } catch {
            return [];
        }
    });

    const [relationshipStatuses, setRelationshipStatuses] = useState<RelationshipStatus[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('relationshipStatuses') ?? '[]');
        } catch {
            return [];
        }
    });

    const [groupPermissionTypes, setGroupPermissionTypes] = useState<GroupPermissionType[]>(() => {
        try {
            return JSON.parse(localStorage.getItem('groupPermissionTypes') ?? '[]');
        } catch {
            return [];
        }
    });

    useEffect(() => {
        localStorage.setItem('sessionUser', JSON.stringify(sessionUser));
    }, [sessionUser]);

    useEffect(() => {
        localStorage.setItem('themes', JSON.stringify(themes));
    }, [themes]);
    useEffect(() => {
        localStorage.setItem('userTypes', JSON.stringify(userTypes));
    }, [userTypes]);
    useEffect(() => {
        localStorage.setItem('userStatuses', JSON.stringify(userStatuses));
    }, [userStatuses]);
    useEffect(() => {
        localStorage.setItem('showNameChoices', JSON.stringify(showNameChoices));
    }, [showNameChoices]);
    useEffect(() => {
        localStorage.setItem('accountStatuses', JSON.stringify(accountStatuses));
    }, [accountStatuses]);
    useEffect(() => {
        localStorage.setItem('relationshipStatuses', JSON.stringify(relationshipStatuses));
    }, [relationshipStatuses]);
    useEffect(() => {
        localStorage.setItem('groupPermissionTypes', JSON.stringify(groupPermissionTypes));
    }, [groupPermissionTypes]);

    // --- TOKEN REFRESH ---
    const refreshTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    function clearRefreshTimeout() {
      if (refreshTimeoutRef.current) {
        clearTimeout(refreshTimeoutRef.current); 
        refreshTimeoutRef.current = null;
      }
    }

    async function attemptRefresh(): Promise<boolean> {
        try {
            const res = await fetch(`${API_URL}login/refresh`, {
                method: 'POST',
                headers: {
                    Authorization: SECRET,
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
            });

            const data = await res.json();

            if (data.success === true) {
                setAccessToken(data.data);
                return true;
            } else {
                logout();
                return false;
            }
        } catch {
            logout();
            return false;
        }
    }

    // Schedule refresh 1 minute before token expires
    useEffect(() => {
        if (!accessToken) return;

        try {
            const decoded = jwtDecode<JwtPayload>(accessToken);
            const msUntilExpiry = decoded.exp * 1000 - Date.now();
            const msUntilRefresh = msUntilExpiry - 60000;

            if (msUntilRefresh <= 0) {
                attemptRefresh();
                return;
            }

            refreshTimeoutRef.current = setTimeout(() => {
                attemptRefresh();
            }, msUntilRefresh);
        } catch {
            logout();
        }

        return () => {
            clearRefreshTimeout();
        };
    }, [accessToken]);

    // On app load — try to refresh if we have a session user
    useEffect(() => {
        if (sessionUser !== null && accessToken === null) {
            attemptRefresh();
        }
    }, []);

    // --- THEME ---
    useEffect(() => {
        if (!sessionUser) return;

        const colours = sessionUser.theme_dark_mode
            ? {
                primary: sessionUser.dark_theme_primary_colour,
                secondary: sessionUser.dark_theme_secondary_colour,
                accent: sessionUser.dark_theme_accent_colour,
                sentBg: sessionUser.dark_theme_sent_colour,
                receivedBg: sessionUser.dark_theme_received_colour,
                textOnDark: sessionUser.dark_theme_dark_text_colour,
                textOnLight: sessionUser.dark_theme_light_text_colour,
            }
            : {
                primary: sessionUser.light_theme_primary_colour,
                secondary: sessionUser.light_theme_secondary_colour,
                accent: sessionUser.light_theme_accent_colour,
                sentBg: sessionUser.light_theme_sent_colour,
                receivedBg: sessionUser.light_theme_received_colour,
                textOnDark: sessionUser.light_theme_dark_text_colour,
                textOnLight: sessionUser.light_theme_light_text_colour,
            };

        const root = document.documentElement;

        if (colours.primary) root.style.setProperty('--primary', colours.primary);
        if (colours.secondary) root.style.setProperty('--secondary', colours.secondary);
        if (colours.accent) root.style.setProperty('--accent', colours.accent);
        if (colours.sentBg) root.style.setProperty('--sent-bg', colours.sentBg);
        if (colours.receivedBg) root.style.setProperty('--received-bg', colours.receivedBg);
        if (colours.textOnDark) root.style.setProperty('--text-on-dark', colours.textOnDark);
        if (colours.textOnLight) root.style.setProperty('--text-on-light', colours.textOnLight);

        const themeName = themes.find(t => t.id === sessionUser.theme_id)?.theme ?? 'default';
        document.documentElement.setAttribute('data-theme', themeName);

    }, [sessionUser, themes]);


    function logout() {
        fetch(`${API_URL}users/logout`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${accessToken}`,
                'Content-Type': 'application/json',
            },
            credentials: 'include',
        }).catch(() => {});

        setAccessToken(null);
        setSessionUser(null);

        if (refreshTimeoutRef.current) {
            clearTimeout(refreshTimeoutRef.current);
        }
    }

    // --- WEBSOCKET ---
    const wsRef = useRef<WebSocket | null>(null);
    const listenersRef = useRef<Record<string, ((data: unknown) => void)[]>>({});

    useEffect(() => {
        if (!accessToken) {
            wsRef.current?.close();
            wsRef.current = null;
            return;
        }

        const ws = new WebSocket(`${WS_URL}ws?token=${accessToken}`);
        wsRef.current = ws;

        ws.onmessage = (event) => {
            try {
                const { event: eventName, data } = JSON.parse(event.data);

                if (eventName === 'force_logout') {
                    logout();
                    return;
                }

                if (eventName === 'user_updated' && data.user_id === sessionUser?.id) {
                    if (data.user_type === 5 || data.account_status !== 1) {
                        logout();
                        return;
                    }
                    setSessionUser((prev: User | null) => prev ? {
                        ...prev,
                        user_type_id: data.user_type_id,
                        account_status: data.account_status_id,
                        is_online: data.is_online,
                        status_id: data.status_id,
                        } : null);
                        return;
                }
                const callbacks = listenersRef.current[eventName] ?? [];
                callbacks.forEach(cb => cb(data));
            } catch {
                console.error('WS message parse error');
            }
        };

        ws.onclose = () => {
            setTimeout(() => {
                if (accessToken) attemptRefresh();
            }, 5000);
        };

        return () => {
            ws.close();
        };
    }, [accessToken]);

    function subscribe(eventName: string, callback: (data: unknown) => void): () => void {
        if (!listenersRef.current[eventName]) {
            listenersRef.current[eventName] = [];
        }
        listenersRef.current[eventName].push(callback);

        return () => {
            listenersRef.current[eventName] = listenersRef.current[eventName]
                .filter(cb => cb !== callback);
        };
    }

    return (
        <Router>
            <AuthContext.Provider value={{
                accessToken,
                setAccessToken,
                sessionUser,
                setSessionUser,
                themes,
                setThemes,
                userTypes, 
                setUserTypes,
                userStatuses,
                setUserStatuses,
                showNameChoices,
                setShowNameChoices,
                accountStatuses,
                setAccountStatuses,
                relationshipStatuses,
                setRelationshipStatuses,
                groupPermissionTypes,
                setGroupPermissionTypes,
                clearRefreshTimeout,
                logout,
                API_URL,
                SECRET,
                isDesktop,
            }}>
                <WSContext.Provider value={{ subscribe }}>
                  <UIProvider><Layout /></UIProvider>
                </WSContext.Provider>
            </AuthContext.Provider>
        </Router>
    );
}

export default App;
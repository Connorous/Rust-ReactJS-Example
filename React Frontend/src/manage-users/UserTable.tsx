import { useState, useEffect, useContext } from 'react';
import { AuthContext } from '../context/AuthContext';
import type { UserRow } from '../types/user';
import type { ApiResponse } from '../types/api';
import './manage-users.css';

interface UserTableProps {
    onSelectUser: (user: UserRow) => void;
    onNewUser: () => void;
    refreshTrigger: number;
}

function UserTable({ onSelectUser, onNewUser, refreshTrigger }: UserTableProps) {
    const { accessToken, API_URL, isDesktop, userTypes, accountStatuses } = useContext(AuthContext);

    const [users, setUsers] = useState<UserRow[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [page, setPage] = useState(1);
    const [total] = useState(0);

    var [search, setSearch] = useState('');
    var sr = search;

    const [sortColumn, setSortColumn] = useState<string>('username');
    const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');

    const pageSize = isDesktop ? 20 : 10;

    useEffect(() => {
        fetchUsers();
    }, [page, refreshTrigger]);

    async function fetchUsers() {
        setLoading(true);
        setError('');

        try {
            const res = await fetch(`${API_URL}users/list`, {
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${accessToken}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    page: page,
                    page_size: pageSize,
                    search: sr.trim() === '' ? null : sr.trim(),
                }),
            });

            const data: ApiResponse<UserRow[]> = await res.json();

            if (data.success === true) {
                setUsers(data.data!);
                // total from response if backend returns it
                // setTotal(data.total);
            } else if (data.empty === true) {
                setUsers([]);
            } else {
                setError(data.msg);
            }
        } catch (e) {
            setError('Cannot connect to server');
        }

        setLoading(false);
    }

    function handleSearch() {
        setPage(1);
        fetchUsers();
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === 'Enter') {
            handleSearch();
        }
    }

    function handleSortClick(column: string) {
        if (sortColumn === column) {
            setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
        } else {
            setSortColumn(column);
            setSortDirection('asc');
        }
    }

    function getSortedUsers(): UserRow[] {
        var sorted: UserRow[] = [...users];

        sorted.sort((a, b) => {
            var aVal: string = '';
            var bVal: string = '';

            if (sortColumn === 'username') {
                aVal = a.username.toLowerCase();
                bVal = b.username.toLowerCase();
            } else if (sortColumn === 'email') {
                aVal = a.email.toLowerCase();
                bVal = b.email.toLowerCase();
            } else if (sortColumn === 'name') {
                aVal = a.name.toLowerCase();
                bVal = b.name.toLowerCase();
            } else if (sortColumn === 'user_type') {
                aVal = String(a.user_type_id);
                bVal = String(b.user_type_id);
            } else if (sortColumn === 'account_status') {
                aVal = String(a.account_status_id);
                bVal = String(b.account_status_id);
            } else if (sortColumn === 'is_online') {
                aVal = String(a.is_online);
                bVal = String(b.is_online);
            } else if (sortColumn === 'created_at') {
                aVal = a.created_at;
                bVal = b.created_at;
            } else if (sortColumn === 'updated_at') {
                aVal = a.updated_at;
                bVal = b.updated_at;
            }

            if (sortDirection === 'asc') {
                return aVal > bVal ? 1 : -1;
            } else {
                return aVal < bVal ? 1 : -1;
            }
        });

        return sorted;
    }

    function getSortIcon(column: string): string {
        if (sortColumn !== column) return '↕';
        return sortDirection === 'asc' ? '↑' : '↓';
    }

    function getUserTypeName(userTypeId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < userTypes.length; i++) {
            if (userTypes[i].id === userTypeId) {
                name = userTypes[i].type;
                break;
            }
        }
        return name;
    }

    function getAccountStatusName(accountStatusId: number): string {
        var name: string = 'Unknown';
        for (var i = 0; i < accountStatuses.length; i++) {
            if (accountStatuses[i].id === accountStatusId) {
                name = accountStatuses[i].status;
                break;
            }
        }
        return name;
    }

    function formatDate(dateStr: string): string {
        var date = new Date(dateStr);
        return date.toLocaleDateString();
    }

    function getAccountStatusClass(accountStatusId: number): string {
        if (accountStatusId === 1) return 'badge badge-green';
        if (accountStatusId === 2) return 'badge badge-red';
        return 'badge badge-gray';
    }

    function getUserTypeClass(userTypeId: number): string {
        if (userTypeId <= 2) return 'badge badge-blue';
        return 'badge badge-gray';
    }

    const totalPages = Math.ceil(total / pageSize);

    return (
        <>
            <div className="user-table-container">
                <div className="user-table-header">
                    <h2 className="user-table-title">Manage Users</h2>
                    <button
                        className="user-table-new-btn"
                        onClick={() => onNewUser()}
                    >
                        + New User
                    </button>
                </div>

                <div className="user-table-search">
                    <input
                        className="user-table-search-input"
                        type="text"
                        placeholder="Search by username, email or name..."
                        value={sr}
                        onChange={(e) => {
                            setSearch(e.target.value);
                            sr = e.target.value;
                        }}
                        onKeyDown={(e) => handleKeyDown(e)}
                    />
                    <button
                        className="user-table-search-btn"
                        onClick={() => handleSearch()}
                        disabled={loading}
                    >
                        {loading ? 'Searching...' : 'Search'}
                    </button>
                </div>

                {error !== '' ? (
                    <>
                        <p className="user-table-error">{error}</p>
                    </>
                ) : (
                    <></>
                )}

                <div className="user-table-wrap">
                    <table className="user-table">
                        <thead>
                            <tr>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('username')}
                                >
                                    Username {getSortIcon('username')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('email')}
                                >
                                    Email {getSortIcon('email')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('name')}
                                >
                                    Name {getSortIcon('name')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('user_type')}
                                >
                                    User Type {getSortIcon('user_type')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('account_status')}
                                >
                                    Account {getSortIcon('account_status')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('is_online')}
                                >
                                    Online {getSortIcon('is_online')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('created_at')}
                                >
                                    Created At {getSortIcon('created_at')}
                                </th>
                                <th
                                    className="user-table-th"
                                    onClick={() => handleSortClick('updated_at')}
                                >
                                    Updated At {getSortIcon('updated_at')}
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {loading ? (
                                <tr>
                                    <td colSpan={8} className="user-table-loading">
                                        Loading...
                                    </td>
                                </tr>
                            ) : (
                                <></>
                            )}
                            {!loading && users.length === 0 ? (
                                <tr>
                                    <td colSpan={8} className="user-table-empty">
                                        No users found
                                    </td>
                                </tr>
                            ) : (
                                <></>
                            )}
                            {getSortedUsers().map(user => (
                                <tr
                                    key={user.id}
                                    className="user-table-row"
                                    onClick={() => onSelectUser(user)}
                                >
                                    <td className="user-table-td">@{user.username}</td>
                                    <td className="user-table-td">{user.email}</td>
                                    <td className="user-table-td">{user.name}</td>
                                    <td className="user-table-td">
                                        <span className={getUserTypeClass(user.user_type_id)}>
                                            {getUserTypeName(user.user_type_id)}
                                        </span>
                                    </td>
                                    <td className="user-table-td">
                                        <span className={getAccountStatusClass(user.account_status_id)}>
                                            {getAccountStatusName(user.account_status_id)}
                                        </span>
                                    </td>
                                    <td className="user-table-td">
                                        <span className={user.is_online ? 'online-dot online' : 'online-dot offline'} />
                                    </td>
                                    <td className="user-table-td">{formatDate(user.created_at)}</td>
                                    <td className="user-table-td">{formatDate(user.updated_at)}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>

                <div className="user-table-pagination">
                    <span className="user-table-pagination-info">
                        Page {page} {total > 0 ? `of ${totalPages}` : ''}
                    </span>
                    <div className="user-table-pagination-btns">
                        <button
                            className="user-table-page-btn"
                            onClick={() => setPage(prev => Math.max(1, prev - 1))}
                            disabled={page === 1}
                        >
                            ←
                        </button>
                        <span className="user-table-page-num">{page}</span>
                        <button
                            className="user-table-page-btn"
                            onClick={() => setPage(prev => prev + 1)}
                            disabled={users.length < pageSize}
                        >
                            →
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
}

export default UserTable;
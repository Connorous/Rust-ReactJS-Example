import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import UserTable from './UserTable';
import UserDetails from './UserDetails';
import NewUser from './NewUser';
import './manage-users.css';

function ManageUsers({ APIAdress, token, sessionUser }) {
  var [users, setUsers] = useState([]);
  var [userTypes, setUserTypes] = useState([]);

  var [selectedUser, setSelectedUser] = useState(null);
  var [viewTableorUserorAddUser, setviewTableorUserorAddUser] = useState(0);

  var [responseMsg, setResponseMsg] = useState('');
  var [showResponseMsg, setShowResponseMsg] = useState(false);

  var [error, setError] = useState(null);
  var [loading, setLoading] = useState(true);

  useEffect(() => {
    getUsers();
    getUserTypes();
  }, []);

  const navigate = useNavigate();
  useEffect(() => {
    if (!token) {
      navigate('/login');
    }
  }, []);

  useEffect(() => {
    if (sessionUser.user_type_id > 2) {
      navigate('/');
    }
  }, []);

  async function getUserTypes() {
    const settings = {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };
    try {
      const fetchUserTypes = await fetch(
        APIAdress + `users/user-types/` + sessionUser.id,
        settings
      );

      if (!fetchUserTypes.ok && fetchUserTypes.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUserTypes.json();
      if (response.success == true) {
        const usertypes = response.data;
        setUserTypes(usertypes);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getUsers() {
    const settings = {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };

    try {
      const fetchUsers = await fetch(
        APIAdress + `users/list/` + sessionUser.id,
        settings
      );

      if (!fetchUsers.ok && fetchUsers.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUsers.json();
      if (response.success == true) {
        const users = response.data;
        setUsers(users);
        setLoading(false);
        setError('');
      } else {
        setLoading(false);
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function selectUser(user) {
    setSelectedUser(user);
    switchComponent(1);
  }

  async function switchComponent(section) {
    setviewTableorUserorAddUser(section);
  }

  async function showMessage(msg) {
    setResponseMsg(msg);
    setShowResponseMsg(true);
    setTimeout(hideMessage, 5000);
  }

  async function hideMessage() {
    setResponseMsg('');
    setShowResponseMsg(false);
  }

  if (
    users.length > 0 &&
    userTypes.length > 0 &&
    sessionUser.user_type_id <= 2
  ) {
    return (
      <>
        <div className="manage-users">
          <h1 className="manage-user-title">Manage Users</h1>
          <p className="manage-users-error">{error}</p>
          {loading === true ? (
            <>
              <h3 className="manage-users-loading">Loading</h3>
            </>
          ) : (
            <>
              {showResponseMsg ? (
                <>
                  <p>{responseMsg}</p>
                </>
              ) : (
                <></>
              )}

              {viewTableorUserorAddUser === 0 ? (
                <>
                  <UserTable
                    users={users}
                    userTypes={userTypes}
                    selectUser={selectUser}
                    switchComponent={switchComponent}
                  ></UserTable>
                </>
              ) : (
                <></>
              )}
              {viewTableorUserorAddUser === 1 ? (
                <>
                  <UserDetails
                    APIAdress={APIAdress}
                    token={token}
                    sessionUser={sessionUser}
                    selectedUser={selectedUser}
                    userTypes={userTypes}
                    switchComponent={switchComponent}
                    getUsers={getUsers}
                    showMessage={showMessage}
                  ></UserDetails>
                </>
              ) : (
                <></>
              )}
              {viewTableorUserorAddUser === 2 ? (
                <>
                  <NewUser
                    APIAdress={APIAdress}
                    token={token}
                    sessionUser={sessionUser}
                    userTypes={userTypes}
                    switchComponent={switchComponent}
                    getUsers={getUsers}
                    showMessage={showMessage}
                  ></NewUser>
                </>
              ) : (
                <></>
              )}
            </>
          )}
        </div>
      </>
    );
  } else {
    return (
      <>
        <div className="manage-users">
          <h1 className="manage-users-title">Manage Users</h1>
          <p>Error: No Users Could be Retrieved, Check Connection to Server</p>
        </div>
      </>
    );
  }
}

export default ManageUsers;

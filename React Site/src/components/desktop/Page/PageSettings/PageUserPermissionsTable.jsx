import { useState, useEffect } from 'react';
import './page-settings.css';

function PageUserPermissionsTable({
  sessionUser,
  token,
  APIAdress,
  selectedPage,
  userPermissions,
  usersPagePermissions,
  usersWithPagePermissions,
  permissionTypes,
  getAllPagePermissions,
  getAllUsersWithPagePermissions,
}) {
  useEffect(() => {
    if (selectedPage != null && selectedPage != undefined) {
      if (
        sessionUser.user_type_id <= 2 ||
        (sessionUser.user_type_id == 3 &&
          userPermissions.permission_type_id == 1)
      ) {
        getAllUsersWithoutPagePermissions();
      }
    }
  }, []);

  var [usersWithoutPagePermissions, setUsersWithoutPagePermissions] = useState(
    []
  );

  var [selectedPermission, setSelectedPermission] = useState(null);
  var [selectedPermissionType, setSelectedPermissionType] = useState(3);
  var [selectedUser, setSelectedUser] = useState('');
  var [viewTableorAddorUpdate, setviewTableorUserorAddUser] = useState(0);

  var spt = selectedPermissionType;
  var su = selectedUser;

  var [error, setError] = useState(null);

  async function getAllUsersWithoutPagePermissions() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
      }),
    };

    try {
      const fetchPagePermissions = await fetch(
        APIAdress + `page-permissions/all-page-users-without-permissions`,
        settings
      );

      if (!fetchPagePermissions.ok && fetchPagePermissions.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPagePermissions.json();
      if (response.success == true) {
        const users = structuredClone(response.data);

        setUsersWithoutPagePermissions(
          users.map((user) => ({
            ...user,
          }))
        );
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function newPagePermission() {
    if (!validateNewPermissionInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        user_id: Number(selectedUser),
        page_id: Number(selectedPage.id),
        permission_type_id: Number(selectedPermissionType),
      }),
    };

    try {
      const fetchNewPagePermission = await fetch(
        APIAdress + `page-permissions/page-permission`,
        settings
      );

      if (!fetchNewPagePermission.ok && fetchNewPagePermission.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchNewPagePermission.json();
      if (response.success == true) {
        const users = response.data;
        getAllPagePermissions();
        getAllUsersWithPagePermissions();
        getAllUsersWithoutPagePermissions();
        setError('');
        setSelectedUser('');
        setSelectedPermissionType(3);
        switchComponent(0);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function updatePagePermission() {
    if (!validateUpdatePermissionInput()) {
      return;
    }

    const settings = {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        id: Number(selectedPermission),
        session_user_id: Number(sessionUser.id),
        user_id: Number(selectedUser),
        page_id: Number(selectedPage.id),
        permission_type_id: Number(selectedPermissionType),
      }),
    };

    try {
      const fetchUpdatePagePermission = await fetch(
        APIAdress + `page-permissions/page-permission`,
        settings
      );

      if (
        !fetchUpdatePagePermission.ok &&
        fetchUpdatePagePermission.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUpdatePagePermission.json();
      if (response.success == true) {
        const users = response.data;
        getAllPagePermissions();
        getAllUsersWithPagePermissions();
        getAllUsersWithoutPagePermissions();
        setError('');
        setSelectedUser('');
        setSelectedPermissionType(3);
        switchComponent(0);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function deletePagePermission(userPermission) {
    if (!validateDeletePermission(userPermission)) {
      return;
    }

    if (confirm('Are you sure you want to delete this page permission?')) {
    } else {
      return;
    }

    const settings = {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        id: Number(userPermission),
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
      }),
    };

    console.log('body ', settings);

    try {
      const fetchPages = await fetch(
        APIAdress + `page-permissions/page-permission`,
        settings
      );

      if (!fetchPages.ok && fetchPages.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPages.json();
      if (response.success == true) {
        const users = response.data;
        getAllPagePermissions();
        getAllUsersWithPagePermissions();
        setError('');
        setSelectedPermission(null);
        switchComponent(0);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e);
      setError(e);
      return e;
    }
  }

  function grabUsername(user_id) {
    if (
      usersWithPagePermissions != null ||
      usersWithPagePermissions != undefined
    ) {
      for (let i = 0; i < usersWithPagePermissions.length; i++) {
        if (usersWithPagePermissions[i].id == user_id) {
          return usersWithPagePermissions[i].username;
          break;
        }
      }
    }
  }

  function validateNewPermissionInput() {
    for (let i = 0; i < usersWithPagePermissions.length; i++) {
      if (usersWithPagePermissions[i] == selectedUser) {
        break;
        setError('Selected User Already has Page Permission.');
        return false;
      }
    }

    if (selectedUser == sessionUser.id && sessionUser.user_type_id == 3) {
      setError('User Cannot Create a Permission for Themselves.');
    } else if (
      selectedUser == sessionUser.id &&
      sessionUser.user_type_id <= 2
    ) {
      return true;
    } else {
      return true;
    }
  }

  function validateUpdatePermissionInput() {
    if (selectedPermission == null || selectedPermission == undefined) {
      setError('No User Page Permission Selected.');
      return false;
    }

    if (selectedUser == sessionUser.id && sessionUser.user_type_id == 3) {
      setError('User Cannot Update their own Page Permission.');
    } else {
      return true;
    }
  }

  function validateDeletePermission(userPermission) {
    if (userPermission == userPermissions.id && sessionUser.user_type_id > 2) {
      setError('User Cannot Delete their own Page Permission.');
    } else {
      return true;
    }
  }

  function switchComponent(value) {
    setviewTableorUserorAddUser(value);
  }

  function selectPermissionToUpdate(userPermission) {
    setSelectedPermission(userPermission.id);
    setSelectedUser(userPermission.user_id);
    setSelectedPermissionType(userPermission.permission_type_id);
    setviewTableorUserorAddUser(2);
  }

  function selectPermissionToDelete(userPermission) {
    console.log('deleting', userPermission.id);
    setSelectedPermission(null);
    deletePagePermission(userPermission.id);
  }

  function Discard() {
    setSelectedPermission(null);
    setSelectedUser('');
    setSelectedPermissionType(3);
    setviewTableorUserorAddUser(0);
  }

  if (permissionTypes.length > 0 && usersWithPagePermissions.length > 0) {
    if (viewTableorAddorUpdate == 0) {
      return (
        <>
          <div className="settings-section">
            <br></br>
            <h3 className="section-title">Page Permissions</h3>
            <br></br>
            <div className="table-window">
              <p className="settings-error">{error}</p>
              <br></br>
              <button
                className="settings-add"
                onClick={() => switchComponent(1)}
              >
                +
              </button>
              <br></br>
              <br></br>
              <br></br>
              {usersPagePermissions.length > 0 ? (
                <>
                  <table className="permissions-table">
                    <thead>
                      <tr>
                        <th>Username</th>
                        <th>Permission</th>
                        <th></th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      {usersPagePermissions.length === 0 ||
                      permissionTypes.length == 0 ? (
                        <>
                          <tr>
                            <td>Loading table</td>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td></td>
                          </tr>
                        </>
                      ) : (
                        <></>
                      )}

                      {usersPagePermissions.map((userPermission) => (
                        <tr key={userPermission.id}>
                          <td>{grabUsername(userPermission.user_id)}</td>
                          <td>
                            {userPermission.permission_type_id ===
                            permissionTypes[0].id ? (
                              <>{permissionTypes[0].type}</>
                            ) : (
                              <></>
                            )}
                            {userPermission.permission_type_id ===
                            permissionTypes[1].id ? (
                              <>{permissionTypes[1].type}</>
                            ) : (
                              <></>
                            )}
                            {userPermission.permission_type_id ===
                            permissionTypes[2].id ? (
                              <>{permissionTypes[2].type}</>
                            ) : (
                              <></>
                            )}
                            {userPermission.permission_type_id ===
                            permissionTypes[3].id ? (
                              <>{permissionTypes[3].type}</>
                            ) : (
                              <></>
                            )}
                          </td>
                          <td>
                            <button
                              className="settings-update"
                              onClick={() =>
                                selectPermissionToUpdate(userPermission)
                              }
                            >
                              Update
                            </button>
                          </td>
                          <td>
                            <button
                              className="settings-del2"
                              onClick={() =>
                                selectPermissionToDelete(userPermission)
                              }
                            >
                              <svg
                                width="20px"
                                height="20px"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                              >
                                <path
                                  d="M10 11V17"
                                  stroke-width="2"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                />
                                <path
                                  d="M14 11V17"
                                  stroke-width="2"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                />
                                <path
                                  d="M4 7H20"
                                  stroke-width="2"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                />
                                <path
                                  d="M6 7H12H18V18C18 19.6569 16.6569 21 15 21H9C7.34315 21 6 19.6569 6 18V7Z"
                                  stroke-width="2"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                />
                                <path
                                  d="M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z"
                                  stroke-width="2"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                />
                              </svg>
                            </button>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </>
              ) : (
                <></>
              )}

              <br></br>
            </div>
          </div>
        </>
      );
    } else if (viewTableorAddorUpdate == 1) {
      return (
        <>
          <div className="settings-section">
            <br></br>
            <h3 className="section-title">New Page Permission</h3>
            <br></br>
            <div className="edit-window">
              <p className="settings-error">{error}</p>
              <div className="edit-page-details">
                <p>User</p>
                <select
                  className="edit-select-sheet"
                  style={{ background: '#FCFFFF' }}
                  value={su}
                  onChange={(e) => setSelectedUser(e.target.value)}
                >
                  {usersWithoutPagePermissions.map((user) => (
                    <option key={user.id} value={user.id}>
                      {user.username}
                    </option>
                  ))}
                </select>

                <p>Page Permission Type</p>
                <select
                  className="edit-select-sheet"
                  style={{ background: '#FCFFFF' }}
                  value={spt}
                  onChange={(e) => setSelectedPermissionType(e.target.value)}
                >
                  {permissionTypes.map((permissionType) => (
                    <option key={permissionType.id} value={permissionType.id}>
                      {permissionType.type}
                    </option>
                  ))}
                </select>
              </div>
              <br></br>
              <div className="page-details-save-or-discard">
                <button
                  className="settings-save"
                  onClick={() => newPagePermission()}
                >
                  Save
                </button>
                <button className="settings-discard" onClick={() => Discard()}>
                  Discard
                </button>
              </div>
              <br></br>
            </div>
          </div>
        </>
      );
    } else {
      return (
        <>
          <div className="settings-section">
            <br></br>
            <h3 className="section-title">Update Page Permission</h3>
            <br></br>
            <div className="edit-window">
              <p className="settings-error">{error}</p>
              <div className="edit-page-details">
                <p>User</p>
                <p>{grabUsername(selectedUser)}</p>
                <p>Page Permission Type</p>
                <select
                  className="edit-select-sheet"
                  style={{ background: '#FCFFFF' }}
                  value={spt}
                  onChange={(e) => setSelectedPermissionType(e.target.value)}
                >
                  {permissionTypes.map((permissionType) => (
                    <option key={permissionType.id} value={permissionType.id}>
                      {permissionType.type}
                    </option>
                  ))}
                </select>
              </div>
              <br></br>
              <div className="page-details-save-or-discard">
                <button
                  className="settings-save"
                  onClick={() => updatePagePermission()}
                >
                  Save
                </button>
                <button className="settings-discard" onClick={() => Discard()}>
                  Discard
                </button>
              </div>
              <br></br>
            </div>
          </div>
        </>
      );
    }
  }
}

export default PageUserPermissionsTable;

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './manage-users.css';

function UserDetails({
  APIAdress,
  token,
  sessionUser,
  selectedUser,
  userTypes,
  switchComponent,
  getUsers,
  showMessage,
}) {
  var [editing, setEditing] = useState(false);
  var [username, setUsername] = useState(selectedUser.username);
  var [name, setName] = useState(selectedUser.name);
  var [email, setEmail] = useState(selectedUser.email);
  var [userType, setUserType] = useState(selectedUser.user_type_id);

  var [error, setError] = useState(null);

  var un = username;
  var nm = name;
  var em = email;
  var ut = userType;

  const navigate = useNavigate();

  async function discard() {
    setUsername(selectedUser.username);
    setName(selectedUser.name);
    setEmail(selectedUser.email);
    setUserType(selectedUser.user_type_id);
    setEditing(false);
    setError('');
  }

  async function updateUser() {
    if (!validateUpdateInput()) {
      return;
    }

    var shouldLogout = false;

    if (
      selectedUser.id == sessionUser.id &&
      selectedUser.user_type_id != userType
    ) {
      if (userType == 5) {
        if (
          confirm(
            'Are you sure you want to change your own User Type to Block Yourself? Are you Okay?'
          )
        ) {
          shouldLogout = true;
        } else {
          return;
        }
      } else if (userType >= 3) {
        if (
          confirm(
            'Are you sure you want to change your own User Type to below and Admin? Only an Admin can Update you back. You Will Need to Logout and Log back in to use the App.'
          )
        ) {
          shouldLogout = true;
        } else {
          return;
        }
      } else if (selectedUser.user_type_id == 1 && userType == 2) {
        if (
          confirm('Down Grading your own Admin User Type? Daring Today are we?')
        ) {
          shouldLogout = true;
        } else {
          return;
        }
      }
    }

    const settings = {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        admin_id: sessionUser.id,
        id: selectedUser.id,
        username: username,
        name: name,
        email: email,
        user_type_id: Number(userType),
        original_user_type: selectedUser.user_type_id,
      }),
    };
    try {
      const fetchUpdateUser = await fetch(APIAdress + `users/user`, settings);
      const response = await fetchUpdateUser.json();
      if (response.success == true) {
        showMessage('Updated User Successfully!');
        getUsers();
        switchComponent(0);
        setError('');

        if (shouldLogout == true) {
          navigate('/logout');
        }
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function deleteUser() {
    validateDeletePermission();

    if (
      confirm(
        'Are you sure you want to delete this user? This is not Salesforce, you cannot revert this with Undelete. If you do not know what this means you are likely a moron and should cancel and get an adult.'
      )
    ) {
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
        admin_id: sessionUser.id,
        id: selectedUser.id,
      }),
    };

    try {
      const fetchDeleteUser = await fetch(APIAdress + `users/user`, settings);
      const response = await fetchDeleteUser.json();
      if (response.success == true) {
        showMessage('Deleted User Successfully!');
        getUsers();
        switchComponent(0);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function validateUpdateInput() {
    if (email == '' || username == '' || name == '') {
      setError('No fields can be blank.');
      return false;
    } else {
      return true;
    }
  }

  function validateDeletePermission() {
    if (selectedUser.id == sessionUser.id && sessionUser.user_type_id == 3) {
      setError('User Cannot Delete their own User Account.');
    } else {
      return true;
    }
  }

  return (
    <>
      {editing ? (
        <>
          <h2 className="manage-user-title">
            Editing Details of {selectedUser.username}
          </h2>
        </>
      ) : (
        <>
          <h2 className="manage-user-title">
            Viewing Details for {selectedUser.username}
          </h2>
        </>
      )}

      <p className="manage-users-error">{error}</p>
      <br></br>
      {editing ? (
        <>
          <div>
            <div className="manage-users-user-details">
              <p>Username</p>
              {selectedUser.username !== username && username != '' ? (
                <>
                  <input
                    style={{ background: '#FCFFFF' }}
                    type="text"
                    placeholder="ryanis5hort"
                    name="username"
                    required
                    value={un}
                    onChange={(e) => {
                      setUsername(e.target.value);
                    }}
                  ></input>
                </>
              ) : (
                <>
                  {username === '' ? (
                    <>
                      <input
                        style={{ background: '#FFE5E5' }}
                        type="text"
                        placeholder="ryanis5hort"
                        name="username"
                        required
                        value={un}
                        onChange={(e) => {
                          setUsername(e.target.value);
                        }}
                      ></input>
                    </>
                  ) : (
                    <>
                      <input
                        type="text"
                        placeholder="ryanis5hort"
                        name="username"
                        required
                        value={un}
                        onChange={(e) => {
                          setUsername(e.target.value);
                        }}
                      ></input>
                    </>
                  )}
                </>
              )}
              <p>Name</p>
              {selectedUser.name !== name && name != '' ? (
                <>
                  <input
                    style={{ background: '#FCFFFF' }}
                    type="text"
                    placeholder="Ryan Transing (yes full name please, or not I'm not your life coach)"
                    name="name"
                    required
                    value={nm}
                    onChange={(e) => {
                      setName(e.target.value);
                    }}
                  ></input>
                </>
              ) : (
                <>
                  {name === '' ? (
                    <>
                      <input
                        style={{ background: '#FFE5E5' }}
                        type="text"
                        placeholder="Ryan Transing (yes full name please, or not I'm not your life coach)"
                        name="name"
                        required
                        value={nm}
                        onChange={(e) => {
                          setName(e.target.value);
                        }}
                      ></input>
                    </>
                  ) : (
                    <>
                      <input
                        type="text"
                        placeholder="Ryan Transing (yes full name please, or not I'm not your life coach)"
                        name="name"
                        required
                        value={nm}
                        onChange={(e) => {
                          setName(e.target.value);
                        }}
                      ></input>
                    </>
                  )}
                </>
              )}
              <p>Email</p>
              {selectedUser.email !== email && email !== '' ? (
                <>
                  <input
                    style={{ background: '#FCFFFF' }}
                    type="email"
                    placeholder="example@mail.com"
                    name="name"
                    required
                    value={em}
                    onChange={(e) => {
                      setEmail(e.target.value);
                    }}
                  ></input>
                </>
              ) : (
                <>
                  {email === '' ? (
                    <>
                      <input
                        style={{ background: '#FFE5E5' }}
                        type="email"
                        placeholder="example@mail.com"
                        name="name"
                        required
                        value={em}
                        onChange={(e) => {
                          setEmail(e.target.value);
                        }}
                      ></input>
                    </>
                  ) : (
                    <>
                      <input
                        type="email"
                        placeholder="example@mail.com"
                        name="name"
                        required
                        value={em}
                        onChange={(e) => {
                          setEmail(e.target.value);
                        }}
                      ></input>
                    </>
                  )}
                </>
              )}

              <p>User Type</p>
              {selectedUser.user_type_id === Number(userType) ? (
                <>
                  <select
                    value={ut}
                    onChange={(e) => setUserType(e.target.value)}
                  >
                    {userTypes.map((userType) => (
                      <option key={userType.id} value={userType.id}>
                        {userType.type}
                      </option>
                    ))}
                  </select>
                </>
              ) : (
                <>
                  <select
                    style={{ background: '#FCFFFF' }}
                    value={ut}
                    onChange={(e) => setUserType(e.target.value)}
                  >
                    {userTypes.map((userType) => (
                      <option key={userType.id} value={userType.id}>
                        {userType.type}
                      </option>
                    ))}
                  </select>
                </>
              )}
            </div>
            <br></br>
            <div className="manage-users-edit-or-delete">
              <button
                className="manage-users-discard"
                onClick={() => {
                  discard();
                }}
              >
                Discard
              </button>
              <button
                className="manage-users-save"
                onClick={() => {
                  updateUser();
                }}
              >
                Save
              </button>
            </div>
            <br></br>
            <br></br>
            <br></br>
            <br></br>
            <br></br>
          </div>
        </>
      ) : (
        <>
          <div className="manage-users-user-details">
            <p>Name</p>
            <p>{selectedUser.name}</p>

            <p>Email</p>
            <p>{selectedUser.email}</p>
            {selectedUser.user_type_id === userTypes[0].id ? (
              <>
                <p>User Type</p>
                <p>{userTypes[0].type}</p>
              </>
            ) : (
              <></>
            )}
            {selectedUser.user_type_id === userTypes[1].id ? (
              <>
                <p>User Type</p>
                <p>{userTypes[1].type}</p>
              </>
            ) : (
              <></>
            )}
            {selectedUser.user_type_id === userTypes[2].id ? (
              <>
                <p>User Type</p>
                <p>{userTypes[2].type}</p>
              </>
            ) : (
              <></>
            )}
            {selectedUser.user_type_id === userTypes[3].id ? (
              <>
                <p>User Type</p>
                <p>{userTypes[3].type}</p>
              </>
            ) : (
              <></>
            )}
            {selectedUser.user_type_id === userTypes[4].id ? (
              <>
                <p>User Type</p>
                <p>{userTypes[4].type}</p>
              </>
            ) : (
              <></>
            )}
          </div>
          <br></br>
          <br></br>
          <div className="manage-users-edit-or-delete">
            <button
              className="manage-users-edit"
              onClick={() => setEditing(true)}
            >
              <svg
                width="800px"
                height="800px"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M8.56078 20.2501L20.5608 8.25011L15.7501 3.43945L3.75012 15.4395V20.2501H8.56078ZM15.7501 5.56077L18.4395 8.25011L16.5001 10.1895L13.8108 7.50013L15.7501 5.56077ZM12.7501 8.56079L15.4395 11.2501L7.93946 18.7501H5.25012L5.25012 16.0608L12.7501 8.56079Z"
                />
              </svg>
            </button>
            <button className="manage-users-del" onClick={() => deleteUser()}>
              <svg
                width="800px"
                height="800px"
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
          </div>
          <br></br>
          <br></br>
          <br></br>
          <br></br>
          <button
            className="manage-users-back"
            onClick={() => {
              switchComponent(0);
            }}
          >
            <svg
              width="800px"
              height="800px"
              viewBox="0 0 32 32"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
            >
              <g id="icomoon-ignore"></g>
              <path d="M14.389 7.956v4.374l1.056 0.010c7.335 0.071 11.466 3.333 12.543 9.944-4.029-4.661-8.675-4.663-12.532-4.664h-1.067v4.337l-9.884-7.001 9.884-7zM15.456 5.893l-12.795 9.063 12.795 9.063v-5.332c5.121 0.002 9.869 0.26 13.884 7.42 0-4.547-0.751-14.706-13.884-14.833v-5.381z"></path>
            </svg>
          </button>
        </>
      )}
    </>
  );
}

export default UserDetails;

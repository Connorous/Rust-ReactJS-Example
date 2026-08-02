import { useState } from 'react';
import './manage-users.css';

function UserDetails({
  APIAdress,
  token,
  sessionUser,
  userTypes,
  switchComponent,
  getUsers,
  showMessage,
}) {
  var [username, setUsername] = useState('');
  var [name, setName] = useState('');
  var [email, setEmail] = useState('');
  var [password, setPassword] = useState('');
  var [userType, setUserType] = useState(1);

  var [error, setError] = useState(null);

  var un = username;
  var nm = name;
  var em = email;
  var ps = password;
  var ut = userType;

  async function newUser() {
    if (!validateInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        admin_id: sessionUser.id,
        username: String(username),
        name: String(name),
        email: String(email),
        password: String(password),
        user_type_id: Number(userType),
      }),
    };

    try {
      const fetchNewUser = await fetch(APIAdress + `users/user`, settings);

      if (!fetchNewUser.ok && fetchNewUser.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchNewUser.json();
      if (response.msg == 'Success') {
        showMessage('Created User Successfully!');
        getUsers();
        switchComponent(0);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function discard() {
    setUsername('');
    setName('');
    setEmail('');
    setPassword('');
    setUserType(0);
    switchComponent(0);
    setError('');
  }

  function validateInput() {
    console.log(name);
    if (email == '' || username == '' || name == '' || password == '') {
      setError('No fields can be blank.');
      return false;
    } else {
      return true;
    }
  }

  return (
    <>
      <div>
        <h2 className="manage-user-title">Create New User</h2>

        <p className="manage-users-error">{error}</p>
        <br></br>
        <div className="manage-users-user-details">
          <p>Username</p>
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
          <p>Name</p>
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
          <p>Email</p>
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
          <p>Password</p>
          {password === '' ? (
            <>
              <input
                style={{ background: '#FFE5E5' }}
                type="password"
                placeholder="Ryan's dog"
                name="name"
                required
                value={ps}
                onChange={(e) => {
                  setPassword(e.target.value);
                }}
              ></input>
            </>
          ) : (
            <>
              <input
                type="password"
                placeholder="Ryan's dog"
                name="name"
                required
                value={ps}
                onChange={(e) => {
                  setPassword(e.target.value);
                }}
              ></input>
            </>
          )}

          <p>User Type</p>
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
              newUser();
            }}
          >
            Save
          </button>
        </div>
        <br></br>
        <br></br>
        <br></br>
      </div>
    </>
  );
}

export default UserDetails;

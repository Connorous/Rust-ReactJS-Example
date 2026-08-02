import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import './login-styles.css';

function LoginForm({
  APIAdress,
  token,
  setToken,
  sessionUser,
  setSessionUser,
}) {
  const [switchRegLog, setSwitchRegLog] = useState(0);

  var [email, setEmail] = useState('');
  var [username, setUsername] = useState('');
  var [name, setName] = useState('');
  var [password, setPassword] = useState('');
  var [confirmpassword, setConfirmPassword] = useState('');

  var [emailError, setEmailError] = useState('');
  var [usernameError, setUsernameError] = useState('');
  var [nameError, setNameError] = useState('');
  var [passwordError, setPasswordError] = useState('');
  var [confirmPasswordError, setConfirmPasswordError] = useState('');

  var [error, setError] = useState(null);
  const SECRET = import.meta.env.VITE_SECRET;

  var em = email;
  var us = username;
  var nm = name;
  var ps = password;
  var cnps = confirmpassword;

  function changeRegLog(value) {
    setSwitchRegLog(value);
    setEmail('');
    setUsername('');
    setName('');
    setPassword('');
    setConfirmPassword('');
    setError('');

    setEmailError('');
    setUsernameError('');
    setNameError('');
    setPasswordError('');
    setConfirmPasswordError('');
  }

  function submit() {
    if (!switchRegLog) {
      login();
    } else {
      register();
    }
  }

  async function login() {
    if (!validateLoginInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `${SECRET}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: String(username),
        password: String(password),
      }),
    };

    try {
      const fetchLogin = await fetch(APIAdress + `login/login-user`, settings);
      const response = await fetchLogin.json();

      if (response.success === true) {
        setToken(response.token);
        setSessionUser(response.user);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function register() {
    if (!validateRegisterInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `${SECRET}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: String(username),
        name: String(name),
        email: String(email),
        password: String(password),
      }),
    };

    try {
      const fetchRegisterUser = await fetch(
        APIAdress + `login/register-user`,
        settings
      );

      const response = await fetchRegisterUser.json();
      if (response.success === true) {
        changeRegLog(0);
        setError('Now that you have registered, login!');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function resetPassword() {
    if (!validateResetPasswordInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `${SECRET}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: String(username),
        email: String(email),
        password: String(password),
      }),
    };
    console.log(settings);
    try {
      const fetchResetPassword = await fetch(
        APIAdress + `login/reset-user-password`,
        settings
      );

      const response = await fetchResetPassword.json();
      if (response.success === true) {
        changeRegLog(0);
        setError('Password Reset, try loggining in again.');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function validateLoginInput() {
    if (username == '' || password == '') {
      setError(
        'No fields can be blank, you must provide Username and Password to login.'
      );
      return false;
    } else {
      return true;
    }
  }

  function validateRegisterInput() {
    if (
      email == '' ||
      username == '' ||
      name == '' ||
      password == '' ||
      confirmpassword == ''
    ) {
      setError('No fields can be blank dummy, do not even try it.');
      return false;
    } else if (password != confirmpassword) {
      setError('Password and Confirm Password do not match.');
      return false;
    } else {
      return true;
    }
  }

  function validateResetPasswordInput() {
    if (
      email == '' ||
      username == '' ||
      password == '' ||
      confirmpassword == ''
    ) {
      setError('No fields can be blank, sorry it is just how it is.');
      return false;
    } else if (password != confirmpassword) {
      setError('Password and Confirm Password do not match.');
      return false;
    } else {
      return true;
    }
  }

  const navigate = useNavigate();
  useEffect(() => {
    if (token) {
      navigate('/');
    }
  }, [token]);

  if (!token) {
    if (switchRegLog == 0) {
      return (
        <>
          <div className="mobile-login-page">
            <div className="mobile-login-background"></div>
            <div className="mobile-login-form">
              <h1 className="mobile-login-title">Login to Page Creator</h1>
              <p className="mobile-login-inform">
                You must login before using ANY of the app's features.
              </p>
              <br></br>
              <div>
                {usernameError && (
                  <>
                    <span className="mobile-login-field-error">
                      {usernameError}
                    </span>
                  </>
                )}
                <div className="mobile-signin-grid ">
                  <label>Username </label>
                  <input
                    type="text"
                    placeholder="username"
                    name="username"
                    required
                    value={us}
                    onChange={(e) => {
                      setUsername(e.target.value);
                      if (e.target.value == '') {
                        setUsernameError('Field cannot be blank');
                      } else {
                        setUsernameError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {passwordError && (
                  <span className="mobile-login-field-error">
                    {passwordError}
                  </span>
                )}
                <div className="mobile-signin-grid">
                  <label>Password </label>
                  <input
                    type="password"
                    placeholder="password"
                    name="password"
                    required
                    value={ps}
                    onChange={(e) => {
                      setPassword(e.target.value);
                      if (e.target.value == '') {
                        setPasswordError('Field cannot be blank');
                      } else {
                        setPasswordError('');
                      }
                    }}
                  ></input>
                </div>
              </div>
              <br></br>
              <p className="mobile-login-error">{error}</p>
              <br></br>

              <button className="mobile-login-submit" onClick={() => submit()}>
                Login
              </button>
              <br></br>
              <br></br>
              <br></br>
              <p className="mobile-login-inform">
                Don't have an account? You're a bum!
              </p>
              <br></br>
              <button
                className="mobile-login-switch"
                onClick={() => changeRegLog(1)}
              >
                Register
              </button>
              <br></br>
              <br></br>
              <br></br>
              <br></br>

              <a
                className="mobile-login-inform"
                onClick={() => changeRegLog(2)}
              >
                Did you forget your password? Poor thing.
              </a>
            </div>
          </div>
        </>
      );
    } else if (switchRegLog == 1) {
      return (
        <>
          <div className="mobile-login-page">
            <div className="mobile-login-background"></div>
            <div className="mobile-login-form">
              <h1 className="mobile-login-title">Register</h1>

              <div>
                <br></br>
                {emailError && (
                  <span className="mobile-login-field-error">{emailError}</span>
                )}
                <div className="mobile-signin-grid">
                  <label>Email </label>
                  <input
                    type="text"
                    placeholder="me-myself_and_I@mail.com"
                    name="email"
                    required
                    value={em}
                    onChange={(e) => {
                      setEmail(e.target.value);
                      if (e.target.value == '') {
                        setEmailError('Field cannot be blank');
                      } else {
                        setEmailError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {usernameError && (
                  <span className="mobile-login-field-error">
                    {usernameError}
                  </span>
                )}
                <div className="mobile-signin-grid">
                  <label>Username</label>
                  <input
                    type="text"
                    placeholder="Enter a Username"
                    name="username"
                    required
                    value={us}
                    onChange={(e) => {
                      setUsername(e.target.value);
                      if (e.target.value == '') {
                        setUsernameError('Field cannot be blank');
                      } else {
                        setUsernameError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {nameError && (
                  <span className="mobile-login-field-error">{nameError}</span>
                )}
                <div className="mobile-signin-grid">
                  <label>Name</label>
                  <input
                    type="text"
                    placeholder="Enter your Name"
                    name="name"
                    required
                    value={nm}
                    onChange={(e) => {
                      setName(e.target.value);
                      if (e.target.value == '') {
                        setNameError('Field cannot be blank');
                      } else {
                        setNameError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {passwordError && (
                  <span className="mobile-login-field-error">
                    {passwordError}
                  </span>
                )}
                <div className="mobile-signin-grid">
                  <label>Password </label>
                  <input
                    type="password"
                    placeholder="Enter a Password"
                    name="password"
                    required
                    value={ps}
                    onChange={(e) => {
                      setPassword(e.target.value);
                      if (e.target.value == '') {
                        setPasswordError('Field cannot be blank');
                      } else {
                        setPasswordError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {confirmPasswordError && (
                  <span className="mobile-login-field-error">
                    {confirmPasswordError}
                  </span>
                )}
                <div className="mobile-signin-grid">
                  <label>Confirm Password </label>
                  <input
                    type="password"
                    placeholder="Same Password Again"
                    name="password"
                    required
                    value={cnps}
                    onChange={(e) => {
                      setConfirmPassword(e.target.value);
                      if (e.target.value == '') {
                        setConfirmPasswordError('Field cannot be blank');
                      } else {
                        setConfirmPasswordError('');
                      }
                    }}
                  ></input>
                </div>
              </div>
              <br></br>
              <p className="mobile-login-error">{error}</p>
              <button className="mobile-login-submit" onClick={() => submit()}>
                Register
              </button>
              <br></br>
              <br></br>
              <br></br>
              <br></br>
              <p className="mobile-login-inform">
                Already have an account? Why the Fuck did you click Register?
              </p>
              <br></br>
              <button
                className="mobile-login-switch"
                onClick={() => changeRegLog(0)}
              >
                Log in
              </button>
              <br></br>
              <br></br>
            </div>
          </div>
        </>
      );
    } else {
      return (
        <>
          <div className="mobile-login-page">
            <div className="mobile-login-background"></div>
            <div className="mobile-login-form">
              <h1 className="mobile-login-title">Reset Password</h1>
              <p className="mobile-login-inform">
                Provide your Username and Email and a new Password, then click
                Reset.
              </p>
              <br></br>
              {emailError && (
                <>
                  <span className="mobile-login-field-error">{emailError}</span>
                </>
              )}
              <div className="mobile-signin-grid ">
                <label>Email</label>
                <input
                  type="text"
                  placeholder="me-myself_and_I@mail.com"
                  name="email"
                  required
                  value={em}
                  onChange={(e) => {
                    setEmail(e.target.value);
                    if (e.target.value == '') {
                      setEmailError('Field cannot be blank');
                    } else {
                      setEmailError('');
                    }
                  }}
                ></input>
              </div>
              <br></br>
              <div>
                {usernameError && (
                  <>
                    <span className="mobile-login-field-error">
                      {usernameError}
                    </span>
                  </>
                )}
                <div className="mobile-signin-grid ">
                  <label>Username </label>
                  <input
                    type="text"
                    placeholder="me-myself_and_I@mail.com"
                    name="username"
                    required
                    value={us}
                    onChange={(e) => {
                      setUsername(e.target.value);
                      if (e.target.value == '') {
                        setUsernameError('Field cannot be blank');
                      } else {
                        setUsernameError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                {passwordError && (
                  <span className="mobile-login-field-error">
                    {passwordError}
                  </span>
                )}
                <div className="mobile-signin-grid">
                  <label>Password </label>
                  <input
                    type="password"
                    placeholder="password"
                    name="password"
                    required
                    value={ps}
                    onChange={(e) => {
                      setPassword(e.target.value);
                      if (e.target.value == '') {
                        setPasswordError('Field cannot be blank');
                      } else {
                        setPasswordError('');
                      }
                    }}
                  ></input>
                </div>
                <br></br>
                <div className="mobile-signin-grid">
                  <label>Confirm Password </label>
                  <input
                    type="password"
                    placeholder="Same password again"
                    name="password"
                    required
                    value={cnps}
                    onChange={(e) => {
                      setConfirmPassword(e.target.value);
                      if (e.target.value == '') {
                        setConfirmPasswordError('Field cannot be blank');
                      } else {
                        setConfirmPasswordError('');
                      }
                    }}
                  ></input>
                </div>
              </div>
              <br></br>
              <p className="mobile-login-error">{error}</p>
              <button
                className="mobile-login-submit"
                onClick={() => resetPassword()}
              >
                Reset Password
              </button>
              <br></br>
              <br></br>
              <br></br>
              <br></br>
              <br></br>
              <br></br>
              <button
                className="mobile-login-switch"
                onClick={() => changeRegLog(0)}
              >
                Back
              </button>
            </div>
          </div>
        </>
      );
    }
  } else {
    return (
      <>
        <p>You are already logged in as</p>
        <p>{sessionUser.email}</p>
      </>
    );
  }
}

export default LoginForm;

import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './Main/Home';
import Blocked from './Main/Blocked';
import NavBar from './Main/Navbar';
import LoginForm from './Login/LoginForm';
import Logout from './Login/Logout';
import ManageUsers from './Manage Users/ManageUsers';
import './Mobile.css';
import TopBar from './Main/TopBar';
import Pages from './Page/Pages';
import RedirectToLogin from './Login/RedirectToLogin';
import Page from './Page/Page';
import RedirectToPagesList from './Page/RedirectToPagesList';

function Mobile({
  jwtToken,
  sessionUser,
  APIAdress,
  setJwtToken,
  setSessionUser,
  selectedPage,
  setSelectedPage,
  selectedPageCreator,
  setSelectedPageCreator,
  show,
  setShow,
}) {
  return (
    <>
      <Router>
        {jwtToken !== null && sessionUser !== null ? (
          <>
            {sessionUser.user_type_id <= 4 ? (
              <>
                <TopBar token={jwtToken} sessionUser={sessionUser}></TopBar>
                <div className="mobile-mainpage">
                  <NavBar
                    token={jwtToken}
                    sessionUser={sessionUser}
                    show={show}
                    setShow={setShow}
                  />
                  <div className="mobile-page">
                    <Routes>
                      <Route
                        path="/"
                        element={<Home token={jwtToken} />}
                      ></Route>
                      <Route
                        path="/manage-users"
                        element={
                          <ManageUsers
                            APIAdress={APIAdress}
                            token={jwtToken}
                            sessionUser={sessionUser}
                          />
                        }
                      ></Route>
                      <Route
                        path="/pages"
                        element={
                          <Pages
                            APIAdress={APIAdress}
                            token={jwtToken}
                            sessionUser={sessionUser}
                            setSelectedPage={setSelectedPage}
                            setSelectedPageCreator={setSelectedPageCreator}
                          />
                        }
                      ></Route>
                      <Route
                        path="/page"
                        element={
                          selectedPage != null && selectedPage != undefined ? (
                            <>
                              <Page
                                APIAdress={APIAdress}
                                token={jwtToken}
                                sessionUser={sessionUser}
                                selectedPage={selectedPage}
                                setSelectedPage={setSelectedPage}
                                selectedPageCreator={selectedPageCreator}
                              ></Page>
                            </>
                          ) : (
                            <>
                              <RedirectToPagesList
                                selectedPage={selectedPage}
                              ></RedirectToPagesList>
                            </>
                          )
                        }
                      ></Route>
                      <Route
                        path="/logout"
                        element={
                          <Logout
                            setToken={setJwtToken}
                            setSessionUser={setSessionUser}
                          />
                        }
                      ></Route>
                    </Routes>
                  </div>
                </div>
              </>
            ) : (
              <>
                <TopBar token={jwtToken} sessionUser={sessionUser}></TopBar>
                <div className="mobile-mainpage">
                  <Routes>
                    <Route
                      path="/"
                      element={<Blocked token={jwtToken} />}
                    ></Route>
                    <Route
                      path="/logout"
                      element={
                        <Logout
                          setToken={setJwtToken}
                          setSessionUser={setSessionUser}
                        />
                      }
                    ></Route>
                  </Routes>
                </div>
              </>
            )}
          </>
        ) : (
          <></>
        )}
        <Routes>
          <Route
            path="/"
            element={
              <RedirectToLogin
                token={jwtToken}
                sessionUser={sessionUser}
              ></RedirectToLogin>
            }
          ></Route>
          <Route
            path="/login"
            element={
              <LoginForm
                APIAdress={APIAdress}
                token={jwtToken}
                setToken={setJwtToken}
                sessionUser={sessionUser}
                setSessionUser={setSessionUser}
              />
            }
          ></Route>
        </Routes>
      </Router>
    </>
  );
}

export default Mobile;

import { useMediaQuery } from 'react-responsive';
import { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { jwtDecode } from 'jwt-decode';
import Desktop from './components/desktop/Desktop';
import Mobile from './components/mobile/Mobile';

//import { MobilePage } from "./components/desktop/main/
// Mainpage"; //not setup yet

function App() {
  var [APIAdress, setAPIAdress] = useState(() => {
    try {
      return import.meta.env.VITE_API_URL;
    } catch {
      return null;
    }
  });

  var [jwtToken, setJwtToken] = useState(() => {
    try {
      return JSON.parse(localStorage.getItem('jwtToken'));
    } catch {
      return null;
    }
  });

  useEffect(() => {
    localStorage.setItem('jwtToken', JSON.stringify(jwtToken));
  }, [jwtToken]);

  var [sessionUser, setSessionUser] = useState(() => {
    try {
      return JSON.parse(localStorage.getItem('sessionUser'));
    } catch {
      return null;
    }
  });

  useEffect(() => {
    localStorage.setItem('sessionUser', JSON.stringify(sessionUser));
  }, [sessionUser]);

  const isTokenExpired = (token) => {
    if (!token) return true;
    try {
      const decoded = jwtDecode(token);
      const currentTime = Date.now() / 1000; // Convert milliseconds to seconds

      // Returns true if the token has expired
      return decoded.exp < currentTime;
    } catch (error) {
      return true; // Treat invalid/corrupted tokens as expired
    }
  };

  useEffect(() => {
    if (jwtToken != null) {
      if (isTokenExpired(jwtToken)) {
        window.location.replace('/logout');
        return;
      }

      // Calculate exact milliseconds remaining until expiration
      const decoded = jwtDecode(jwtToken);
      const msUntilExpiry = decoded.exp * 1000 - Date.now();

      const logoutTimeout = setTimeout(() => {
        window.location.replace('/logout');
      }, msUntilExpiry);
    } else {
      if (window.location.pathname != '/login') {
        window.location.replace('/login');
      }
    }
  }, [jwtToken]);

  var [selectedPage, setSelectedPage] = useState(null);
  var [selectedPageCreator, setSelectedPageCreator] = useState(null);
  var [show, setShow] = useState(true);

  const isDesktop = useMediaQuery({ minWidth: 850 });
  const isMobile = useMediaQuery({ maxWidth: 849 });

  return (
    <>
      {isDesktop ? (
        <>
          <Desktop
            jwtToken={jwtToken}
            sessionUser={sessionUser}
            APIAdress={APIAdress}
            setJwtToken={setJwtToken}
            setSessionUser={setSessionUser}
            selectedPage={selectedPage}
            setSelectedPage={setSelectedPage}
            selectedPageCreator={selectedPageCreator}
            setSelectedPageCreator={setSelectedPageCreator}
            show={show}
            setShow={setShow}
          ></Desktop>
        </>
      ) : (
        <>
          <Mobile
            jwtToken={jwtToken}
            sessionUser={sessionUser}
            APIAdress={APIAdress}
            setJwtToken={setJwtToken}
            setSessionUser={setSessionUser}
            selectedPage={selectedPage}
            setSelectedPage={setSelectedPage}
            selectedPageCreator={selectedPageCreator}
            setSelectedPageCreator={setSelectedPageCreator}
            show={show}
            setShow={setShow}
          ></Mobile>
        </>
      )}
      {isMobile ? <></> : <></>}
    </>
  );
}

export default App;

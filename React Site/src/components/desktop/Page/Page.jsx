import { useEffect, useState, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import './page.css';
import PageSettings from './PageSettings/PageSettings';
import DefaultPageElement from './PageElements/DefaultPageElement';
import PageElements from './PageElements/PageElements';

function Page({
  APIAdress,
  token,
  sessionUser,
  selectedPage,
  setSelectedPage,
  selectedPageCreator,
}) {
  const navigate = useNavigate();
  useEffect(() => {
    if (!token) {
      navigate('/login');
    }
  }, []);

  useEffect(() => {
    if (sessionUser.user_type_id > 4) {
      navigate('/');
    }
  }, []);

  useEffect(() => {
    if (selectedPage != null && selectedPage != undefined) {
      getSessionUserPagePermissions();
      getPageCss();
      if (sessionUser.user_type_id <= 2) {
        getAllPageCSS();
      }
      if (sessionUser.user_type_id <= 2) {
        getAllPageElementTypes();
        getAllPageElements();
        getAllPagePermissions();
        getAllUsersWithPagePermissions();
        getAllPagePermissionTypes();
      }
    }
  }, []);

  useEffect(() => {
    setPageClassName('creator_page' + selectedPage.id);
  }, [pageCss]);

  const childDiv = useRef(null);

  useEffect(() => {
    if (childDiv.current) {
      const parentElement = childDiv.current.parentElement;
      if (parentElement) {
        parentElement.className = 'creator-page-parent';
      }
    }
  }, [pageCss]);

  var [userPermissions, setUserPermissions] = useState(null);

  var [pageElements, setPageElements] = useState([]);
  var [pageElementTypes, setPageElementTypes] = useState([]);

  var [usersPagePermissions, setUsersPagePermissions] = useState([]);
  var [usersWithPagePermissions, setUsersWithPagePermissions] = useState([]);
  var [permissionTypes, setPermissionTypes] = useState([]);

  var [pageCss, setPageCss] = useState(null);
  var [pageCsses, setpageCsses] = useState([]);
  var [pageClassName, setPageClassName] = useState(
    'creator_page' + selectedPage.id
  );

  var [editing, setEditing] = useState(false);

  var [error, setError] = useState('');

  useEffect(() => {
    if (
      selectedPage != null &&
      selectedPage != undefined &&
      userPermissions != null
    ) {
      if (
        sessionUser.user_type_id == 3 &&
        userPermissions.permission_type_id <= 2
      ) {
        getAllPageCSS();
      }
      if (
        sessionUser.user_type_id == 3 &&
        userPermissions.permission_type_id == 1
      ) {
        getAllPageElementTypes();
        getAllPageElements();
        getAllPagePermissions();
        getAllPageCSS();
        getAllUsersWithPagePermissions();
        getAllPagePermissionTypes();
      } else if (
        sessionUser.user_type_id == 3 &&
        userPermissions.permission_type_id <= 2
      ) {
        getAllPageElementTypes();
        getAllPageElements();
        getAllPageCSS();
      } else if (
        sessionUser.user_type_id <= 4 &&
        userPermissions.permission_type_id <= 4
      ) {
        getAllPageElementTypes();
        getAllPageElements();
      }
    }
  }, [userPermissions]);

  async function refreshPage() {
    if (selectedPage != null && selectedPage != undefined) {
      getSessionUserPagePermissions();
      getAllPageElementTypes();
      getPageCss();
      getAllPageElements();
      if (
        sessionUser.user_type_id <= 2 ||
        (sessionUser.user_type_id == 3 &&
          userPermissions.permission_type_id <= 2)
      ) {
        getAllPageCSS();
      }
      if (
        sessionUser.user_type_id <= 2 ||
        (sessionUser.user_type_id == 3 &&
          userPermissions.permission_type_id == 1)
      ) {
        getAllPagePermissions();
        getAllUsersWithPagePermissions();
        getAllPagePermissionTypes();
      }
    }
  }

  async function getSessionUserPagePermissions() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
      }),
    };

    try {
      const fetchSessionUserPermissions = await fetch(
        APIAdress + `page-permissions/user-page-permissions`,
        settings
      );

      if (
        !fetchSessionUserPermissions.ok &&
        fetchSessionUserPermissions.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchSessionUserPermissions.json();

      if (response.success == true) {
        const permissions = response.data;
        setUserPermissions(permissions);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getPage(page_id) {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(page_id),
      }),
    };

    try {
      const fetchPage = await fetch(APIAdress + `pages/page/get`, settings);

      if (!fetchPage.ok && fetchPage.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPage.json();
      if (response.success == true) {
        const page = response.data;
        setSelectedPage(page);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getAllPageElements() {
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
      const fetchgetAllPageElements = await fetch(
        APIAdress + `page-elements/list-elements`,
        settings
      );

      if (
        !fetchgetAllPageElements.ok &&
        fetchgetAllPageElements.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchgetAllPageElements.json();
      if (response.success == true) {
        const pageElements = structuredClone(response.data);

        setPageElements(
          pageElements.map((pageElement) => ({ ...pageElement }))
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

  async function getAllPageElementTypes() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
      }),
    };

    try {
      const fetchAllPageElementTypes = await fetch(
        APIAdress + `page-elements/page-elements-types`,
        settings
      );

      if (
        !fetchAllPageElementTypes.ok &&
        fetchAllPageElementTypes.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchAllPageElementTypes.json();
      if (response.success == true) {
        const elementTypes = structuredClone(response.data);

        setPageElementTypes(
          elementTypes.map((pageElementType) => ({ ...pageElementType }))
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

  async function getPageCss() {
    const settings = {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };

    try {
      const fetchPageCss = await fetch(
        APIAdress + `page-css/css/` + selectedPage.selected_css_id,
        settings
      );

      if (!fetchPageCss.ok && fetchPageCss.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPageCss.json();
      if (response.success == true) {
        const css = response.data;
        setPageCss(css);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getAllPagePermissions() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
      }),
    };

    try {
      const fetchAllPagePermissions = await fetch(
        APIAdress + `page-permissions/all-page-user-permissions`,
        settings
      );

      if (!fetchAllPagePermissions.ok && response.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchAllPagePermissions.json();
      if (response.success == true) {
        const permissions = structuredClone(response.data);

        setUsersPagePermissions(
          permissions.map((userPermission) => ({ ...userPermission }))
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

  async function getAllUsersWithPagePermissions() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
      }),
    };

    try {
      const fetchPages = await fetch(
        APIAdress + `page-permissions/all-page-users-with-permissions`,
        settings
      );

      if (!fetchPages.ok && fetchPages.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPages.json();

      if (response.success == true) {
        const users = structuredClone(response.data);

        setUsersWithPagePermissions(
          users.map((userPermission) => ({
            ...userPermission,
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

  async function getAllPagePermissionTypes() {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
      }),
    };

    try {
      const fetchAllPagePermissionsTypes = await fetch(
        APIAdress + `page-permissions/page-permission-types`,
        settings
      );

      if (
        !fetchAllPagePermissionsTypes.ok &&
        fetchAllPagePermissionsTypes.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchAllPagePermissionsTypes.json();

      if (response.success == true) {
        const permissionTypes = structuredClone(response.data);

        setPermissionTypes(
          permissionTypes.map((permissionType) => ({
            ...permissionType,
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

  async function getAllPageCSS() {
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
      const fetchAllPageCss = await fetch(
        APIAdress + `page-css/list-css`,
        settings
      );

      if (!fetchAllPageCss.ok && fetchAllPageCss.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchAllPageCss.json();

      if (response.success == true) {
        const csses = structuredClone(response.data);

        setpageCsses(
          csses.map((csss) => ({
            ...csss,
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

  const dateCreated = new Date(selectedPage.date_created);

  if (userPermissions != null) {
    return (
      <>
        <div ref={childDiv}>
          <div className="page-top-section">
            <div className="page-top-actions">
              <button className="refresh" onClick={() => refreshPage()}>
                <svg
                  width="24px"
                  height="24px"
                  viewBox="0 0 24 24"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M21 3V8M21 8H16M21 8L18 5.29168C16.4077 3.86656 14.3051 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21C16.2832 21 19.8675 18.008 20.777 14"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </button>
              <PageSettings
                sessionUser={sessionUser}
                token={token}
                selectedPage={selectedPage}
                setSelectedPage={setSelectedPage}
                getPage={getPage}
                APIAdress={APIAdress}
                pageCss={pageCss}
                getPageCss={getPageCss}
                userPermissions={userPermissions}
                usersPagePermissions={usersPagePermissions}
                usersWithPagePermissions={usersWithPagePermissions}
                permissionTypes={permissionTypes}
                getAllPagePermissions={getAllPagePermissions}
                getAllUsersWithPagePermissions={getAllUsersWithPagePermissions}
                pageCsses={pageCsses}
                getAllPageCSS={getAllPageCSS}
                pageClassName={pageClassName}
              ></PageSettings>
            </div>
            {editing === false ? (
              <>
                <br></br>
                <br></br>
                {sessionUser.user_type_id <= 3 &&
                userPermissions.permission_type_id <= 2 ? (
                  <>
                    <button
                      className="edit-elements"
                      onClick={() => setEditing(true)}
                    >
                      <svg
                        width="24px"
                        height="24px"
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
                  </>
                ) : (
                  <></>
                )}
              </>
            ) : (
              <></>
            )}
          </div>
          <div className={pageClassName}>
            <div
              style={{ flexGrow: '2', height: '100%', paddingBottom: '30%' }}
            >
              <div>
                <p className="error">{error}</p>
                <div className="page-content">
                  <h1 style={{ textAlign: 'center' }} className={pageClassName}>
                    {selectedPage.title}
                  </h1>

                  <p className="page-createdby">
                    Created on the {dateCreated.getDay()}/
                    {dateCreated.getMonth()}/{dateCreated.getFullYear()} by{' '}
                    {selectedPageCreator}.
                  </p>
                </div>
              </div>
              <div>
                <PageElements
                  sessionUser={sessionUser}
                  token={token}
                  APIAdress={APIAdress}
                  selectedPage={selectedPage}
                  pageElements={pageElements}
                  getAllPageElements={getAllPageElements}
                  pageElementTypes={pageElementTypes}
                  pageCss={pageCss}
                  editing={editing}
                  setEditing={setEditing}
                ></PageElements>
              </div>
            </div>
          </div>
        </div>
      </>
    );
  }
}

export default Page;

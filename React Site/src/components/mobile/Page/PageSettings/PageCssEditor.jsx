import { useState } from 'react';
import './page-settings.css';

function PageCSSEditor({
  sessionUser,
  token,
  APIAdress,
  selectedPage,
  getPage,
  pageCss,
  getPageCss,
  pageCsses,
  getAllPageCSS,
  elementsClassName,
}) {
  var [editing, setEditing] = useState(true);

  var [css, setCss] = useState(pageCss.css);
  var [cssSheetId, setCssSheetId] = useState(selectedPage.selected_css_id);
  var [cssSheetName, setCssSheetName] = useState('');

  var [error, setError] = useState(null);

  var cs = css;
  var csname = cssSheetName;
  var pcss = cssSheetId;

  async function newCSS() {
    if (!validateNewInput()) {
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
        page_id: Number(selectedPage.id),
        sheet_name: cssSheetName,
        css: css,
      }),
    };
    try {
      const fetchNewCSS = await fetch(APIAdress + `page-css/css`, settings);

      if (!fetchNewCSS.ok && fetchNewCSS.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchNewCSS.json();
      if (response.success == true) {
        setError('');
        getAllPageCSS();
        switchToAdd();
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function updateCSS() {
    if (!validateUpdateInput()) {
      return;
    }

    const settings = {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        id: Number(cssSheetId),
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
        css: css,
      }),
    };
    try {
      const fetchUpdateCSS = await fetch(APIAdress + `page-css/css`, settings);

      if (!fetchUpdateCSS.ok && fetchUpdateCSS.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUpdateCSS.json();
      if (response.success == true) {
        setError('');
        if (cssSheetId == selectedPage.selected_css_id) {
          getPageCss();
        }
        getAllPageCSS();
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function deleteCSS() {
    if (!validateDelete()) {
      return;
    }

    if (
      confirm(
        'Are you sure you want to delete this CSS Sheet? It would be a shame to lose it.'
      )
    ) {
      const settings = {
        method: 'DELETE',
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id: Number(cssSheetId),
          session_user_id: Number(sessionUser.id),
          page_id: Number(selectedPage.id),
        }),
      };
      try {
        const fetchDeleteCSS = await fetch(
          APIAdress + `page-css/css`,
          settings
        );

        if (!fetchDeleteCSS.ok && fetchDeleteCSS.status !== 400) {
          throw new Error('Cannot Connect to Server');
        }

        const response = await fetchDeleteCSS.json();
        if (response.success == true) {
          setError('');
          getPageCss();
          getAllPageCSS();
          setCss('');
          setCssSheetId('None');
        } else {
          setError(response.msg);
        }
      } catch (e) {
        console.log(e.message);
        setError('Cannot Connect to Server');
      }
    } else {
      return;
    }
  }

  function validateNewInput() {
    if (cssSheetName == '') {
      setError('A CSS Sheet Name must be provided.');
      return false;
    } else {
      return true;
    }
  }

  function validateUpdateInput() {
    if (cssSheetId == 'None') {
      setError('A CSS Sheet must be Selected.');
      return false;
    } else {
      return true;
    }
  }

  function validateDelete() {
    if (cssSheetId == selectedPage.selected_css_id) {
      setError('Cannot Delete Css currently assigned to Page.');
      return false;
    }
    for (let i = 0; i < pageCsses.length; i++) {
      if (
        pageCsses[i].id == cssSheetId &&
        pageCsses[i].sheet_name == 'Default'
      ) {
        console.log();
        setError('Cannot Delete Default Page Css.');
        return false;
      }

      return true;
    }
  }

  function switchToAdd() {
    if (editing) {
      setEditing(false);
      setCss('');
    } else {
      setEditing(true);
      setCss(pageCss.css);
      setCssSheetName('');
    }
  }

  function loadCssFromList(css_id) {
    if (pageCsses != null || pageCsses != undefined) {
      for (let i = 0; i < pageCsses.length; i++) {
        if (pageCsses[i].id == css_id) {
          setCss(pageCsses[i].css);
          break;
        }
      }
    }
  }

  if (pageCsses.length > 0) {
    if (editing) {
      return (
        <>
          <div className="mobile-settings-section">
            <div className="mobile-edit-window">
              <br></br>
              <h3 className="mobile-section-title">Page CSS</h3>
              <br></br>
              <p className="mobile-settings-error">{error}</p>

              {pageCsses.length > 0 ? (
                <>
                  <div className="mobile-edit-page-details">
                    <p>Css Sheet</p>
                    <select
                      className="mobile-edit-select-sheet"
                      style={{ background: '#FCFFFF' }}
                      value={pcss}
                      onChange={(e) => {
                        setCssSheetId(e.target.value);
                        if (e.target.value == 'None') {
                          setCss('');
                        } else {
                          loadCssFromList(e.target.value);
                        }
                      }}
                    >
                      <option value={null}>None</option>
                      {pageCsses.map((pageCs) => (
                        <option key={pageCs.id} value={pageCs.id}>
                          {pageCs.sheet_name}
                        </option>
                      ))}
                    </select>

                    {cssSheetId === 'None' ? (
                      <></>
                    ) : (
                      <>
                        <p>Page CSS</p>
                        <textarea
                          className="mobile-edit-css"
                          type="text"
                          placeholder="div.page {
              text-align: backwards;
              }"
                          name="css"
                          required
                          value={cs}
                          onChange={(e) => {
                            setCss(e.target.value);
                          }}
                        ></textarea>
                      </>
                    )}
                  </div>
                  {cssSheetId != 'None' ? (
                    <>
                      <p className="mobile-tip">
                        Please write Css with .{elementsClassName} after any
                        element type, for example "div.{elementsClassName} {'{'}
                        text-align: center; color: black;
                        {'}'}" or even "a.{elementsClassName} {'{'}color: blue;
                        {'}'}".
                        <br></br>
                        for videos div.{elementsClassName}-video is required
                        instead for, example div.{elementsClassName}-video {'{'}
                        width
                        {':'} {'50%; }'}.<br></br>
                        Additionally for the mobile version of the page use
                        mobile- in front of the class name, you may see it
                        change above ^ to include mobile- if you modify the
                        screen width.
                        <br></br>
                        You can also add class names to individual page elements
                        and use those instead.
                      </p>
                    </>
                  ) : (
                    <></>
                  )}
                </>
              ) : (
                <></>
              )}

              {cssSheetId === 'None' ? (
                <></>
              ) : (
                <>
                  <br></br>

                  <div className="mobile-update-or-delete">
                    <div className="mobile-page-details-edit-or-delete">
                      <button
                        className="mobile-settings-update"
                        onClick={() => updateCSS()}
                      >
                        Update
                      </button>
                      <button
                        className="mobile-settings-del"
                        onClick={() => deleteCSS()}
                      >
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
                  </div>
                </>
              )}

              <br></br>

              <div>
                <button
                  className="mobile-settings-add"
                  onClick={() => switchToAdd()}
                >
                  +
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
          <div className="mobile-settings-section">
            <br></br>
            <h3 className="mobile-section-title">New Page CSS</h3>
            <br></br>
            <div className="mobile-edit-window">
              <p className="mobile-settings-error">{error}</p>
              <div className="mobile-edit-page-details">
                <p>Sheet Name</p>
                <input
                  className="mobile-edit-css"
                  type="text"
                  placeholder="dark-mode"
                  name="css-sheet-name"
                  required
                  value={csname}
                  onChange={(e) => {
                    setCssSheetName(e.target.value);
                  }}
                ></input>

                <p>Page CSS</p>
                <textarea
                  className="mobile-edit-css"
                  type="text"
                  placeholder="div.page {
              text-align: backwards;
              }"
                  name="css"
                  required
                  value={cs}
                  onChange={(e) => {
                    setCss(e.target.value);
                  }}
                ></textarea>
              </div>
              <br></br>
              <div className="mobile-update-or-delete">
                <div className="mobile-page-details-save-or-discard">
                  <button
                    className="mobile-settings-save"
                    onClick={() => newCSS()}
                  >
                    Save
                  </button>
                  <button
                    className="mobile-settings-discard"
                    onClick={() => switchToAdd()}
                  >
                    Discard
                  </button>
                </div>
                <br></br>
              </div>
              <br></br>
            </div>
          </div>
        </>
      );
    }
  }
}

export default PageCSSEditor;

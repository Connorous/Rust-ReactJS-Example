import { useState } from 'react';

function ParagraphPageElement({
  pageElement,
  content,
  setContent,
  editing,
  updateType,
  et,
  ct,
  csn,
  pageElementTypes,
  updatePageElement,
  updatePosition,
  deleteElement,
  cssClassName,
  setCssClassName,
}) {
  function updateContent(content) {
    setContent(content);
    var updatedPageElement = pageElement;
    updatedPageElement.content = content;
    updatePageElement(updatedPageElement);
  }

  function updateCssClassName(cssClassName) {
    setCssClassName(cssClassName);
    var updatedPageElement = pageElement;
    updatedPageElement.css_class_name = cssClassName;
    updatePageElement(updatedPageElement);
  }

  if (editing) {
    return (
      <>
        <div className="mobile-page-element-editing">
          <div className="mobile-elements-editor">
            <div className="mobile-elements-inputs">
              <select
                className="mobile-elements-change-type"
                value={et}
                onChange={(e) => updateType(Number(e.target.value))}
              >
                {pageElementTypes.map((pageElementType) => (
                  <option key={pageElementType.id} value={pageElementType.id}>
                    {pageElementType.type}
                  </option>
                ))}
              </select>
              <textarea
                className="mobile-elements-content"
                value={ct}
                onChange={(e) => updateContent(e.target.value)}
              ></textarea>
              <label>CSS Classname</label>
              <input
                className="mobile-elements-css"
                value={csn}
                onChange={(e) => updateCssClassName(e.target.value)}
              ></input>
            </div>
            <div className="mobile-elements-up-or-down">
              <svg
                onClick={() => updatePosition(pageElement, false)}
                className="mobile-elements-up-down"
                width="20px"
                height="20px"
                viewBox="0.5 1 25 25"
                fill="none"
                fill-rule="evenodd"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M3 7.41992L3 17.4199C3 19.6291 4.79086 21.4199 7 21.4199H17C19.2091 21.4199 21 19.6291 21 17.4199V7.41992C21 5.21078 19.2091 3.41992 17 3.41992H7C4.79086 3.41992 3 5.21078 3 7.41992Z"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  paint-order="stroke"
                />
                <path
                  d="M8 13.8599L10.87 10.8C11.0125 10.6416 11.1868 10.5149 11.3815 10.4282C11.5761 10.3415 11.7869 10.2966 12 10.2966C12.2131 10.2966 12.4239 10.3415 12.6185 10.4282C12.8132 10.5149 12.9875 10.6416 13.13 10.8L16 13.8599"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  paint-order="stroke"
                />
              </svg>

              <svg
                onClick={() => updatePosition(pageElement, true)}
                className="mobile-elements-up-down"
                width="20px"
                height="20px"
                viewBox="0.5 1 25 25"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M21 17.4199V7.41992C21 5.21078 19.2091 3.41992 17 3.41992L7 3.41992C4.79086 3.41992 3 5.21078 3 7.41992V17.4199C3 19.6291 4.79086 21.4199 7 21.4199H17C19.2091 21.4199 21 19.6291 21 17.4199Z"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  paint-order="stroke"
                />
                <path
                  d="M16 10.99L13.1299 14.05C12.9858 14.2058 12.811 14.3298 12.6166 14.4148C12.4221 14.4998 12.2122 14.5437 12 14.5437C11.7878 14.5437 11.5779 14.4998 11.3834 14.4148C11.189 14.3298 11.0142 14.2058 10.87 14.05L8 10.99"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  paint-order="stroke"
                />
              </svg>
            </div>
          </div>

          <button
            className="mobile-elements-del"
            onClick={() => {
              deleteElement(pageElement);
            }}
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
      </>
    );
  } else {
    return (
      <>
        <p className={cssClassName}>{content}</p>
      </>
    );
  }
}

export default ParagraphPageElement;

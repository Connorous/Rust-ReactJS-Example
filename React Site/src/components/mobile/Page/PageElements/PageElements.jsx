import { useState, useEffect, useRef } from 'react';
import uuid from 'react-uuid';
import DefaultPageElement from './DefaultPageElement';
import './page-elements.css';

function PageElements({
  APIAdress,
  sessionUser,
  token,
  selectedPage,
  pageElements,
  getAllPageElements,
  pageElementTypes,
  getPageElements,
  pageCss,
  editing,
  setEditing,
  refreshPage,
}) {
  const cssRef = useRef(null);
  useEffect(() => {
    if (cssRef.current && pageCss != null) {
      const styleElement = document.createElement('style');
      styleElement.textContent = pageCss.css;

      cssRef.current.appendChild(styleElement);
    }
  }, [pageCss, editing]);

  function assignCss() {
    if (cssRef.current && pageCss != null) {
      const styleElement = document.createElement('style');
      styleElement.textContent = pageCss.css;

      cssRef.current.appendChild(styleElement);
    }
  }

  useEffect(() => {
    const elements = structuredClone(pageElements);

    setOriginalPageElements(
      elements.map((pageElement) => ({ ...pageElement }))
    );
    setCurrentPageElements(elements.map((pageElement) => ({ ...pageElement })));
  }, [pageElements]);

  var [orignalPageElements, setOriginalPageElements] = useState(pageElements);
  var [currentPageElements, setCurrentPageElements] = useState(
    structuredClone(orignalPageElements)
  );
  var [newPageElements, setNewPageElements] = useState([]);
  var [updatingPageElements, setUpdatingPageElements] = useState([]);
  var [deletingPageElements, setDeletingPageElements] = useState([]);

  var [pageClassName, setPageClassName] = useState(
    'mobile-creator_page' + selectedPage.id
  );

  var [error, setError] = useState('');

  var combinedElementLists = Array.from(
    new Map(
      [...currentPageElements, ...updatingPageElements, ...newPageElements].map(
        (item) => [item.id, item]
      )
    ).values()
  );

  var sortedCombinedElementList = Array.from(
    combinedElementLists.values()
  ).sort((a, b) => a.position - b.position);

  function uuidFromUuidV4() {
    const newUuid = uuid();
    return newUuid;
  }

  function newPageElement(parent_element_id) {
    setNewPageElements((prevElements) => [
      ...prevElements,
      {
        id: uuidFromUuidV4(),
        element_type_id: null,
        parent_element_id: parent_element_id,
        page_id: selectedPage.id,
        position: currentPageElements.length + newPageElements.length,
        content: '',
        link: '',
        css_class_name: '',
      },
    ]);
  }

  function updateNewPageElement(newPageElementToUpdate) {
    setNewPageElements(
      newPageElements.map((pageElement) => {
        if (pageElement.id === newPageElementToUpdate.id) {
          return {
            ...pageElement,
            element_type_id: newPageElementToUpdate.element_type_id,
            parent_element_id: newPageElementToUpdate.parent_element_id,
            position: newPageElementToUpdate.position,
            content: newPageElementToUpdate.content,
            link: newPageElementToUpdate.link,
            css_class_name: newPageElementToUpdate.css_class_name,
          };
        } else {
          return pageElement;
        }
      })
    );
  }

  function addUpdatingExistingPageElement(newPageElementToUpdate) {
    setUpdatingPageElements((prevElements) => [
      ...prevElements,
      {
        id: newPageElementToUpdate.id,
        element_type_id: newPageElementToUpdate.element_type_id,
        parent_element_id: newPageElementToUpdate.parent_element_id,
        page_id: selectedPage.id,
        position: newPageElementToUpdate.position,
        content: newPageElementToUpdate.content,
        link: newPageElementToUpdate.link,
        css_class_name: newPageElementToUpdate.css_class_name,
      },
    ]);
  }

  function updateExistingPageElement(pageElementToUpdate) {
    setUpdatingPageElements(
      updatingPageElements.map((pageElement) => {
        if (pageElement.id === pageElementToUpdate.id) {
          return {
            ...pageElement,
            element_type_id: pageElementToUpdate.element_type_id,
            parent_element_id: pageElementToUpdate.parent_element_id,
            position: pageElementToUpdate.position,
            content: pageElementToUpdate.content,
            link: pageElementToUpdate.link,
            css_class_name: pageElementToUpdate.css_class_name,
          };
        } else {
          return pageElement;
        }
      })
    );
  }

  function updatePageElement(pageElement) {
    var existing = false;
    for (let i = 0; i < orignalPageElements.length; i++) {
      if (orignalPageElements[i].id == pageElement.id) {
        existing = true;
        break;
      }
    }

    if (existing) {
      var inUpdateList = false;
      for (let i = 0; i < updatingPageElements.length; i++) {
        if (updatingPageElements[i].id == pageElement.id) {
          inUpdateList = true;
          break;
        }
      }
      if (inUpdateList) {
        updateExistingPageElement(pageElement);
      } else {
        addUpdatingExistingPageElement(pageElement);
      }
    } else {
      var inNewList = false;
      for (let i = 0; i < newPageElements.length; i++) {
        if (newPageElements[i].id == pageElement.id) {
          inNewList = true;
          break;
        }
      }
      if (inNewList) {
        updateNewPageElement(pageElement);
      }
    }
  }

  function getPageElementInPositionFromAnyList(position) {
    for (let i = 0; i < sortedCombinedElementList.length; i++) {
      if (sortedCombinedElementList[i].position == position) {
        return sortedCombinedElementList[i];
      }
    }

    return null;
  }

  async function updatePosition(pageElement, moveDown) {
    if (
      (moveDown && pageElement.position >= sortedCombinedElementList.length) ||
      (!moveDown && pageElement.position == 0)
    ) {
      return;
    }

    var elementsPosition = pageElement.position;
    var elementToUpdatePosition;

    if (moveDown) {
      var newPosition = elementsPosition + 1;
      var elementToUpdatePosition =
        getPageElementInPositionFromAnyList(newPosition);
      if (elementToUpdatePosition != null) {
        elementToUpdatePosition.position = elementsPosition;
        pageElement.position = newPosition;
      }
    } else {
      var newPosition = elementsPosition - 1;
      var elementToUpdatePosition =
        getPageElementInPositionFromAnyList(newPosition);
      if (elementToUpdatePosition != null) {
        elementToUpdatePosition.position = elementsPosition;
        pageElement.position = newPosition;
      }
    }

    if (
      elementToUpdatePosition == null ||
      elementToUpdatePosition == undefined
    ) {
      return;
    }

    var element1Exists = false;
    var element2Exists = false;

    for (let i = 0; i < orignalPageElements.length; i++) {
      if (element1Exists && element2Exists) {
        break;
      }
      if (orignalPageElements[i].id == pageElement.id) {
        element1Exists = true;
      }
      if (orignalPageElements[i].id == elementToUpdatePosition.id) {
        element2Exists = true;
      }
    }

    var element1InNew = false;
    var element2InNew = false;

    if (!element1Exists || !element2Exists) {
      for (let i = 0; i < newPageElements.length; i++) {
        if (element1InNew && element2InNew) {
          break;
        }
        if (newPageElements[i].id == pageElement.id) {
          element1InNew = true;
        }
        if (newPageElements[i].id == elementToUpdatePosition.id) {
          element2InNew = true;
        }
      }
    }

    var element1InUpdating = false;
    var element2InUpdating = false;

    if (element1Exists || element2Exists) {
      for (let i = 0; i < updatingPageElements.length; i++) {
        if (element1InUpdating && element2InUpdating) {
          break;
        }
        if (updatingPageElements[i].id == pageElement.id) {
          element1InUpdating = true;
        }
        if (updatingPageElements[i].id == elementToUpdatePosition.id) {
          element2InUpdating = true;
        }
      }
    }

    if (element1Exists && element2Exists) {
      if (element1InUpdating && element2InUpdating) {
        setUpdatingPageElements((prev) =>
          updatingPageElements.map((updatingElement) => {
            if (updatingElement.id === pageElement.id) {
              return {
                ...updatingElement,
                position: pageElement.position,
              };
            } else if (updatingElement.id == elementToUpdatePosition.id) {
              return {
                ...updatingElement,
                position: elementToUpdatePosition.position,
              };
            } else {
              return updatingElement;
            }
          })
        );
      } else if (element1InUpdating) {
        var preUpdating = updatingPageElements.map((updatingElement) => {
          if (updatingElement.id === pageElement.id) {
            return {
              ...updatingElement,
              position: pageElement.position,
            };
          } else {
            return updatingElement;
          }
        });

        setUpdatingPageElements([...preUpdating, elementToUpdatePosition]);
      } else if (element2InUpdating) {
        var preUpdating = updatingPageElements.map((updatingElement) => {
          if (updatingElement.id === elementToUpdatePosition.id) {
            return {
              ...updatingElement,
              position: elementToUpdatePosition.position,
            };
          } else {
            return updatingElement;
          }
        });

        setUpdatingPageElements([...preUpdating, pageElement]);
      } else {
        setUpdatingPageElements((prevItems) => [
          ...prevItems,
          pageElement,
          elementToUpdatePosition,
        ]);
      }
    } else if (element1InNew || element2InNew) {
      if (element1InNew && element2InNew) {
        setNewPageElements((prev) =>
          newPageElements.map((newElement) => {
            if (newElement.id === pageElement.id) {
              return {
                ...newElement,
                position: pageElement.position,
              };
            } else if (newElement.id == elementToUpdatePosition.id) {
              return {
                ...newElement,
                position: elementToUpdatePosition.position,
              };
            } else {
              return newElement;
            }
          })
        );
      } else if (element1InNew) {
        var prevNew = newPageElements.map((newElement) => {
          if (newElement.id === pageElement.id) {
            return {
              ...newElement,
              position: pageElement.position,
            };
          } else {
            return newElement;
          }
        });

        if (!element2Exists) {
          var prevNew = newPageElements.map((newElement) => {
            if (newElement.id === pageElement.id) {
              return {
                ...newElement,
                position: pageElement.position,
              };
            } else {
              return newElement;
            }
          });

          setNewPageElements([...prevNew, elementToUpdatePosition]);
        } else if (element2Exists) {
          updateNewPageElement(pageElement);
          if (element2InUpdating) {
            updateExistingPageElement(elementToUpdatePosition);
          } else {
            addUpdatingExistingPageElement(elementToUpdatePosition);
          }
        }
      } else if (element2InNew) {
        if (!element1Exists) {
          var prevNew = newPageElements.map((newElement) => {
            if (newElement.id === elementToUpdatePosition.id) {
              return {
                ...newElement,
                position: elementToUpdatePosition.position,
              };
            } else {
              return newElement;
            }
          });

          setNewPageElements([...prevNew, pageElement]);
        } else if (element2Exists) {
          updateNewPageElement(elementToUpdatePosition);
          if (element2InUpdating) {
            updateExistingPageElement(pageElement);
          } else {
            addUpdatingExistingPageElement(pageElement);
          }
        }
      } else {
        setNewPageElements((prevItems) => [
          ...prevItems,
          pageElement,
          elementToUpdatePosition,
        ]);
      }
    }
  }

  function deleteElement(pageElementToDelete) {
    var inUpdateList = false;
    var inCurrentList = false;

    for (let i = 0; i < inUpdateList.length; i++) {
      if (inUpdateList[i].id == pageElementToDelete.id) {
        inUpdateList = true;
        break;
      }
    }

    if (!inUpdateList) {
      for (let i = 0; i < currentPageElements.length; i++) {
        if (currentPageElements[i].id == pageElementToDelete.id) {
          inCurrentList = true;
          break;
        }
      }
    }

    if (inUpdateList || inCurrentList) {
      setDeletingPageElements([
        ...deletingPageElements,
        {
          id: pageElementToDelete.id,
          element_type_id: pageElementToDelete.element_type_id,
          parent_element_id: pageElementToDelete.parent_element_id,
          page_id: selectedPage.id,
          position: pageElementToDelete.position,
          content: pageElementToDelete.content,
          link: pageElementToDelete.link,
          css_class_name: pageElementToDelete.css_class_name,
        },
      ]);
    }

    var currentElements = currentPageElements.filter(
      (pageElement) => pageElement.id !== pageElementToDelete.id
    );

    var updatingElements = updatingPageElements.filter(
      (pageElement) => pageElement.id !== pageElementToDelete.id
    );

    var newElements = newPageElements.filter(
      (pageElement) => pageElement.id !== pageElementToDelete.id
    );

    setCurrentPageElements((preElements) =>
      preElements.filter(
        (pageElement) => pageElement.id !== pageElementToDelete.id
      )
    );

    decrementPositionAfter(
      pageElementToDelete.position,
      updatingElements,
      newElements
    );
  }

  function decrementPositionAfter(
    position,
    updatingElementsList,
    newElementsList
  ) {
    var elementsToDecreasePosition = [];
    for (let i = position + 1; i < sortedCombinedElementList.length; i++) {
      var elementAtPosition = getPageElementInPositionFromAnyList(i);
      if (elementAtPosition != null) {
        elementAtPosition.position = elementAtPosition.position - 1;

        elementsToDecreasePosition.push(elementAtPosition);
      }
    }

    var elementsInUpdateList = [];
    var elementsExisting = [];
    var elementsInNewList = [];

    for (let i = 0; i < elementsToDecreasePosition.length; i++) {
      var found = false;
      for (let j = 0; j < updatingPageElements.length; j++) {
        if (updatingPageElements[j].id == elementsToDecreasePosition[i].id) {
          elementsInUpdateList.push(elementsToDecreasePosition[i]);
          found = true;
          break;
        }
      }
      if (found) {
        continue;
      }
      for (let j = 0; j < orignalPageElements.length; j++) {
        if (orignalPageElements[j].id == elementsToDecreasePosition[i].id) {
          elementsExisting.push(elementsToDecreasePosition[i]);
          found = true;
          break;
        }
      }
      if (found) {
        continue;
      }
      for (let j = 0; j < newPageElements.length; j++) {
        if (newPageElements[j].id == elementsToDecreasePosition[i].id) {
          elementsInNewList.push(elementsToDecreasePosition[i]);
          break;
        }
      }
    }

    const updateElementsMap = new Map(
      elementsInUpdateList.map((element) => [element.id, element])
    );

    var updatingElements = updatingElementsList.map((pageElement) => {
      // Check if this item has an update pending
      if (updateElementsMap.has(pageElement.id)) {
        // Return a brand new object combining old values with new updates
        return { ...pageElement, ...updateElementsMap.get(pageElement.id) };
      }
      // Return unchanged item if no updates match
      return pageElement;
    });

    setUpdatingPageElements([...updatingElements, ...elementsExisting]);

    const newElementsMap = new Map(
      elementsInNewList.map((element) => [element.id, element])
    );

    var newElements = newElementsList.map((pageElement) => {
      // Check if this item has an update pending
      if (newElementsMap.has(pageElement.id)) {
        // Return a brand new object combining old values with new updates
        return { ...pageElement, ...newElementsMap.get(pageElement.id) };
      }
      // Return unchanged item if no updates match
      return pageElement;
    });

    setNewPageElements(newElements);
  }

  async function save() {
    addPageElements();
    updatePageElements();
    deletePageElements();
    getAllPageElements();

    const elements = pageElements;

    setEditing(false);
  }

  async function addPageElements() {
    if (newPageElements.length == 0) {
      return;
    }

    const newElements = newPageElements.map(({ id, ...rest }) => rest);

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
        new_page_elements: newElements,
      }),
    };

    try {
      const fetchAddPageElements = await fetch(
        APIAdress + `page-elements/new-elements`,
        settings
      );

      if (!fetchAddPageElements.ok && fetchAddPageElements.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchAddPageElements.json();
      if (response.success == true) {
        setNewPageElements([]);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function updatePageElements() {
    if (updatingPageElements.length == 0) {
      return;
    }

    const settings = {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
        updating_page_elements: updatingPageElements,
      }),
    };

    try {
      const fetchUpdatePageElements = await fetch(
        APIAdress + `page-elements/update-elements`,
        settings
      );

      if (
        !fetchUpdatePageElements.ok &&
        fetchUpdatePageElements.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUpdatePageElements.json();
      if (response.success == true) {
        setUpdatingPageElements([]);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function deletePageElements() {
    if (deletingPageElements.length == 0) {
      return;
    }

    class deletingPageElement {
      constructor(id) {
        this.id = id;
      }
    }

    var deletingElements = [];
    for (let i = 0; i < deletingPageElements.length; i++) {
      deletingElements.push(
        (deletingPageElement = {
          id: deletingPageElements[i].id,
        })
      );
    }

    const settings = {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(selectedPage.id),
        deleting_page_elements: deletingElements,
      }),
    };

    try {
      const fetchDeletePageElements = await fetch(
        APIAdress + `page-elements/delete-elements`,
        settings
      );

      if (
        !fetchDeletePageElements.ok &&
        fetchDeletePageElements.status !== 400
      ) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchDeletePageElements.json();
      if (response.success == true) {
        setUpdatingPageElements([]);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function discard() {
    setCurrentPageElements(structuredClone(orignalPageElements));
    setUpdatingPageElements([]);
    setNewPageElements([]);
    setDeletingPageElements([]);
    setEditing(false);
  }

  if (editing) {
    return (
      <>
        <div>
          <br></br>
          <p className="mobile-elements-error">{error}</p>
          <div className="mobile-elements-save-or-discard">
            <button className="mobile-elements-save" onClick={() => save()}>
              Save
            </button>
            <button
              className="mobile-elements-discard"
              onClick={() => discard()}
            >
              Discard
            </button>
          </div>
          <br></br>
          <br></br>
          <div className="mobile-editing-page-elements">
            {sortedCombinedElementList.map((pageElement) => (
              <DefaultPageElement
                key={pageElement.id}
                pageElement={pageElement}
                pageElementTypes={pageElementTypes}
                editing={editing}
                updatePageElement={updatePageElement}
                updatePosition={updatePosition}
                deleteElement={deleteElement}
                pageClassName={pageClassName}
              ></DefaultPageElement>
            ))}
          </div>
          <br></br>
          <button
            className="mobile-elements-add"
            onClick={() => newPageElement(null)}
          >
            +
          </button>
          <br></br>
          <br></br>
          <br></br>
          <div className="mobile-elements-save-or-discard">
            <button className="mobile-elements-save" onClick={() => save()}>
              Save
            </button>
            <button
              className="mobile-elements-discard"
              onClick={() => discard()}
            >
              Discard
            </button>
          </div>
        </div>
      </>
    );
  } else {
    return (
      <>
        <div>
          <p className="mobile-elements-error">{error}</p>
        </div>
        <div ref={cssRef} className={pageClassName}>
          <div className={pageClassName}>
            {orignalPageElements.map(({ ...pageElement }) => {
              return (
                <DefaultPageElement
                  key={pageElement.id}
                  pageElement={pageElement}
                  pageElementTypes={pageElementTypes}
                  editing={editing}
                  updatePageElement={updatePageElement}
                  pageClassName={pageClassName}
                ></DefaultPageElement>
              );
            })}
            <br></br>
          </div>
        </div>
      </>
    );
  }
}

export default PageElements;

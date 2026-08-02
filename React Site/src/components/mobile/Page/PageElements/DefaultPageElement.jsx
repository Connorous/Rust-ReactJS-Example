import { useState, useEffect } from 'react';
import TitlePageElement from './TitleElement';
import ParagraphPageElement from './ParagraphElement';
import ImageElement from './ImageElement';
import HyperLinkElement from './HyperLinkElement';
import CodeElement from './CodeElement';
import VideoElement from './VideoElement';

function DefaultPageElement({
  pageElement,
  pageElementTypes,
  editing,
  updatePageElement,
  updatePosition,
  deleteElement,
  pageClassName,
}) {
  var [elementType, setElementType] = useState(pageElement.element_type_id);
  var [content, setContent] = useState(pageElement.content);
  var [link, setLink] = useState(pageElement.link);
  var [position, setPosition] = useState(pageElement.position);
  var [cssClassName, setCssClassName] = useState(pageElement.css_class_name);

  useEffect(() => {
    if (elementType == null || elementType == undefined) {
      setElementType(2);
      pageElement.element_type_id = 2;
    }
  }, []);

  useEffect(() => {
    setElementType(pageElement.element_type_id);
    setContent(pageElement.content);
    setLink(pageElement.link);
    setPosition(pageElement.position);
    setCssClassName(pageElement.css_class_name);
  }, [pageElement]);

  var et = elementType;
  var ct = content;
  var lk = link;
  var ps = position;
  var csn = cssClassName;

  function updateType(elementType) {
    //console.log('currnet element type ', elementType);
    setElementType(elementType);
    var updatedPageElement = pageElement;
    updatedPageElement.element_type_id = elementType;
    updatePageElement(updatedPageElement);
    //console.log('updated', elementType);
  }

  if (
    elementType != null &&
    elementType != undefined &&
    pageElementTypes.length > 0
  ) {
    if (elementType == pageElementTypes[0].id) {
      return (
        <TitlePageElement
          pageElement={pageElement}
          content={content}
          setContent={setContent}
          editing={editing}
          updateType={updateType}
          et={et}
          ct={ct}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></TitlePageElement>
      );
    } else if (elementType == pageElementTypes[1].id) {
      return (
        <ParagraphPageElement
          pageElement={pageElement}
          content={content}
          setContent={setContent}
          editing={editing}
          updateType={updateType}
          et={et}
          ct={ct}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></ParagraphPageElement>
      );
    } else if (elementType == pageElementTypes[2].id) {
      return (
        <HyperLinkElement
          pageElement={pageElement}
          content={content}
          setContent={setContent}
          link={link}
          setLink={setLink}
          editing={editing}
          updateType={updateType}
          et={et}
          ct={ct}
          lk={lk}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></HyperLinkElement>
      );
    } else if (elementType == pageElementTypes[3].id) {
      return (
        <ImageElement
          pageElement={pageElement}
          link={link}
          setLink={setLink}
          editing={editing}
          updateType={updateType}
          et={et}
          lk={lk}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></ImageElement>
      );
    } else if (elementType == pageElementTypes[4].id) {
      return (
        <CodeElement
          pageElement={pageElement}
          content={content}
          setContent={setContent}
          editing={editing}
          updateType={updateType}
          et={et}
          ct={ct}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></CodeElement>
      );
    } else if (elementType == pageElementTypes[5].id) {
      return (
        <VideoElement
          pageElement={pageElement}
          link={link}
          setLink={setLink}
          editing={editing}
          updateType={updateType}
          et={et}
          lk={lk}
          csn={csn}
          pageElementTypes={pageElementTypes}
          updatePageElement={updatePageElement}
          updatePosition={updatePosition}
          deleteElement={deleteElement}
          pageClassName={pageClassName}
          cssClassName={cssClassName}
          setCssClassName={setCssClassName}
        ></VideoElement>
      );
    }
  }
}

export default DefaultPageElement;

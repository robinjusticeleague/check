///! lint/a11y/useKeyWithClickEvents
///! lint/a11y/useSemanticElements
///! lint/correctness/noChildrenProp

function bothFailing() {
  return (
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function firstDisabled() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function firstDisabledStar() {
  return (
    /* check-ignore lint/a11y/useKeyWithClickEvents: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function secondDisabled() {
  return (
    // check-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabled() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledMixed1() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    /* check-ignore lint/a11y/useSemanticElements: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledMixed2() {
  return (
    /* check-ignore lint/a11y/useKeyWithClickEvents: ... */
    // check-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function onlyLastDisabledWithSpacing() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...

    // check-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledFarAway() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint/a11y/useSemanticElements: ...
    // check-ignore lint/a11y/noRedundantAlt: ...
    // check-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused1() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint/a11y/noRedundantAlt: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused2() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unused3() {
  return (
    // check-ignore lint/style/noImplicitBoolean: ...
    // check-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function wideThenNarrow() {
  return (
    // check-ignore lint: ...
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function wideThenNarrowUnused() {
  return (
    // check-ignore lint: ...
    // check-ignore lint/security/noBlankTarget: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowThenWide() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowThenWideUnused() {
  return (
    // check-ignore lint/a11y/useKeyWithClickEvents: ...
    // check-ignore lint/a11y/useSemanticElements: ...
    // check-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function narrowUnusedThenWide() {
  return (
    // check-ignore lint/security/noBlankTarget: ...
    // check-ignore lint: ...
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function tagAndAttribute1() {
  return (
    // check-ignore lint/a11y/useSemanticElements: ...
    <span
      role="button"
      // check-ignore lint/correctness/noChildrenProp: ...
      children={[]}
    >Some text</span>
  )
}

function tagAndAttribute2() {
  return (
    // check-ignore lint/a11y/useSemanticElements: ...
    // check-ignore lint/correctness/noChildrenProp: ...
    <span
      role="button"
      children={[]}
    >Some text</span>
  )
}

function bothDisabledInsideJsx() {
  return (<>
    {/* check-ignore lint/a11y/useKeyWithClickEvents: ... */}
    {/* check-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledInsideJsxSameToken() {
  return (<>
    {/* check-ignore lint/a11y/useKeyWithClickEvents: ... */
    /* check-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledViaSameComment() {
  return (
    /*check-ignore lint/a11y/useKeyWithClickEvents: ...
    check-ignore lint/a11y/useSemanticElements: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledViaSameComment2() {
  return (
    /*
    check-ignore lint/a11y/useKeyWithClickEvents: ...
    check-ignore lint/a11y/useSemanticElements: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function bothDisabledViaSameCommentInJsx() {
  return (<>
    {/*
    check-ignore lint/a11y/useKeyWithClickEvents: ...
    check-ignore lint/a11y/useSemanticElements: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

function bothDisabledViaSameCommentInJsx2() {
  return (<>
    {/*
    check-ignore lint/a11y/useKeyWithClickEvents: ...
    check-ignore lint/a11y/useSemanticElements: ... */ <span
      role="button"
      onClick={()=>null}
    >Some text</span>}
  </>)
}

// Four examples below are still imperfect - the whole comment is reported unused.
// That is probably good enough - suppression still works as intended, but we
// don't have access to precise ranges of each part.

function unusedCaughtWithinSameComment() {
  return (
    /*
    check-ignore lint/a11y/useKeyWithClickEvents: ...
    check-ignore lint/security/noBlankTarget: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameComment2() {
  return (
    /*
    check-ignore lint/security/noBlankTarget: ...
    check-ignore lint/a11y/useKeyWithClickEvents: ...
    */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameComment3() {
  return (
    /* check-ignore lint/security/noBlankTarget: ...
    check-ignore lint/a11y/useKeyWithClickEvents: ... */
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  )
}

function unusedCaughtWithinSameCommentInJsx() {
  return (<>
    {/* check-ignore lint/security/noBlankTarget: ...
    check-ignore lint/a11y/useKeyWithClickEvents: ... */}
    <span
      role="button"
      onClick={()=>null}
    >Some text</span>
  </>)
}

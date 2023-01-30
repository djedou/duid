pub(crate) fn base_styles() -> String {
    r##"html {
    font-family: sans-serif;
        -ms-text-size-adjust: 100%;
        -webkit-text-size-adjust: 100%;
    }
    body {
        margin: 0;
    }
    article,
    aside,
    details,
    figcaption,
    figure,
    footer,
    header,
    main,
    menu,
    nav,
    section {
        display: block;
    }
    summary {
        display: list-item;
    }
    audio,
    canvas,
    progress,
    video {
        display: inline-block;
    }
    audio:not([controls]) {
        display: none;
        height: 0;
    }
    progress {
        vertical-align: baseline;
    }
    template,
    [hidden] {
        display: none !important;
    }
    a {
        background-color: transparent;
    }
    b,
    strong {
        font-weight: inherit;
    }
    dfn {
        font-style: italic;
    }
    h1 {
        font-size: 2em;
        margin: 0.67em 0;
    }
    mark {
        background-color: var(--color-attention-subtle);
        color: var(--color-text-primary);
    }
    small {
        font-size: 80%;
    }
    sub,
    sup {
        font-size: 75%;
        line-height: 0;
        position: relative;
        vertical-align: baseline;
    }
    sub {
        bottom: -0.25em;
    }
    sup {
        top: -0.5em;
    }
    img {
        border-style: none;
    }
    svg:not(:root) {
        overflow: hidden;
    }
    code,
    kbd,
    pre,
    samp {
        font-family: monospace;
        font-size: 1em;
    }
    figure {
        margin: 1em 40px;
    }
    hr {
        box-sizing: content-box;
        height: 0;
        overflow: visible;
    }
    button,
    input,
    select,
    textarea {
        font: inherit;
        margin: 0;
    }
    optgroup {
        font-weight: 600;
    }
    button,
    input {
        overflow: visible;
    }
    button,
    select {
        text-transform: none;
    }
    button,
    html [type=button],
    [type=reset],
    [type=submit] {
        -webkit-appearance: button;
    }
    fieldset {
        border: 1px solid #c0c0c0;
        margin: 0 2px;
        padding: 0.35em 0.625em 0.75em;
    }
    legend {
      box-sizing: border-box;
      color: inherit;
      display: table;
      max-width: 100%;
      padding: 0;
      white-space: normal;
    }
    textarea {
      overflow: auto;
    }
    [type=checkbox],
    [type=radio] {
      box-sizing: border-box;
      padding: 0;
    }
    [type=number]::-webkit-inner-spin-button,
    [type=number]::-webkit-outer-spin-button {
      height: auto;
    }
    [type=search]::-webkit-search-cancel-button,
    [type=search]::-webkit-search-decoration {
      -webkit-appearance: none;
    }
    ::-webkit-input-placeholder {
      color: inherit;
      opacity: 0.54;
    }
    ::-webkit-file-upload-button {
      -webkit-appearance: button;
      font: inherit;
    }
    * {
      box-sizing: border-box;
    }
    input,
    select,
    textarea,
    button {
      font-family: inherit;
      font-size: inherit;
      line-height: inherit;
    }
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
      font-size: 14px;
      line-height: 1.5;
    }
    a {
      text-decoration: none;
    }
    a:hover {
      text-decoration: underline;
    }
    b,
    strong {
      font-weight: 600;
    }
    hr {
      height: 0;
      margin: 15px 0;
      overflow: hidden;
      background: transparent;
      border: 0;
    }
    hr::before {
      display: table;
      content: "";
    }
    hr::after {
      display: table;
      clear: both;
      content: "";
    }
    table {
      border-spacing: 0;
      border-collapse: collapse;
    }
    td,
    th {
      padding: 0;
    }
    button {
      cursor: pointer;
      border-radius: 0;
    }
    [hidden][hidden] {
      display: none !important;
    }
    details summary {
      cursor: pointer;
    }
    details:not([open]) > *:not(summary) {
      display: none !important;
    }
    a:focus,
    button:focus,
    [role=button]:focus,
    input[type=radio]:focus,
    input[type=checkbox]:focus {
      outline-offset: -2px;
      box-shadow: none;
    }
    a:focus:not(:focus-visible),
    button:focus:not(:focus-visible),
    [role=button]:focus:not(:focus-visible),
    input[type=radio]:focus:not(:focus-visible),
    input[type=checkbox]:focus:not(:focus-visible) {
      outline: solid 1px transparent;
    }
    a:focus-visible,
    button:focus-visible,
    [role=button]:focus-visible,
    input[type=radio]:focus-visible,
    input[type=checkbox]:focus-visible {
      outline-offset: -2px;
      box-shadow: none;
    }
    a:not([class]):focus, a:not([class]):focus-visible,
    input[type=radio]:focus,
    input[type=radio]:focus-visible,
    input[type=checkbox]:focus,
    input[type=checkbox]:focus-visible {
      outline-offset: 0;
    }
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
      margin-top: 0;
      margin-bottom: 0;
    }
    h1 {
      font-size: 32px;
      font-weight: 600;
    }
    h2 {
      font-size: 24px;
      font-weight: 600;
    }
    h3 {
      font-size: 20px;
      font-weight: 600;
    }
    h4 {
      font-size: 16px;
      font-weight: 600;
    }
    h5 {
      font-size: 14px;
      font-weight: 600;
    }
    h6 {
      font-size: 12px;
      font-weight: 600;
    }
    p {
      margin-top: 0;
      margin-bottom: 10px;
    }
    small {
      font-size: 90%;
    }
    blockquote {
      margin: 0;
    }
    ul,
    ol {
      padding-left: 0;
      margin-top: 0;
      margin-bottom: 0;
    }
    ol ol,
    ul ol {
      list-style-type: lower-roman;
    }
    ul ul ol,
    ul ol ol,
    ol ul ol,
    ol ol ol {
      list-style-type: lower-alpha;
    }
    dd {
      margin-left: 0;
    }
    tt,
    code,
    samp {
      font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
      font-size: 12px;
    }
    pre {
      margin-top: 0;
      margin-bottom: 0;
      font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
      font-size: 12px;
    }
    .octicon {
      display: inline-block;
      overflow: visible !important;
      vertical-align: text-bottom;
      fill: currentColor;
    }
    "##.to_string().replace("\n", "").replace("  ", "")
}















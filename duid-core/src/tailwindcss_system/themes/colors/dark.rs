
pub fn dark_variables() -> String {
    r##":root {
        --inherit: inherit;
        --current: currentColor;
        --transparent: transparent;
        --color-black: #ffffff;
        --color-white: #1b1f24;
        
        --color-btn-bg: #21262d;
        --color-btn-border: rgba(240,246,252,0.1);
        --color-btn-shadow: 0 0 transparent;
        --color-btn-inset-shadow: 0 0 transparent;
        --color-btn-hover-bg: #30363d;
        --color-btn-hover-border: #8b949e;
        --color-btn-active-bg: hsla(212,12%,18%,1);
        --color-btn-active-border: #6e7681;
        --color-btn-selected-bg: #161b22;
        --color-btn-focus-bg: #21262d;
        --color-btn-focus-border: #8b949e;
        --color-btn-focus-shadow: 0 0 0 3px rgba(139,148,158,0.3);
        --color-btn-shadow-active: inset 0 0.15em 0.3em rgba(1,4,9,0.15);
        --color-btn-shadow-input-focus: 0 0 0 0.2em rgba(31,111,235,0.3);
        --color-btn-counter-bg: #30363d;
        --color-btn-filled-bg: #238636;
        --color-btn-filled-border: rgba(240,246,252,0.1);
        --color-btn-filled-shadow: 0 0 transparent;
        --color-btn-filled-inset-shadow: 0 0 transparent;
        --color-btn-filled-hover-bg: #2ea043;
        --color-btn-filled-hover-border: rgba(240,246,252,0.1);
        --color-btn-filled-selected-bg: #238636;
        --color-btn-filled-selected-shadow: 0 0 transparent;
        --color-btn-filled-disabled-text: rgba(255,255,255,0.5);
        --color-btn-filled-disabled-bg: rgba(35,134,54,0.6);
        --color-btn-filled-disabled-border: rgba(240,246,252,0.1);
        --color-btn-filled-focus-bg: #238636;
        --color-btn-filled-focus-border: rgba(240,246,252,0.1);
        --color-btn-filled-focus-shadow: 0 0 0 3px rgba(46,164,79,0.4);
        --color-btn-filled-counter-bg: rgba(255,255,255,0.2);
        --color-btn-outline-text: #58a6ff;
        --color-btn-outline-hover-text: #58a6ff;
        --color-btn-outline-hover-bg: #30363d;
        --color-btn-outline-hover-border: rgba(240,246,252,0.1);
        --color-btn-outline-hover-shadow: 0 1px 0 rgba(1,4,9,0.1);
        --color-btn-outline-hover-inset-shadow: inset 0 1px 0 rgba(255,255,255,0.03);
        --color-btn-outline-hover-counter-bg: rgba(255,255,255,0.2);
        --color-btn-outline-selected-bg: #0d419d;
        --color-btn-outline-selected-border: rgba(240,246,252,0.1);
        --color-btn-outline-selected-shadow: 0 0 transparent;
        --color-btn-outline-disabled-text: rgba(88,166,255,0.5);
        --color-btn-outline-disabled-bg: #0d1117;
        --color-btn-outline-disabled-counter-bg: rgba(31,111,235,0.05);
        --color-btn-outline-focus-border: rgba(240,246,252,0.1);
        --color-btn-outline-focus-shadow: 0 0 0 3px rgba(17,88,199,0.4);
        --color-btn-outline-counter-bg: rgba(31,111,235,0.1);
    }"##.to_string().replace("\n", "").replace("  ", "")
}

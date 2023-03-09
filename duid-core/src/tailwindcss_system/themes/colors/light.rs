
pub fn light_variables() -> String {
    r##":root {
        --inherit: inherit;
        --current: currentColor;
        --transparent: transparent;
        --color-black: #1b1f24;
        --color-white: #ffffff;
    
        --color-btn-border: rgba(27,31,36,0.15);
        --color-btn-shadow: 0 1px 0 rgba(27,31,36,0.04);
        --color-btn-inset-shadow: inset 0 1px 0 rgba(255,255,255,0.25);
        --color-btn-hover-bg: #f3f4f6;
        --color-btn-hover-border: rgba(27,31,36,0.15);
        --color-btn-active-bg: hsla(220,14%,93%,1);
        --color-btn-active-border: rgba(27,31,36,0.15);
        --color-btn-selected-bg: hsla(220,14%,94%,1);
        --color-btn-focus-bg: #f6f8fa;
        --color-btn-focus-border: rgba(27,31,36,0.15);
        --color-btn-focus-shadow: 0 0 0 3px rgba(9,105,218,0.3);
        --color-btn-shadow-active: inset 0 0.15em 0.3em rgba(27,31,36,0.15);
        --color-btn-shadow-input-focus: 0 0 0 0.2em rgba(9,105,218,0.3);
        --color-btn-counter-bg: rgba(27,31,36,0.08);
        --color-btn-filled-bg: #2da44e;
        --color-btn-filled-border: rgba(27,31,36,0.15);
        --color-btn-filled-shadow: 0 1px 0 rgba(27,31,36,0.1);
        --color-btn-filled-inset-shadow: inset 0 1px 0 rgba(255,255,255,0.03);
        --color-btn-filled-hover-bg: #2c974b;
        --color-btn-filled-hover-border: rgba(27,31,36,0.15);
        --color-btn-filled-selected-bg: hsla(137,55%,36%,1);
        --color-btn-filled-selected-shadow: inset 0 1px 0 rgba(0,45,17,0.2);
        --color-btn-filled-disabled-text: rgba(255,255,255,0.8);
        --color-btn-filled-disabled-bg: #94d3a2;
        --color-btn-filled-disabled-border: rgba(27,31,36,0.15);
        --color-btn-filled-focus-bg: #2da44e;
        --color-btn-filled-focus-border: rgba(27,31,36,0.15);
        --color-btn-filled-focus-shadow: 0 0 0 3px rgba(45,164,78,0.4);
        --color-btn-filled-icon: rgba(255,255,255,0.8);
        --color-btn-filled-counter-bg: rgba(255,255,255,0.2);
        --color-btn-outline-text: #0969da;
        --color-btn-outline-hover-bg: #0969da;
        --color-btn-outline-hover-border: rgba(27,31,36,0.15);
        --color-btn-outline-hover-shadow: 0 1px 0 rgba(27,31,36,0.1);
        --color-btn-outline-hover-inset-shadow: inset 0 1px 0 rgba(255,255,255,0.03);
        --color-btn-outline-hover-counter-bg: rgba(255,255,255,0.2);
        --color-btn-outline-selected-bg: hsla(212,92%,42%,1);
        --color-btn-outline-selected-border: rgba(27,31,36,0.15);
        --color-btn-outline-selected-shadow: inset 0 1px 0 rgba(0,33,85,0.2);
        --color-btn-outline-disabled-text: rgba(9,105,218,0.5);
        --color-btn-outline-disabled-bg: #f6f8fa;
        --color-btn-outline-disabled-counter-bg: rgba(9,105,218,0.05);
        --color-btn-outline-focus-border: rgba(27,31,36,0.15);
        --color-btn-outline-focus-shadow: 0 0 0 3px rgba(5,80,174,0.4);
        --color-btn-outline-counter-bg: rgba(9,105,218,0.1);
    }"##.to_string().replace("\n", "").replace("  ", "")
}

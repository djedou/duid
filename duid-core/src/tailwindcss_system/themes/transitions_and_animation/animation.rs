use std::collections::HashMap;


pub(crate) fn animation() -> HashMap<String, String> {
    let mut animation = HashMap::new();
    let _ = animation.insert("animate-none".to_owned(), "animation: none;".to_owned());
    let _ = animation.insert("animate-spin".to_owned(), "animation: spin 1s linear infinite;@keyframes spin {from {transform: rotate(0deg);}to {transform: rotate(360deg);}}".to_owned());
    let _ = animation.insert("animate-ping".to_owned(), "animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;@keyframes ping {75%, 100% {transform: scale(2);opacity: 0;}}".to_owned());
    let _ = animation.insert("animate-pulse".to_owned(), "animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;@keyframes pulse {0%, 100% {opacity: 1;}50% {opacity: .5;}}".to_owned());
    let _ = animation.insert("animate-bounce".to_owned(), "animation: bounce 1s infinite;@keyframes bounce {0%, 100% {transform: translateY(-25%);animation-timing-function: cubic-bezier(0.8, 0, 1, 1);}50% {transform: translateY(0);animation-timing-function: cubic-bezier(0, 0, 0.2, 1);}}".to_owned());

    animation
}
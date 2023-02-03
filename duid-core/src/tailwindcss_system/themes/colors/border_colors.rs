use std::collections::HashMap;

pub fn border_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("border-inherit".to_owned(), "border-color: var(--inherit);".to_owned());
    let _ = styles.insert("border-current".to_owned(), "border-color: var(--current);".to_owned());
    let _ = styles.insert("border-transparent".to_owned(), "border-color: var(--transparent);".to_owned());
    let _ = styles.insert("border-black".to_owned(), "border-color: var(--color-black);".to_owned());
    let _ = styles.insert("border-white".to_owned(), "border-color: var(--color-white);".to_owned());
    let _ = styles.insert("border-slate-50".to_owned(), "border-color: var(--color-slate-50);".to_owned());
    let _ = styles.insert("border-slate-100".to_owned(), "border-color: var(--color-slate-100);".to_owned());
    let _ = styles.insert("border-slate-200".to_owned(), "border-color: var(--color-slate-200);".to_owned());
    let _ = styles.insert("border-slate-300".to_owned(), "border-color: var(--color-slate-300);".to_owned());
    let _ = styles.insert("border-slate-400".to_owned(), "border-color: var(--color-slate-400);".to_owned());
    let _ = styles.insert("border-slate-500".to_owned(), "border-color: var(--color-slate-500);".to_owned());
    let _ = styles.insert("border-slate-600".to_owned(), "border-color: var(--color-slate-600);".to_owned());
    let _ = styles.insert("border-slate-700".to_owned(), "border-color: var(--color-slate-700);".to_owned());
    let _ = styles.insert("border-slate-800".to_owned(), "border-color: var(--color-slate-800);".to_owned());
    let _ = styles.insert("border-slate-900".to_owned(), "border-color: var(--color-slate-900);".to_owned());
    let _ = styles.insert("border-gray-50".to_owned(), "border-color: var(--color-gray-50);".to_owned());
    let _ = styles.insert("border-gray-100".to_owned(), "border-color: var(--color-gray-100);".to_owned());
    let _ = styles.insert("border-gray-200".to_owned(), "border-color: var(--color-gray-200);".to_owned());
    let _ = styles.insert("border-gray-300".to_owned(), "border-color: var(--color-gray-300);".to_owned());
    let _ = styles.insert("border-gray-400".to_owned(), "border-color: var(--color-gray-400);".to_owned());
    let _ = styles.insert("border-gray-500".to_owned(), "border-color: var(--color-gray-500);".to_owned());
    let _ = styles.insert("border-gray-600".to_owned(), "border-color: var(--color-gray-600);".to_owned());
    let _ = styles.insert("border-gray-700".to_owned(), "border-color: var(--color-gray-700);".to_owned());
    let _ = styles.insert("border-gray-800".to_owned(), "border-color: var(--color-gray-800);".to_owned());
    let _ = styles.insert("border-gray-900".to_owned(), "border-color: var(--color-gray-900);".to_owned());
    let _ = styles.insert("border-zinc-50".to_owned(), "border-color: var(--color-zinc-50);".to_owned());
    let _ = styles.insert("border-zinc-100".to_owned(), "border-color: var(--color-zinc-100);".to_owned());
    let _ = styles.insert("border-zinc-200".to_owned(), "border-color: var(--color-zinc-200);".to_owned());
    let _ = styles.insert("border-zinc-300".to_owned(), "border-color: var(--color-zinc-300);".to_owned());
    let _ = styles.insert("border-zinc-400".to_owned(), "border-color: var(--color-zinc-400);".to_owned());
    let _ = styles.insert("border-zinc-500".to_owned(), "border-color: var(--color-zinc-500);".to_owned());
    let _ = styles.insert("border-zinc-600".to_owned(), "border-color: var(--color-zinc-600);".to_owned());
    let _ = styles.insert("border-zinc-700".to_owned(), "border-color: var(--color-zinc-700);".to_owned());
    let _ = styles.insert("border-zinc-800".to_owned(), "border-color: var(--color-zinc-800);".to_owned());
    let _ = styles.insert("border-zinc-900".to_owned(), "border-color: var(--color-zinc-900);".to_owned());
    let _ = styles.insert("border-neutral-50".to_owned(), "border-color: var(--color-neutral-50);".to_owned());
    let _ = styles.insert("border-neutral-100".to_owned(), "border-color: var(--color-neutral-100);".to_owned());
    let _ = styles.insert("border-neutral-200".to_owned(), "border-color: var(--color-neutral-200);".to_owned());
    let _ = styles.insert("border-neutral-300".to_owned(), "border-color: var(--color-neutral-300);".to_owned());
    let _ = styles.insert("border-neutral-400".to_owned(), "border-color: var(--color-neutral-400);".to_owned());
    let _ = styles.insert("border-neutral-500".to_owned(), "border-color: var(--color-neutral-500);".to_owned());
    let _ = styles.insert("border-neutral-600".to_owned(), "border-color: var(--color-neutral-600);".to_owned());
    let _ = styles.insert("border-neutral-700".to_owned(), "border-color: var(--color-neutral-700);".to_owned());
    let _ = styles.insert("border-neutral-800".to_owned(), "border-color: var(--color-neutral-800);".to_owned());
    let _ = styles.insert("border-neutral-900".to_owned(), "border-color: var(--color-neutral-900);".to_owned());
    let _ = styles.insert("border-stone-50".to_owned(), "border-color: var(--color-stone-50);".to_owned());
    let _ = styles.insert("border-stone-100".to_owned(), "border-color: var(--color-stone-100);".to_owned());
    let _ = styles.insert("border-stone-200".to_owned(), "border-color: var(--color-stone-200);".to_owned());
    let _ = styles.insert("border-stone-300".to_owned(), "border-color: var(--color-stone-300);".to_owned());
    let _ = styles.insert("border-stone-400".to_owned(), "border-color: var(--color-stone-400);".to_owned());
    let _ = styles.insert("border-stone-500".to_owned(), "border-color: var(--color-stone-500);".to_owned());
    let _ = styles.insert("border-stone-600".to_owned(), "border-color: var(--color-stone-600);".to_owned());
    let _ = styles.insert("border-stone-700".to_owned(), "border-color: var(--color-stone-700);".to_owned());
    let _ = styles.insert("border-stone-800".to_owned(), "border-color: var(--color-stone-800);".to_owned());
    let _ = styles.insert("border-stone-900".to_owned(), "border-color: var(--color-stone-900);".to_owned());
    let _ = styles.insert("border-red-50".to_owned(), "border-color: var(--color-red-50);".to_owned());
    let _ = styles.insert("border-red-100".to_owned(), "border-color: var(--color-red-100);".to_owned());
    let _ = styles.insert("border-red-200".to_owned(), "border-color: var(--color-red-200);".to_owned());
    let _ = styles.insert("border-red-300".to_owned(), "border-color: var(--color-red-300);".to_owned());
    let _ = styles.insert("border-red-400".to_owned(), "border-color: var(--color-red-400);".to_owned());
    let _ = styles.insert("border-red-500".to_owned(), "border-color: var(--color-red-500);".to_owned());
    let _ = styles.insert("border-red-600".to_owned(), "border-color: var(--color-red-600);".to_owned());
    let _ = styles.insert("border-red-700".to_owned(), "border-color: var(--color-red-700);".to_owned());
    let _ = styles.insert("border-red-800".to_owned(), "border-color: var(--color-red-800);".to_owned());
    let _ = styles.insert("border-red-900".to_owned(), "border-color: var(--color-red-900);".to_owned());
    let _ = styles.insert("border-orange-50".to_owned(), "border-color: var(--color-orange-50);".to_owned());
    let _ = styles.insert("border-orange-100".to_owned(), "border-color: var(--color-orange-100);".to_owned());
    let _ = styles.insert("border-orange-200".to_owned(), "border-color: var(--color-orange-200);".to_owned());
    let _ = styles.insert("border-orange-300".to_owned(), "border-color: var(--color-orange-300);".to_owned());
    let _ = styles.insert("border-orange-400".to_owned(), "border-color: var(--color-orange-400);".to_owned());
    let _ = styles.insert("border-orange-500".to_owned(), "border-color: var(--color-orange-500);".to_owned());
    let _ = styles.insert("border-orange-600".to_owned(), "border-color: var(--color-orange-600);".to_owned());
    let _ = styles.insert("border-orange-700".to_owned(), "border-color: var(--color-orange-700);".to_owned());
    let _ = styles.insert("border-orange-800".to_owned(), "border-color: var(--color-orange-800);".to_owned());
    let _ = styles.insert("border-orange-900".to_owned(), "border-color: var(--color-orange-900);".to_owned());
    let _ = styles.insert("border-amber-50".to_owned(), "border-color: var(--color-amber-50);".to_owned());
    let _ = styles.insert("border-amber-100".to_owned(), "border-color: var(--color-amber-100);".to_owned());
    let _ = styles.insert("border-amber-200".to_owned(), "border-color: var(--color-amber-200);".to_owned());
    let _ = styles.insert("border-amber-300".to_owned(), "border-color: var(--color-amber-300);".to_owned());
    let _ = styles.insert("border-amber-400".to_owned(), "border-color: var(--color-amber-400);".to_owned());
    let _ = styles.insert("border-amber-500".to_owned(), "border-color: var(--color-amber-500);".to_owned());
    let _ = styles.insert("border-amber-600".to_owned(), "border-color: var(--color-amber-600);".to_owned());
    let _ = styles.insert("border-amber-700".to_owned(), "border-color: var(--color-amber-700);".to_owned());
    let _ = styles.insert("border-amber-800".to_owned(), "border-color: var(--color-amber-800);".to_owned());
    let _ = styles.insert("border-amber-900".to_owned(), "border-color: var(--color-amber-900);".to_owned());
    let _ = styles.insert("border-yellow-50".to_owned(), "border-color: var(--color-yellow-50);".to_owned());
    let _ = styles.insert("border-yellow-100".to_owned(), "border-color: var(--color-yellow-100);".to_owned());
    let _ = styles.insert("border-yellow-200".to_owned(), "border-color: var(--color-yellow-200);".to_owned());
    let _ = styles.insert("border-yellow-300".to_owned(), "border-color: var(--color-yellow-300);".to_owned());
    let _ = styles.insert("border-yellow-400".to_owned(), "border-color: var(--color-yellow-400);".to_owned());
    let _ = styles.insert("border-yellow-500".to_owned(), "border-color: var(--color-yellow-500);".to_owned());
    let _ = styles.insert("border-yellow-600".to_owned(), "border-color: var(--color-yellow-600);".to_owned());
    let _ = styles.insert("border-yellow-700".to_owned(), "border-color: var(--color-yellow-700);".to_owned());
    let _ = styles.insert("border-yellow-800".to_owned(), "border-color: var(--color-yellow-800);".to_owned());
    let _ = styles.insert("border-yellow-900".to_owned(), "border-color: var(--color-yellow-900);".to_owned());
    let _ = styles.insert("border-lime-50".to_owned(), "border-color: var(--color-lime-50);".to_owned());
    let _ = styles.insert("border-lime-100".to_owned(), "border-color: var(--color-lime-100);".to_owned());
    let _ = styles.insert("border-lime-200".to_owned(), "border-color: var(--color-lime-200);".to_owned());
    let _ = styles.insert("border-lime-300".to_owned(), "border-color: var(--color-lime-300);".to_owned());
    let _ = styles.insert("border-lime-400".to_owned(), "border-color: var(--color-lime-400);".to_owned());
    let _ = styles.insert("border-lime-500".to_owned(), "border-color: var(--color-lime-500);".to_owned());
    let _ = styles.insert("border-lime-600".to_owned(), "border-color: var(--color-lime-600);".to_owned());
    let _ = styles.insert("border-lime-700".to_owned(), "border-color: var(--color-lime-700);".to_owned());
    let _ = styles.insert("border-lime-800".to_owned(), "border-color: var(--color-lime-800);".to_owned());
    let _ = styles.insert("border-lime-900".to_owned(), "border-color: var(--color-lime-900);".to_owned());
    let _ = styles.insert("border-green-50".to_owned(), "border-color: var(--color-green-50);".to_owned());
    let _ = styles.insert("border-green-100".to_owned(), "border-color: var(--color-green-100);".to_owned());
    let _ = styles.insert("border-green-200".to_owned(), "border-color: var(--color-green-200);".to_owned());
    let _ = styles.insert("border-green-300".to_owned(), "border-color: var(--color-green-300);".to_owned());
    let _ = styles.insert("border-green-400".to_owned(), "border-color: var(--color-green-400);".to_owned());
    let _ = styles.insert("border-green-500".to_owned(), "border-color: var(--color-green-500);".to_owned());
    let _ = styles.insert("border-green-600".to_owned(), "border-color: var(--color-green-600);".to_owned());
    let _ = styles.insert("border-green-700".to_owned(), "border-color: var(--color-green-700);".to_owned());
    let _ = styles.insert("border-green-800".to_owned(), "border-color: var(--color-green-800);".to_owned());
    let _ = styles.insert("border-green-900".to_owned(), "border-color: var(--color-green-900);".to_owned());
    let _ = styles.insert("border-emerald-50".to_owned(), "border-color: var(--color-emerald-50);".to_owned());
    let _ = styles.insert("border-emerald-100".to_owned(), "border-color: var(--color-emerald-100);".to_owned());
    let _ = styles.insert("border-emerald-200".to_owned(), "border-color: var(--color-emerald-200);".to_owned());
    let _ = styles.insert("border-emerald-300".to_owned(), "border-color: var(--color-emerald-300);".to_owned());
    let _ = styles.insert("border-emerald-400".to_owned(), "border-color: var(--color-emerald-400);".to_owned());
    let _ = styles.insert("border-emerald-500".to_owned(), "border-color: var(--color-emerald-500);".to_owned());
    let _ = styles.insert("border-emerald-600".to_owned(), "border-color: var(--color-emerald-600);".to_owned());
    let _ = styles.insert("border-emerald-700".to_owned(), "border-color: var(--color-emerald-700);".to_owned());
    let _ = styles.insert("border-emerald-800".to_owned(), "border-color: var(--color-emerald-800);".to_owned());
    let _ = styles.insert("border-emerald-900".to_owned(), "border-color: var(--color-emerald-900);".to_owned());
    let _ = styles.insert("border-teal-50".to_owned(), "border-color: var(--color-teal-50);".to_owned());
    let _ = styles.insert("border-teal-100".to_owned(), "border-color: var(--color-teal-100);".to_owned());
    let _ = styles.insert("border-teal-200".to_owned(), "border-color: var(--color-teal-200);".to_owned());
    let _ = styles.insert("border-teal-300".to_owned(), "border-color: var(--color-teal-300);".to_owned());
    let _ = styles.insert("border-teal-400".to_owned(), "border-color: var(--color-teal-400);".to_owned());
    let _ = styles.insert("border-teal-500".to_owned(), "border-color: var(--color-teal-500);".to_owned());
    let _ = styles.insert("border-teal-600".to_owned(), "border-color: var(--color-teal-600);".to_owned());
    let _ = styles.insert("border-teal-700".to_owned(), "border-color: var(--color-teal-700);".to_owned());
    let _ = styles.insert("border-teal-800".to_owned(), "border-color: var(--color-teal-800);".to_owned());
    let _ = styles.insert("border-teal-900".to_owned(), "border-color: var(--color-teal-900);".to_owned());
    let _ = styles.insert("border-cyan-50".to_owned(), "border-color: var(--color-cyan-50);".to_owned());
    let _ = styles.insert("border-cyan-100".to_owned(), "border-color: var(--color-cyan-100);".to_owned());
    let _ = styles.insert("border-cyan-200".to_owned(), "border-color: var(--color-cyan-200);".to_owned());
    let _ = styles.insert("border-cyan-300".to_owned(), "border-color: var(--color-cyan-300);".to_owned());
    let _ = styles.insert("border-cyan-400".to_owned(), "border-color: var(--color-cyan-400);".to_owned());
    let _ = styles.insert("border-cyan-500".to_owned(), "border-color: var(--color-cyan-500);".to_owned());
    let _ = styles.insert("border-cyan-600".to_owned(), "border-color: var(--color-cyan-600);".to_owned());
    let _ = styles.insert("border-cyan-700".to_owned(), "border-color: var(--color-cyan-700);".to_owned());
    let _ = styles.insert("border-cyan-800".to_owned(), "border-color: var(--color-cyan-800);".to_owned());
    let _ = styles.insert("border-cyan-900".to_owned(), "border-color: var(--color-cyan-900);".to_owned());
    let _ = styles.insert("border-sky-50".to_owned(), "border-color: var(--color-sky-50);".to_owned());
    let _ = styles.insert("border-sky-100".to_owned(), "border-color: var(--color-sky-100);".to_owned());
    let _ = styles.insert("border-sky-200".to_owned(), "border-color: var(--color-sky-200);".to_owned());
    let _ = styles.insert("border-sky-300".to_owned(), "border-color: var(--color-sky-300);".to_owned());
    let _ = styles.insert("border-sky-400".to_owned(), "border-color: var(--color-sky-400);".to_owned());
    let _ = styles.insert("border-sky-500".to_owned(), "border-color: var(--color-sky-500);".to_owned());
    let _ = styles.insert("border-sky-600".to_owned(), "border-color: var(--color-sky-600);".to_owned());
    let _ = styles.insert("border-sky-700".to_owned(), "border-color: var(--color-sky-700);".to_owned());
    let _ = styles.insert("border-sky-800".to_owned(), "border-color: var(--color-sky-800);".to_owned());
    let _ = styles.insert("border-sky-900".to_owned(), "border-color: var(--color-sky-900);".to_owned());
    let _ = styles.insert("border-blue-50".to_owned(), "border-color: var(--color-blue-50);".to_owned());
    let _ = styles.insert("border-blue-100".to_owned(), "border-color: var(--color-blue-100);".to_owned());
    let _ = styles.insert("border-blue-200".to_owned(), "border-color: var(--color-blue-200);".to_owned());
    let _ = styles.insert("border-blue-300".to_owned(), "border-color: var(--color-blue-300);".to_owned());
    let _ = styles.insert("border-blue-400".to_owned(), "border-color: var(--color-blue-400);".to_owned());
    let _ = styles.insert("border-blue-500".to_owned(), "border-color: var(--color-blue-500);".to_owned());
    let _ = styles.insert("border-blue-600".to_owned(), "border-color: var(--color-blue-600);".to_owned());
    let _ = styles.insert("border-blue-700".to_owned(), "border-color: var(--color-blue-700);".to_owned());
    let _ = styles.insert("border-blue-800".to_owned(), "border-color: var(--color-blue-800);".to_owned());
    let _ = styles.insert("border-blue-900".to_owned(), "border-color: var(--color-blue-900);".to_owned());
    let _ = styles.insert("border-indigo-50".to_owned(), "border-color: var(--color-indigo-50);".to_owned());
    let _ = styles.insert("border-indigo-100".to_owned(), "border-color: var(--color-indigo-100);".to_owned());
    let _ = styles.insert("border-indigo-200".to_owned(), "border-color: var(--color-indigo-200);".to_owned());
    let _ = styles.insert("border-indigo-300".to_owned(), "border-color: var(--color-indigo-300);".to_owned());
    let _ = styles.insert("border-indigo-400".to_owned(), "border-color: var(--color-indigo-400);".to_owned());
    let _ = styles.insert("border-indigo-500".to_owned(), "border-color: var(--color-indigo-500);".to_owned());
    let _ = styles.insert("border-indigo-600".to_owned(), "border-color: var(--color-indigo-600);".to_owned());
    let _ = styles.insert("border-indigo-700".to_owned(), "border-color: var(--color-indigo-700);".to_owned());
    let _ = styles.insert("border-indigo-800".to_owned(), "border-color: var(--color-indigo-800);".to_owned());
    let _ = styles.insert("border-indigo-900".to_owned(), "border-color: var(--color-indigo-900);".to_owned());
    let _ = styles.insert("border-violet-50".to_owned(), "border-color: var(--color-violet-50);".to_owned());
    let _ = styles.insert("border-violet-100".to_owned(), "border-color: var(--color-violet-100);".to_owned());
    let _ = styles.insert("border-violet-200".to_owned(), "border-color: var(--color-violet-200);".to_owned());
    let _ = styles.insert("border-violet-300".to_owned(), "border-color: var(--color-violet-300);".to_owned());
    let _ = styles.insert("border-violet-400".to_owned(), "border-color: var(--color-violet-400);".to_owned());
    let _ = styles.insert("border-violet-500".to_owned(), "border-color: var(--color-violet-500);".to_owned());
    let _ = styles.insert("border-violet-600".to_owned(), "border-color: var(--color-violet-600);".to_owned());
    let _ = styles.insert("border-violet-700".to_owned(), "border-color: var(--color-violet-700);".to_owned());
    let _ = styles.insert("border-violet-800".to_owned(), "border-color: var(--color-violet-800);".to_owned());
    let _ = styles.insert("border-violet-900".to_owned(), "border-color: var(--color-violet-900);".to_owned());
    let _ = styles.insert("border-purple-50".to_owned(), "border-color: var(--color-purple-50);".to_owned());
    let _ = styles.insert("border-purple-100".to_owned(), "border-color: var(--color-purple-100);".to_owned());
    let _ = styles.insert("border-purple-200".to_owned(), "border-color: var(--color-purple-200);".to_owned());
    let _ = styles.insert("border-purple-300".to_owned(), "border-color: var(--color-purple-300);".to_owned());
    let _ = styles.insert("border-purple-400".to_owned(), "border-color: var(--color-purple-400);".to_owned());
    let _ = styles.insert("border-purple-500".to_owned(), "border-color: var(--color-purple-500);".to_owned());
    let _ = styles.insert("border-purple-600".to_owned(), "border-color: var(--color-purple-600);".to_owned());
    let _ = styles.insert("border-purple-700".to_owned(), "border-color: var(--color-purple-700);".to_owned());
    let _ = styles.insert("border-purple-800".to_owned(), "border-color: var(--color-purple-800);".to_owned());
    let _ = styles.insert("border-purple-900".to_owned(), "border-color: var(--color-purple-900);".to_owned());
    let _ = styles.insert("border-fuchsia-50".to_owned(), "border-color: var(--color-fuchsia-50);".to_owned());
    let _ = styles.insert("border-fuchsia-100".to_owned(), "border-color: var(--color-fuchsia-100);".to_owned());
    let _ = styles.insert("border-fuchsia-200".to_owned(), "border-color: var(--color-fuchsia-200);".to_owned());
    let _ = styles.insert("border-fuchsia-300".to_owned(), "border-color: var(--color-fuchsia-300);".to_owned());
    let _ = styles.insert("border-fuchsia-400".to_owned(), "border-color: var(--color-fuchsia-400);".to_owned());
    let _ = styles.insert("border-fuchsia-500".to_owned(), "border-color: var(--color-fuchsia-500);".to_owned());
    let _ = styles.insert("border-fuchsia-600".to_owned(), "border-color: var(--color-fuchsia-600);".to_owned());
    let _ = styles.insert("border-fuchsia-700".to_owned(), "border-color: var(--color-fuchsia-700);".to_owned());
    let _ = styles.insert("border-fuchsia-800".to_owned(), "border-color: var(--color-fuchsia-800);".to_owned());
    let _ = styles.insert("border-fuchsia-900".to_owned(), "border-color: var(--color-fuchsia-900);".to_owned());
    let _ = styles.insert("border-pink-50".to_owned(), "border-color: var(--color-pink-50);".to_owned());
    let _ = styles.insert("border-pink-100".to_owned(), "border-color: var(--color-pink-100);".to_owned());
    let _ = styles.insert("border-pink-200".to_owned(), "border-color: var(--color-pink-200);".to_owned());
    let _ = styles.insert("border-pink-300".to_owned(), "border-color: var(--color-pink-300);".to_owned());
    let _ = styles.insert("border-pink-400".to_owned(), "border-color: var(--color-pink-400);".to_owned());
    let _ = styles.insert("border-pink-500".to_owned(), "border-color: var(--color-pink-500);".to_owned());
    let _ = styles.insert("border-pink-600".to_owned(), "border-color: var(--color-pink-600);".to_owned());
    let _ = styles.insert("border-pink-700".to_owned(), "border-color: var(--color-pink-700);".to_owned());
    let _ = styles.insert("border-pink-800".to_owned(), "border-color: var(--color-pink-800);".to_owned());
    let _ = styles.insert("border-pink-900".to_owned(), "border-color: var(--color-pink-900);".to_owned());
    let _ = styles.insert("border-rose-50".to_owned(), "border-color: var(--color-rose-50);".to_owned());
    let _ = styles.insert("border-rose-100".to_owned(), "border-color: var(--color-rose-100);".to_owned());
    let _ = styles.insert("border-rose-200".to_owned(), "border-color: var(--color-rose-200);".to_owned());
    let _ = styles.insert("border-rose-300".to_owned(), "border-color: var(--color-rose-300);".to_owned());
    let _ = styles.insert("border-rose-400".to_owned(), "border-color: var(--color-rose-400);".to_owned());
    let _ = styles.insert("border-rose-500".to_owned(), "border-color: var(--color-rose-500);".to_owned());
    let _ = styles.insert("border-rose-600".to_owned(), "border-color: var(--color-rose-600);".to_owned());
    let _ = styles.insert("border-rose-700".to_owned(), "border-color: var(--color-rose-700);".to_owned());
    let _ = styles.insert("border-rose-800".to_owned(), "border-color: var(--color-rose-800);".to_owned());
    let _ = styles.insert("border-rose-900".to_owned(), "border-color: var(--color-rose-900);".to_owned());
    let _ = styles.insert("border-color-btn".to_owned(), "border-color: var(--color-btn-border);".to_owned());
    let _ = styles.insert("border-color--btn-hover".to_owned(), "border-color: var(--color-btn-hover-border);".to_owned());
    let _ = styles.insert("border-color--btn-active".to_owned(), "border-color: var(--color-btn-active-border);".to_owned());
    let _ = styles.insert("border-color-btn-focus".to_owned(), "border-color: var(--color-btn-focus-border);".to_owned());
    let _ = styles.insert("border-color-btn-filled".to_owned(), "border-color: var(--color-btn-filled-border);".to_owned());
    let _ = styles.insert("border-color-btn-filled-hover".to_owned(), "border-color: var(--color-btn-filled-hover-border);".to_owned());
    let _ = styles.insert("border-color-btn-filled-disabled".to_owned(), "border-color: var(--color-btn-filled-disabled-border);".to_owned());
    let _ = styles.insert("border-color-btn-filled-focus".to_owned(), "border-color: var(--color-btn-filled-focus-border);".to_owned());
    let _ = styles.insert("border-color-btn-outline-hover".to_owned(), "border-color: var(--color-btn-outline-hover-border);".to_owned());
    let _ = styles.insert("border-color-btn-outline-selected".to_owned(), "border-color: var(--color-btn-outline-selected-border);".to_owned());
    let _ = styles.insert("border-color-btn-outline-focus".to_owned(), "border-color: var(--color-btn-outline-focus-border);".to_owned());

    styles
}

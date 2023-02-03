use std::collections::HashMap;

pub fn divide_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("divide-inherit".to_owned(), "border-color: var(--inherit);".to_owned());
    let _ = styles.insert("divide-current".to_owned(), "border-color: var(--current);".to_owned());
    let _ = styles.insert("divide-transparent".to_owned(), "border-color: var(--transparent);".to_owned());
    let _ = styles.insert("divide-black".to_owned(), "border-color: var(--color-black);".to_owned());
    let _ = styles.insert("divide-white".to_owned(), "border-color: var(--color-white);".to_owned());
    let _ = styles.insert("divide-slate-50".to_owned(), "border-color: var(--color-slate-50);".to_owned());
    let _ = styles.insert("divide-slate-100".to_owned(), "border-color: var(--color-slate-100);".to_owned());
    let _ = styles.insert("divide-slate-200".to_owned(), "border-color: var(--color-slate-200);".to_owned());
    let _ = styles.insert("divide-slate-300".to_owned(), "border-color: var(--color-slate-300);".to_owned());
    let _ = styles.insert("divide-slate-400".to_owned(), "border-color: var(--color-slate-400);".to_owned());
    let _ = styles.insert("divide-slate-500".to_owned(), "border-color: var(--color-slate-500);".to_owned());
    let _ = styles.insert("divide-slate-600".to_owned(), "border-color: var(--color-slate-600);".to_owned());
    let _ = styles.insert("divide-slate-700".to_owned(), "border-color: var(--color-slate-700);".to_owned());
    let _ = styles.insert("divide-slate-800".to_owned(), "border-color: var(--color-slate-800);".to_owned());
    let _ = styles.insert("divide-slate-900".to_owned(), "border-color: var(--color-slate-900);".to_owned());
    let _ = styles.insert("divide-gray-50".to_owned(), "border-color: var(--color-gray-50);".to_owned());
    let _ = styles.insert("divide-gray-100".to_owned(), "border-color: var(--color-gray-100);".to_owned());
    let _ = styles.insert("divide-gray-200".to_owned(), "border-color: var(--color-gray-200);".to_owned());
    let _ = styles.insert("divide-gray-300".to_owned(), "border-color: var(--color-gray-300);".to_owned());
    let _ = styles.insert("divide-gray-400".to_owned(), "border-color: var(--color-gray-400);".to_owned());
    let _ = styles.insert("divide-gray-500".to_owned(), "border-color: var(--color-gray-500);".to_owned());
    let _ = styles.insert("divide-gray-600".to_owned(), "border-color: var(--color-gray-600);".to_owned());
    let _ = styles.insert("divide-gray-700".to_owned(), "border-color: var(--color-gray-700);".to_owned());
    let _ = styles.insert("divide-gray-800".to_owned(), "border-color: var(--color-gray-800);".to_owned());
    let _ = styles.insert("divide-gray-900".to_owned(), "border-color: var(--color-gray-900);".to_owned());
    let _ = styles.insert("divide-zinc-50".to_owned(), "border-color: var(--color-zinc-50);".to_owned());
    let _ = styles.insert("divide-zinc-100".to_owned(), "border-color: var(--color-zinc-100);".to_owned());
    let _ = styles.insert("divide-zinc-200".to_owned(), "border-color: var(--color-zinc-200);".to_owned());
    let _ = styles.insert("divide-zinc-300".to_owned(), "border-color: var(--color-zinc-300);".to_owned());
    let _ = styles.insert("divide-zinc-400".to_owned(), "border-color: var(--color-zinc-400);".to_owned());
    let _ = styles.insert("divide-zinc-500".to_owned(), "border-color: var(--color-zinc-500);".to_owned());
    let _ = styles.insert("divide-zinc-600".to_owned(), "border-color: var(--color-zinc-600);".to_owned());
    let _ = styles.insert("divide-zinc-700".to_owned(), "border-color: var(--color-zinc-700);".to_owned());
    let _ = styles.insert("divide-zinc-800".to_owned(), "border-color: var(--color-zinc-800);".to_owned());
    let _ = styles.insert("divide-zinc-900".to_owned(), "border-color: var(--color-zinc-900);".to_owned());
    let _ = styles.insert("divide-neutral-50".to_owned(), "border-color: var(--color-neutral-50);".to_owned());
    let _ = styles.insert("divide-neutral-100".to_owned(), "border-color: var(--color-neutral-100);".to_owned());
    let _ = styles.insert("divide-neutral-200".to_owned(), "border-color: var(--color-neutral-200);".to_owned());
    let _ = styles.insert("divide-neutral-300".to_owned(), "border-color: var(--color-neutral-300);".to_owned());
    let _ = styles.insert("divide-neutral-400".to_owned(), "border-color: var(--color-neutral-400);".to_owned());
    let _ = styles.insert("divide-neutral-500".to_owned(), "border-color: var(--color-neutral-500);".to_owned());
    let _ = styles.insert("divide-neutral-600".to_owned(), "border-color: var(--color-neutral-600);".to_owned());
    let _ = styles.insert("divide-neutral-700".to_owned(), "border-color: var(--color-neutral-700);".to_owned());
    let _ = styles.insert("divide-neutral-800".to_owned(), "border-color: var(--color-neutral-800);".to_owned());
    let _ = styles.insert("divide-neutral-900".to_owned(), "border-color: var(--color-neutral-900);".to_owned());
    let _ = styles.insert("divide-stone-50".to_owned(), "border-color: var(--color-stone-50);".to_owned());
    let _ = styles.insert("divide-stone-100".to_owned(), "border-color: var(--color-stone-100);".to_owned());
    let _ = styles.insert("divide-stone-200".to_owned(), "border-color: var(--color-stone-200);".to_owned());
    let _ = styles.insert("divide-stone-300".to_owned(), "border-color: var(--color-stone-300);".to_owned());
    let _ = styles.insert("divide-stone-400".to_owned(), "border-color: var(--color-stone-400);".to_owned());
    let _ = styles.insert("divide-stone-500".to_owned(), "border-color: var(--color-stone-500);".to_owned());
    let _ = styles.insert("divide-stone-600".to_owned(), "border-color: var(--color-stone-600);".to_owned());
    let _ = styles.insert("divide-stone-700".to_owned(), "border-color: var(--color-stone-700);".to_owned());
    let _ = styles.insert("divide-stone-800".to_owned(), "border-color: var(--color-stone-800);".to_owned());
    let _ = styles.insert("divide-stone-900".to_owned(), "border-color: var(--color-stone-900);".to_owned());
    let _ = styles.insert("divide-red-50".to_owned(), "border-color: var(--color-red-50);".to_owned());
    let _ = styles.insert("divide-red-100".to_owned(), "border-color: var(--color-red-100);".to_owned());
    let _ = styles.insert("divide-red-200".to_owned(), "border-color: var(--color-red-200);".to_owned());
    let _ = styles.insert("divide-red-300".to_owned(), "border-color: var(--color-red-300);".to_owned());
    let _ = styles.insert("divide-red-400".to_owned(), "border-color: var(--color-red-400);".to_owned());
    let _ = styles.insert("divide-red-500".to_owned(), "border-color: var(--color-red-500);".to_owned());
    let _ = styles.insert("divide-red-600".to_owned(), "border-color: var(--color-red-600);".to_owned());
    let _ = styles.insert("divide-red-700".to_owned(), "border-color: var(--color-red-700);".to_owned());
    let _ = styles.insert("divide-red-800".to_owned(), "border-color: var(--color-red-800);".to_owned());
    let _ = styles.insert("divide-red-900".to_owned(), "border-color: var(--color-red-900);".to_owned());
    let _ = styles.insert("divide-orange-50".to_owned(), "border-color: var(--color-orange-50);".to_owned());
    let _ = styles.insert("divide-orange-100".to_owned(), "border-color: var(--color-orange-100);".to_owned());
    let _ = styles.insert("divide-orange-200".to_owned(), "border-color: var(--color-orange-200);".to_owned());
    let _ = styles.insert("divide-orange-300".to_owned(), "border-color: var(--color-orange-300);".to_owned());
    let _ = styles.insert("divide-orange-400".to_owned(), "border-color: var(--color-orange-400);".to_owned());
    let _ = styles.insert("divide-orange-500".to_owned(), "border-color: var(--color-orange-500);".to_owned());
    let _ = styles.insert("divide-orange-600".to_owned(), "border-color: var(--color-orange-600);".to_owned());
    let _ = styles.insert("divide-orange-700".to_owned(), "border-color: var(--color-orange-700);".to_owned());
    let _ = styles.insert("divide-orange-800".to_owned(), "border-color: var(--color-orange-800);".to_owned());
    let _ = styles.insert("divide-orange-900".to_owned(), "border-color: var(--color-orange-900);".to_owned());
    let _ = styles.insert("divide-amber-50".to_owned(), "border-color: var(--color-amber-50);".to_owned());
    let _ = styles.insert("divide-amber-100".to_owned(), "border-color: var(--color-amber-100);".to_owned());
    let _ = styles.insert("divide-amber-200".to_owned(), "border-color: var(--color-amber-200);".to_owned());
    let _ = styles.insert("divide-amber-300".to_owned(), "border-color: var(--color-amber-300);".to_owned());
    let _ = styles.insert("divide-amber-400".to_owned(), "border-color: var(--color-amber-400);".to_owned());
    let _ = styles.insert("divide-amber-500".to_owned(), "border-color: var(--color-amber-500);".to_owned());
    let _ = styles.insert("divide-amber-600".to_owned(), "border-color: var(--color-amber-600);".to_owned());
    let _ = styles.insert("divide-amber-700".to_owned(), "border-color: var(--color-amber-700);".to_owned());
    let _ = styles.insert("divide-amber-800".to_owned(), "border-color: var(--color-amber-800);".to_owned());
    let _ = styles.insert("divide-amber-900".to_owned(), "border-color: var(--color-amber-900);".to_owned());
    let _ = styles.insert("divide-yellow-50".to_owned(), "border-color: var(--color-yellow-50);".to_owned());
    let _ = styles.insert("divide-yellow-100".to_owned(), "border-color: var(--color-yellow-100);".to_owned());
    let _ = styles.insert("divide-yellow-200".to_owned(), "border-color: var(--color-yellow-200);".to_owned());
    let _ = styles.insert("divide-yellow-300".to_owned(), "border-color: var(--color-yellow-300);".to_owned());
    let _ = styles.insert("divide-yellow-400".to_owned(), "border-color: var(--color-yellow-400);".to_owned());
    let _ = styles.insert("divide-yellow-500".to_owned(), "border-color: var(--color-yellow-500);".to_owned());
    let _ = styles.insert("divide-yellow-600".to_owned(), "border-color: var(--color-yellow-600);".to_owned());
    let _ = styles.insert("divide-yellow-700".to_owned(), "border-color: var(--color-yellow-700);".to_owned());
    let _ = styles.insert("divide-yellow-800".to_owned(), "border-color: var(--color-yellow-800);".to_owned());
    let _ = styles.insert("divide-yellow-900".to_owned(), "border-color: var(--color-yellow-900);".to_owned());
    let _ = styles.insert("divide-lime-50".to_owned(), "border-color: var(--color-lime-50);".to_owned());
    let _ = styles.insert("divide-lime-100".to_owned(), "border-color: var(--color-lime-100);".to_owned());
    let _ = styles.insert("divide-lime-200".to_owned(), "border-color: var(--color-lime-200);".to_owned());
    let _ = styles.insert("divide-lime-300".to_owned(), "border-color: var(--color-lime-300);".to_owned());
    let _ = styles.insert("divide-lime-400".to_owned(), "border-color: var(--color-lime-400);".to_owned());
    let _ = styles.insert("divide-lime-500".to_owned(), "border-color: var(--color-lime-500);".to_owned());
    let _ = styles.insert("divide-lime-600".to_owned(), "border-color: var(--color-lime-600);".to_owned());
    let _ = styles.insert("divide-lime-700".to_owned(), "border-color: var(--color-lime-700);".to_owned());
    let _ = styles.insert("divide-lime-800".to_owned(), "border-color: var(--color-lime-800);".to_owned());
    let _ = styles.insert("divide-lime-900".to_owned(), "border-color: var(--color-lime-900);".to_owned());
    let _ = styles.insert("divide-green-50".to_owned(), "border-color: var(--color-green-50);".to_owned());
    let _ = styles.insert("divide-green-100".to_owned(), "border-color: var(--color-green-100);".to_owned());
    let _ = styles.insert("divide-green-200".to_owned(), "border-color: var(--color-green-200);".to_owned());
    let _ = styles.insert("divide-green-300".to_owned(), "border-color: var(--color-green-300);".to_owned());
    let _ = styles.insert("divide-green-400".to_owned(), "border-color: var(--color-green-400);".to_owned());
    let _ = styles.insert("divide-green-500".to_owned(), "border-color: var(--color-green-500);".to_owned());
    let _ = styles.insert("divide-green-600".to_owned(), "border-color: var(--color-green-600);".to_owned());
    let _ = styles.insert("divide-green-700".to_owned(), "border-color: var(--color-green-700);".to_owned());
    let _ = styles.insert("divide-green-800".to_owned(), "border-color: var(--color-green-800);".to_owned());
    let _ = styles.insert("divide-green-900".to_owned(), "border-color: var(--color-green-900);".to_owned());
    let _ = styles.insert("divide-emerald-50".to_owned(), "border-color: var(--color-emerald-50);".to_owned());
    let _ = styles.insert("divide-emerald-100".to_owned(), "border-color: var(--color-emerald-100);".to_owned());
    let _ = styles.insert("divide-emerald-200".to_owned(), "border-color: var(--color-emerald-200);".to_owned());
    let _ = styles.insert("divide-emerald-300".to_owned(), "border-color: var(--color-emerald-300);".to_owned());
    let _ = styles.insert("divide-emerald-400".to_owned(), "border-color: var(--color-emerald-400);".to_owned());
    let _ = styles.insert("divide-emerald-500".to_owned(), "border-color: var(--color-emerald-500);".to_owned());
    let _ = styles.insert("divide-emerald-600".to_owned(), "border-color: var(--color-emerald-600);".to_owned());
    let _ = styles.insert("divide-emerald-700".to_owned(), "border-color: var(--color-emerald-700);".to_owned());
    let _ = styles.insert("divide-emerald-800".to_owned(), "border-color: var(--color-emerald-800);".to_owned());
    let _ = styles.insert("divide-emerald-900".to_owned(), "border-color: var(--color-emerald-900);".to_owned());
    let _ = styles.insert("divide-teal-50".to_owned(), "border-color: var(--color-teal-50);".to_owned());
    let _ = styles.insert("divide-teal-100".to_owned(), "border-color: var(--color-teal-100);".to_owned());
    let _ = styles.insert("divide-teal-200".to_owned(), "border-color: var(--color-teal-200);".to_owned());
    let _ = styles.insert("divide-teal-300".to_owned(), "border-color: var(--color-teal-300);".to_owned());
    let _ = styles.insert("divide-teal-400".to_owned(), "border-color: var(--color-teal-400);".to_owned());
    let _ = styles.insert("divide-teal-500".to_owned(), "border-color: var(--color-teal-500);".to_owned());
    let _ = styles.insert("divide-teal-600".to_owned(), "border-color: var(--color-teal-600);".to_owned());
    let _ = styles.insert("divide-teal-700".to_owned(), "border-color: var(--color-teal-700);".to_owned());
    let _ = styles.insert("divide-teal-800".to_owned(), "border-color: var(--color-teal-800);".to_owned());
    let _ = styles.insert("divide-teal-900".to_owned(), "border-color: var(--color-teal-900);".to_owned());
    let _ = styles.insert("divide-cyan-50".to_owned(), "border-color: var(--color-cyan-50);".to_owned());
    let _ = styles.insert("divide-cyan-100".to_owned(), "border-color: var(--color-cyan-100);".to_owned());
    let _ = styles.insert("divide-cyan-200".to_owned(), "border-color: var(--color-cyan-200);".to_owned());
    let _ = styles.insert("divide-cyan-300".to_owned(), "border-color: var(--color-cyan-300);".to_owned());
    let _ = styles.insert("divide-cyan-400".to_owned(), "border-color: var(--color-cyan-400);".to_owned());
    let _ = styles.insert("divide-cyan-500".to_owned(), "border-color: var(--color-cyan-500);".to_owned());
    let _ = styles.insert("divide-cyan-600".to_owned(), "border-color: var(--color-cyan-600);".to_owned());
    let _ = styles.insert("divide-cyan-700".to_owned(), "border-color: var(--color-cyan-700);".to_owned());
    let _ = styles.insert("divide-cyan-800".to_owned(), "border-color: var(--color-cyan-800);".to_owned());
    let _ = styles.insert("divide-cyan-900".to_owned(), "border-color: var(--color-cyan-900);".to_owned());
    let _ = styles.insert("divide-sky-50".to_owned(), "border-color: var(--color-sky-50);".to_owned());
    let _ = styles.insert("divide-sky-100".to_owned(), "border-color: var(--color-sky-100);".to_owned());
    let _ = styles.insert("divide-sky-200".to_owned(), "border-color: var(--color-sky-200);".to_owned());
    let _ = styles.insert("divide-sky-300".to_owned(), "border-color: var(--color-sky-300);".to_owned());
    let _ = styles.insert("divide-sky-400".to_owned(), "border-color: var(--color-sky-400);".to_owned());
    let _ = styles.insert("divide-sky-500".to_owned(), "border-color: var(--color-sky-500);".to_owned());
    let _ = styles.insert("divide-sky-600".to_owned(), "border-color: var(--color-sky-600);".to_owned());
    let _ = styles.insert("divide-sky-700".to_owned(), "border-color: var(--color-sky-700);".to_owned());
    let _ = styles.insert("divide-sky-800".to_owned(), "border-color: var(--color-sky-800);".to_owned());
    let _ = styles.insert("divide-sky-900".to_owned(), "border-color: var(--color-sky-900);".to_owned());
    let _ = styles.insert("divide-blue-50".to_owned(), "border-color: var(--color-blue-50);".to_owned());
    let _ = styles.insert("divide-blue-100".to_owned(), "border-color: var(--color-blue-100);".to_owned());
    let _ = styles.insert("divide-blue-200".to_owned(), "border-color: var(--color-blue-200);".to_owned());
    let _ = styles.insert("divide-blue-300".to_owned(), "border-color: var(--color-blue-300);".to_owned());
    let _ = styles.insert("divide-blue-400".to_owned(), "border-color: var(--color-blue-400);".to_owned());
    let _ = styles.insert("divide-blue-500".to_owned(), "border-color: var(--color-blue-500);".to_owned());
    let _ = styles.insert("divide-blue-600".to_owned(), "border-color: var(--color-blue-600);".to_owned());
    let _ = styles.insert("divide-blue-700".to_owned(), "border-color: var(--color-blue-700);".to_owned());
    let _ = styles.insert("divide-blue-800".to_owned(), "border-color: var(--color-blue-800);".to_owned());
    let _ = styles.insert("divide-blue-900".to_owned(), "border-color: var(--color-blue-900);".to_owned());
    let _ = styles.insert("divide-indigo-50".to_owned(), "border-color: var(--color-indigo-50);".to_owned());
    let _ = styles.insert("divide-indigo-100".to_owned(), "border-color: var(--color-indigo-100);".to_owned());
    let _ = styles.insert("divide-indigo-200".to_owned(), "border-color: var(--color-indigo-200);".to_owned());
    let _ = styles.insert("divide-indigo-300".to_owned(), "border-color: var(--color-indigo-300);".to_owned());
    let _ = styles.insert("divide-indigo-400".to_owned(), "border-color: var(--color-indigo-400);".to_owned());
    let _ = styles.insert("divide-indigo-500".to_owned(), "border-color: var(--color-indigo-500);".to_owned());
    let _ = styles.insert("divide-indigo-600".to_owned(), "border-color: var(--color-indigo-600);".to_owned());
    let _ = styles.insert("divide-indigo-700".to_owned(), "border-color: var(--color-indigo-700);".to_owned());
    let _ = styles.insert("divide-indigo-800".to_owned(), "border-color: var(--color-indigo-800);".to_owned());
    let _ = styles.insert("divide-indigo-900".to_owned(), "border-color: var(--color-indigo-900);".to_owned());
    let _ = styles.insert("divide-violet-50".to_owned(), "border-color: var(--color-violet-50);".to_owned());
    let _ = styles.insert("divide-violet-100".to_owned(), "border-color: var(--color-violet-100);".to_owned());
    let _ = styles.insert("divide-violet-200".to_owned(), "border-color: var(--color-violet-200);".to_owned());
    let _ = styles.insert("divide-violet-300".to_owned(), "border-color: var(--color-violet-300);".to_owned());
    let _ = styles.insert("divide-violet-400".to_owned(), "border-color: var(--color-violet-400);".to_owned());
    let _ = styles.insert("divide-violet-500".to_owned(), "border-color: var(--color-violet-500);".to_owned());
    let _ = styles.insert("divide-violet-600".to_owned(), "border-color: var(--color-violet-600);".to_owned());
    let _ = styles.insert("divide-violet-700".to_owned(), "border-color: var(--color-violet-700);".to_owned());
    let _ = styles.insert("divide-violet-800".to_owned(), "border-color: var(--color-violet-800);".to_owned());
    let _ = styles.insert("divide-violet-900".to_owned(), "border-color: var(--color-violet-900);".to_owned());
    let _ = styles.insert("divide-purple-50".to_owned(), "border-color: var(--color-purple-50);".to_owned());
    let _ = styles.insert("divide-purple-100".to_owned(), "border-color: var(--color-purple-100);".to_owned());
    let _ = styles.insert("divide-purple-200".to_owned(), "border-color: var(--color-purple-200);".to_owned());
    let _ = styles.insert("divide-purple-300".to_owned(), "border-color: var(--color-purple-300);".to_owned());
    let _ = styles.insert("divide-purple-400".to_owned(), "border-color: var(--color-purple-400);".to_owned());
    let _ = styles.insert("divide-purple-500".to_owned(), "border-color: var(--color-purple-500);".to_owned());
    let _ = styles.insert("divide-purple-600".to_owned(), "border-color: var(--color-purple-600);".to_owned());
    let _ = styles.insert("divide-purple-700".to_owned(), "border-color: var(--color-purple-700);".to_owned());
    let _ = styles.insert("divide-purple-800".to_owned(), "border-color: var(--color-purple-800);".to_owned());
    let _ = styles.insert("divide-purple-900".to_owned(), "border-color: var(--color-purple-900);".to_owned());
    let _ = styles.insert("divide-fuchsia-50".to_owned(), "border-color: var(--color-fuchsia-50);".to_owned());
    let _ = styles.insert("divide-fuchsia-100".to_owned(), "border-color: var(--color-fuchsia-100);".to_owned());
    let _ = styles.insert("divide-fuchsia-200".to_owned(), "border-color: var(--color-fuchsia-200);".to_owned());
    let _ = styles.insert("divide-fuchsia-300".to_owned(), "border-color: var(--color-fuchsia-300);".to_owned());
    let _ = styles.insert("divide-fuchsia-400".to_owned(), "border-color: var(--color-fuchsia-400);".to_owned());
    let _ = styles.insert("divide-fuchsia-500".to_owned(), "border-color: var(--color-fuchsia-500);".to_owned());
    let _ = styles.insert("divide-fuchsia-600".to_owned(), "border-color: var(--color-fuchsia-600);".to_owned());
    let _ = styles.insert("divide-fuchsia-700".to_owned(), "border-color: var(--color-fuchsia-700);".to_owned());
    let _ = styles.insert("divide-fuchsia-800".to_owned(), "border-color: var(--color-fuchsia-800);".to_owned());
    let _ = styles.insert("divide-fuchsia-900".to_owned(), "border-color: var(--color-fuchsia-900);".to_owned());
    let _ = styles.insert("divide-pink-50".to_owned(), "border-color: var(--color-pink-50);".to_owned());
    let _ = styles.insert("divide-pink-100".to_owned(), "border-color: var(--color-pink-100);".to_owned());
    let _ = styles.insert("divide-pink-200".to_owned(), "border-color: var(--color-pink-200);".to_owned());
    let _ = styles.insert("divide-pink-300".to_owned(), "border-color: var(--color-pink-300);".to_owned());
    let _ = styles.insert("divide-pink-400".to_owned(), "border-color: var(--color-pink-400);".to_owned());
    let _ = styles.insert("divide-pink-500".to_owned(), "border-color: var(--color-pink-500);".to_owned());
    let _ = styles.insert("divide-pink-600".to_owned(), "border-color: var(--color-pink-600);".to_owned());
    let _ = styles.insert("divide-pink-700".to_owned(), "border-color: var(--color-pink-700);".to_owned());
    let _ = styles.insert("divide-pink-800".to_owned(), "border-color: var(--color-pink-800);".to_owned());
    let _ = styles.insert("divide-pink-900".to_owned(), "border-color: var(--color-pink-900);".to_owned());
    let _ = styles.insert("divide-rose-50".to_owned(), "border-color: var(--color-rose-50);".to_owned());
    let _ = styles.insert("divide-rose-100".to_owned(), "border-color: var(--color-rose-100);".to_owned());
    let _ = styles.insert("divide-rose-200".to_owned(), "border-color: var(--color-rose-200);".to_owned());
    let _ = styles.insert("divide-rose-300".to_owned(), "border-color: var(--color-rose-300);".to_owned());
    let _ = styles.insert("divide-rose-400".to_owned(), "border-color: var(--color-rose-400);".to_owned());
    let _ = styles.insert("divide-rose-500".to_owned(), "border-color: var(--color-rose-500);".to_owned());
    let _ = styles.insert("divide-rose-600".to_owned(), "border-color: var(--color-rose-600);".to_owned());
    let _ = styles.insert("divide-rose-700".to_owned(), "border-color: var(--color-rose-700);".to_owned());
    let _ = styles.insert("divide-rose-800".to_owned(), "border-color: var(--color-rose-800);".to_owned());
    let _ = styles.insert("divide-rose-900".to_owned(), "border-color: var(--color-rose-900);".to_owned());
    
    styles
}
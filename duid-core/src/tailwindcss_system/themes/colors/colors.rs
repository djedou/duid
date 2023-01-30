use std::collections::HashMap;


pub fn colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("color-inherit".to_owned(), "color: var(--inherit);".to_owned());
    let _ = styles.insert("color-current".to_owned(), "color: var(--current);".to_owned());
    let _ = styles.insert("color-transparent".to_owned(), "color: var(--transparent);".to_owned());
    let _ = styles.insert("color-black".to_owned(), "color: var(--color-black);".to_owned());
    let _ = styles.insert("color-white".to_owned(), "color: var(--color-white);".to_owned());
    let _ = styles.insert("color-slate-50".to_owned(), "color: var(--color-slate-50);".to_owned());
    let _ = styles.insert("color-slate-100".to_owned(), "color: var(--color-slate-100);".to_owned());
    let _ = styles.insert("color-slate-200".to_owned(), "color: var(--color-slate-200);".to_owned());
    let _ = styles.insert("color-slate-300".to_owned(), "color: var(--color-slate-300);".to_owned());
    let _ = styles.insert("color-slate-400".to_owned(), "color: var(--color-slate-400);".to_owned());
    let _ = styles.insert("color-slate-500".to_owned(), "color: var(--color-slate-500);".to_owned());
    let _ = styles.insert("color-slate-600".to_owned(), "color: var(--color-slate-600);".to_owned());
    let _ = styles.insert("color-slate-700".to_owned(), "color: var(--color-slate-700);".to_owned());
    let _ = styles.insert("color-slate-800".to_owned(), "color: var(--color-slate-800);".to_owned());
    let _ = styles.insert("color-slate-900".to_owned(), "color: var(--color-slate-900);".to_owned());
    let _ = styles.insert("color-gray-50".to_owned(), "color: var(--color-gray-50);".to_owned());
    let _ = styles.insert("color-gray-100".to_owned(), "color: var(--color-gray-100);".to_owned());
    let _ = styles.insert("color-gray-200".to_owned(), "color: var(--color-gray-200);".to_owned());
    let _ = styles.insert("color-gray-300".to_owned(), "color: var(--color-gray-300);".to_owned());
    let _ = styles.insert("color-gray-400".to_owned(), "color: var(--color-gray-400);".to_owned());
    let _ = styles.insert("color-gray-500".to_owned(), "color: var(--color-gray-500);".to_owned());
    let _ = styles.insert("color-gray-600".to_owned(), "color: var(--color-gray-600);".to_owned());
    let _ = styles.insert("color-gray-700".to_owned(), "color: var(--color-gray-700);".to_owned());
    let _ = styles.insert("color-gray-800".to_owned(), "color: var(--color-gray-800);".to_owned());
    let _ = styles.insert("color-gray-900".to_owned(), "color: var(--color-gray-900);".to_owned());
    let _ = styles.insert("color-zinc-50".to_owned(), "color: var(--color-zinc-50);".to_owned());
    let _ = styles.insert("color-zinc-100".to_owned(), "color: var(--color-zinc-100);".to_owned());
    let _ = styles.insert("color-zinc-200".to_owned(), "color: var(--color-zinc-200);".to_owned());
    let _ = styles.insert("color-zinc-300".to_owned(), "color: var(--color-zinc-300);".to_owned());
    let _ = styles.insert("color-zinc-400".to_owned(), "color: var(--color-zinc-400);".to_owned());
    let _ = styles.insert("color-zinc-500".to_owned(), "color: var(--color-zinc-500);".to_owned());
    let _ = styles.insert("color-zinc-600".to_owned(), "color: var(--color-zinc-600);".to_owned());
    let _ = styles.insert("color-zinc-700".to_owned(), "color: var(--color-zinc-700);".to_owned());
    let _ = styles.insert("color-zinc-800".to_owned(), "color: var(--color-zinc-800);".to_owned());
    let _ = styles.insert("color-zinc-900".to_owned(), "color: var(--color-zinc-900);".to_owned());
    let _ = styles.insert("color-neutral-50".to_owned(), "color: var(--color-neutral-50);".to_owned());
    let _ = styles.insert("color-neutral-100".to_owned(), "color: var(--color-neutral-100);".to_owned());
    let _ = styles.insert("color-neutral-200".to_owned(), "color: var(--color-neutral-200);".to_owned());
    let _ = styles.insert("color-neutral-300".to_owned(), "color: var(--color-neutral-300);".to_owned());
    let _ = styles.insert("color-neutral-400".to_owned(), "color: var(--color-neutral-400);".to_owned());
    let _ = styles.insert("color-neutral-500".to_owned(), "color: var(--color-neutral-500);".to_owned());
    let _ = styles.insert("color-neutral-600".to_owned(), "color: var(--color-neutral-600);".to_owned());
    let _ = styles.insert("color-neutral-700".to_owned(), "color: var(--color-neutral-700);".to_owned());
    let _ = styles.insert("color-neutral-800".to_owned(), "color: var(--color-neutral-800);".to_owned());
    let _ = styles.insert("color-neutral-900".to_owned(), "color: var(--color-neutral-900);".to_owned());
    let _ = styles.insert("color-stone-50".to_owned(), "color: var(--color-stone-50);".to_owned());
    let _ = styles.insert("color-stone-100".to_owned(), "color: var(--color-stone-100);".to_owned());
    let _ = styles.insert("color-stone-200".to_owned(), "color: var(--color-stone-200);".to_owned());
    let _ = styles.insert("color-stone-300".to_owned(), "color: var(--color-stone-300);".to_owned());
    let _ = styles.insert("color-stone-400".to_owned(), "color: var(--color-stone-400);".to_owned());
    let _ = styles.insert("color-stone-500".to_owned(), "color: var(--color-stone-500);".to_owned());
    let _ = styles.insert("color-stone-600".to_owned(), "color: var(--color-stone-600);".to_owned());
    let _ = styles.insert("color-stone-700".to_owned(), "color: var(--color-stone-700);".to_owned());
    let _ = styles.insert("color-stone-800".to_owned(), "color: var(--color-stone-800);".to_owned());
    let _ = styles.insert("color-stone-900".to_owned(), "color: var(--color-stone-900);".to_owned());
    let _ = styles.insert("color-red-50".to_owned(), "color: var(--color-red-50);".to_owned());
    let _ = styles.insert("color-red-100".to_owned(), "color: var(--color-red-100);".to_owned());
    let _ = styles.insert("color-red-200".to_owned(), "color: var(--color-red-200);".to_owned());
    let _ = styles.insert("color-red-300".to_owned(), "color: var(--color-red-300);".to_owned());
    let _ = styles.insert("color-red-400".to_owned(), "color: var(--color-red-400);".to_owned());
    let _ = styles.insert("color-red-500".to_owned(), "color: var(--color-red-500);".to_owned());
    let _ = styles.insert("color-red-600".to_owned(), "color: var(--color-red-600);".to_owned());
    let _ = styles.insert("color-red-700".to_owned(), "color: var(--color-red-700);".to_owned());
    let _ = styles.insert("color-red-800".to_owned(), "color: var(--color-red-800);".to_owned());
    let _ = styles.insert("color-red-900".to_owned(), "color: var(--color-red-900);".to_owned());
    let _ = styles.insert("color-orange-50".to_owned(), "color: var(--color-orange-50);".to_owned());
    let _ = styles.insert("color-orange-100".to_owned(), "color: var(--color-orange-100);".to_owned());
    let _ = styles.insert("color-orange-200".to_owned(), "color: var(--color-orange-200);".to_owned());
    let _ = styles.insert("color-orange-300".to_owned(), "color: var(--color-orange-300);".to_owned());
    let _ = styles.insert("color-orange-400".to_owned(), "color: var(--color-orange-400);".to_owned());
    let _ = styles.insert("color-orange-500".to_owned(), "color: var(--color-orange-500);".to_owned());
    let _ = styles.insert("color-orange-600".to_owned(), "color: var(--color-orange-600);".to_owned());
    let _ = styles.insert("color-orange-700".to_owned(), "color: var(--color-orange-700);".to_owned());
    let _ = styles.insert("color-orange-800".to_owned(), "color: var(--color-orange-800);".to_owned());
    let _ = styles.insert("color-orange-900".to_owned(), "color: var(--color-orange-900);".to_owned());
    let _ = styles.insert("color-amber-50".to_owned(), "color: var(--color-amber-50);".to_owned());
    let _ = styles.insert("color-amber-100".to_owned(), "color: var(--color-amber-100);".to_owned());
    let _ = styles.insert("color-amber-200".to_owned(), "color: var(--color-amber-200);".to_owned());
    let _ = styles.insert("color-amber-300".to_owned(), "color: var(--color-amber-300);".to_owned());
    let _ = styles.insert("color-amber-400".to_owned(), "color: var(--color-amber-400);".to_owned());
    let _ = styles.insert("color-amber-500".to_owned(), "color: var(--color-amber-500);".to_owned());
    let _ = styles.insert("color-amber-600".to_owned(), "color: var(--color-amber-600);".to_owned());
    let _ = styles.insert("color-amber-700".to_owned(), "color: var(--color-amber-700);".to_owned());
    let _ = styles.insert("color-amber-800".to_owned(), "color: var(--color-amber-800);".to_owned());
    let _ = styles.insert("color-amber-900".to_owned(), "color: var(--color-amber-900);".to_owned());
    let _ = styles.insert("color-yellow-50".to_owned(), "color: var(--color-yellow-50);".to_owned());
    let _ = styles.insert("color-yellow-100".to_owned(), "color: var(--color-yellow-100);".to_owned());
    let _ = styles.insert("color-yellow-200".to_owned(), "color: var(--color-yellow-200);".to_owned());
    let _ = styles.insert("color-yellow-300".to_owned(), "color: var(--color-yellow-300);".to_owned());
    let _ = styles.insert("color-yellow-400".to_owned(), "color: var(--color-yellow-400);".to_owned());
    let _ = styles.insert("color-yellow-500".to_owned(), "color: var(--color-yellow-500);".to_owned());
    let _ = styles.insert("color-yellow-600".to_owned(), "color: var(--color-yellow-600);".to_owned());
    let _ = styles.insert("color-yellow-700".to_owned(), "color: var(--color-yellow-700);".to_owned());
    let _ = styles.insert("color-yellow-800".to_owned(), "color: var(--color-yellow-800);".to_owned());
    let _ = styles.insert("color-yellow-900".to_owned(), "color: var(--color-yellow-900);".to_owned());
    let _ = styles.insert("color-lime-50".to_owned(), "color: var(--color-lime-50);".to_owned());
    let _ = styles.insert("color-lime-100".to_owned(), "color: var(--color-lime-100);".to_owned());
    let _ = styles.insert("color-lime-200".to_owned(), "color: var(--color-lime-200);".to_owned());
    let _ = styles.insert("color-lime-300".to_owned(), "color: var(--color-lime-300);".to_owned());
    let _ = styles.insert("color-lime-400".to_owned(), "color: var(--color-lime-400);".to_owned());
    let _ = styles.insert("color-lime-500".to_owned(), "color: var(--color-lime-500);".to_owned());
    let _ = styles.insert("color-lime-600".to_owned(), "color: var(--color-lime-600);".to_owned());
    let _ = styles.insert("color-lime-700".to_owned(), "color: var(--color-lime-700);".to_owned());
    let _ = styles.insert("color-lime-800".to_owned(), "color: var(--color-lime-800);".to_owned());
    let _ = styles.insert("color-lime-900".to_owned(), "color: var(--color-lime-900);".to_owned());
    let _ = styles.insert("color-green-50".to_owned(), "color: var(--color-green-50);".to_owned());
    let _ = styles.insert("color-green-100".to_owned(), "color: var(--color-green-100);".to_owned());
    let _ = styles.insert("color-green-200".to_owned(), "color: var(--color-green-200);".to_owned());
    let _ = styles.insert("color-green-300".to_owned(), "color: var(--color-green-300);".to_owned());
    let _ = styles.insert("color-green-400".to_owned(), "color: var(--color-green-400);".to_owned());
    let _ = styles.insert("color-green-500".to_owned(), "color: var(--color-green-500);".to_owned());
    let _ = styles.insert("color-green-600".to_owned(), "color: var(--color-green-600);".to_owned());
    let _ = styles.insert("color-green-700".to_owned(), "color: var(--color-green-700);".to_owned());
    let _ = styles.insert("color-green-800".to_owned(), "color: var(--color-green-800);".to_owned());
    let _ = styles.insert("color-green-900".to_owned(), "color: var(--color-green-900);".to_owned());
    let _ = styles.insert("color-emerald-50".to_owned(), "color: var(--color-emerald-50);".to_owned());
    let _ = styles.insert("color-emerald-100".to_owned(), "color: var(--color-emerald-100);".to_owned());
    let _ = styles.insert("color-emerald-200".to_owned(), "color: var(--color-emerald-200);".to_owned());
    let _ = styles.insert("color-emerald-300".to_owned(), "color: var(--color-emerald-300);".to_owned());
    let _ = styles.insert("color-emerald-400".to_owned(), "color: var(--color-emerald-400);".to_owned());
    let _ = styles.insert("color-emerald-500".to_owned(), "color: var(--color-emerald-500);".to_owned());
    let _ = styles.insert("color-emerald-600".to_owned(), "color: var(--color-emerald-600);".to_owned());
    let _ = styles.insert("color-emerald-700".to_owned(), "color: var(--color-emerald-700);".to_owned());
    let _ = styles.insert("color-emerald-800".to_owned(), "color: var(--color-emerald-800);".to_owned());
    let _ = styles.insert("color-emerald-900".to_owned(), "color: var(--color-emerald-900);".to_owned());
    let _ = styles.insert("color-teal-50".to_owned(), "color: var(--color-teal-50);".to_owned());
    let _ = styles.insert("color-teal-100".to_owned(), "color: var(--color-teal-100);".to_owned());
    let _ = styles.insert("color-teal-200".to_owned(), "color: var(--color-teal-200);".to_owned());
    let _ = styles.insert("color-teal-300".to_owned(), "color: var(--color-teal-300);".to_owned());
    let _ = styles.insert("color-teal-400".to_owned(), "color: var(--color-teal-400);".to_owned());
    let _ = styles.insert("color-teal-500".to_owned(), "color: var(--color-teal-500);".to_owned());
    let _ = styles.insert("color-teal-600".to_owned(), "color: var(--color-teal-600);".to_owned());
    let _ = styles.insert("color-teal-700".to_owned(), "color: var(--color-teal-700);".to_owned());
    let _ = styles.insert("color-teal-800".to_owned(), "color: var(--color-teal-800);".to_owned());
    let _ = styles.insert("color-teal-900".to_owned(), "color: var(--color-teal-900);".to_owned());
    let _ = styles.insert("color-cyan-50".to_owned(), "color: var(--color-cyan-50);".to_owned());
    let _ = styles.insert("color-cyan-100".to_owned(), "color: var(--color-cyan-100);".to_owned());
    let _ = styles.insert("color-cyan-200".to_owned(), "color: var(--color-cyan-200);".to_owned());
    let _ = styles.insert("color-cyan-300".to_owned(), "color: var(--color-cyan-300);".to_owned());
    let _ = styles.insert("color-cyan-400".to_owned(), "color: var(--color-cyan-400);".to_owned());
    let _ = styles.insert("color-cyan-500".to_owned(), "color: var(--color-cyan-500);".to_owned());
    let _ = styles.insert("color-cyan-600".to_owned(), "color: var(--color-cyan-600);".to_owned());
    let _ = styles.insert("color-cyan-700".to_owned(), "color: var(--color-cyan-700);".to_owned());
    let _ = styles.insert("color-cyan-800".to_owned(), "color: var(--color-cyan-800);".to_owned());
    let _ = styles.insert("color-cyan-900".to_owned(), "color: var(--color-cyan-900);".to_owned());
    let _ = styles.insert("color-sky-50".to_owned(), "color: var(--color-sky-50);".to_owned());
    let _ = styles.insert("color-sky-100".to_owned(), "color: var(--color-sky-100);".to_owned());
    let _ = styles.insert("color-sky-200".to_owned(), "color: var(--color-sky-200);".to_owned());
    let _ = styles.insert("color-sky-300".to_owned(), "color: var(--color-sky-300);".to_owned());
    let _ = styles.insert("color-sky-400".to_owned(), "color: var(--color-sky-400);".to_owned());
    let _ = styles.insert("color-sky-500".to_owned(), "color: var(--color-sky-500);".to_owned());
    let _ = styles.insert("color-sky-600".to_owned(), "color: var(--color-sky-600);".to_owned());
    let _ = styles.insert("color-sky-700".to_owned(), "color: var(--color-sky-700);".to_owned());
    let _ = styles.insert("color-sky-800".to_owned(), "color: var(--color-sky-800);".to_owned());
    let _ = styles.insert("color-sky-900".to_owned(), "color: var(--color-sky-900);".to_owned());
    let _ = styles.insert("color-blue-50".to_owned(), "color: var(--color-blue-50);".to_owned());
    let _ = styles.insert("color-blue-100".to_owned(), "color: var(--color-blue-100);".to_owned());
    let _ = styles.insert("color-blue-200".to_owned(), "color: var(--color-blue-200);".to_owned());
    let _ = styles.insert("color-blue-300".to_owned(), "color: var(--color-blue-300);".to_owned());
    let _ = styles.insert("color-blue-400".to_owned(), "color: var(--color-blue-400);".to_owned());
    let _ = styles.insert("color-blue-500".to_owned(), "color: var(--color-blue-500);".to_owned());
    let _ = styles.insert("color-blue-600".to_owned(), "color: var(--color-blue-600);".to_owned());
    let _ = styles.insert("color-blue-700".to_owned(), "color: var(--color-blue-700);".to_owned());
    let _ = styles.insert("color-blue-800".to_owned(), "color: var(--color-blue-800);".to_owned());
    let _ = styles.insert("color-blue-900".to_owned(), "color: var(--color-blue-900);".to_owned());
    let _ = styles.insert("color-indigo-50".to_owned(), "color: var(--color-indigo-50);".to_owned());
    let _ = styles.insert("color-indigo-100".to_owned(), "color: var(--color-indigo-100);".to_owned());
    let _ = styles.insert("color-indigo-200".to_owned(), "color: var(--color-indigo-200);".to_owned());
    let _ = styles.insert("color-indigo-300".to_owned(), "color: var(--color-indigo-300);".to_owned());
    let _ = styles.insert("color-indigo-400".to_owned(), "color: var(--color-indigo-400);".to_owned());
    let _ = styles.insert("color-indigo-500".to_owned(), "color: var(--color-indigo-500);".to_owned());
    let _ = styles.insert("color-indigo-600".to_owned(), "color: var(--color-indigo-600);".to_owned());
    let _ = styles.insert("color-indigo-700".to_owned(), "color: var(--color-indigo-700);".to_owned());
    let _ = styles.insert("color-indigo-800".to_owned(), "color: var(--color-indigo-800);".to_owned());
    let _ = styles.insert("color-indigo-900".to_owned(), "color: var(--color-indigo-900);".to_owned());
    let _ = styles.insert("color-violet-50".to_owned(), "color: var(--color-violet-50);".to_owned());
    let _ = styles.insert("color-violet-100".to_owned(), "color: var(--color-violet-100);".to_owned());
    let _ = styles.insert("color-violet-200".to_owned(), "color: var(--color-violet-200);".to_owned());
    let _ = styles.insert("color-violet-300".to_owned(), "color: var(--color-violet-300);".to_owned());
    let _ = styles.insert("color-violet-400".to_owned(), "color: var(--color-violet-400);".to_owned());
    let _ = styles.insert("color-violet-500".to_owned(), "color: var(--color-violet-500);".to_owned());
    let _ = styles.insert("color-violet-600".to_owned(), "color: var(--color-violet-600);".to_owned());
    let _ = styles.insert("color-violet-700".to_owned(), "color: var(--color-violet-700);".to_owned());
    let _ = styles.insert("color-violet-800".to_owned(), "color: var(--color-violet-800);".to_owned());
    let _ = styles.insert("color-violet-900".to_owned(), "color: var(--color-violet-900);".to_owned());
    let _ = styles.insert("color-purple-50".to_owned(), "color: var(--color-purple-50);".to_owned());
    let _ = styles.insert("color-purple-100".to_owned(), "color: var(--color-purple-100);".to_owned());
    let _ = styles.insert("color-purple-200".to_owned(), "color: var(--color-purple-200);".to_owned());
    let _ = styles.insert("color-purple-300".to_owned(), "color: var(--color-purple-300);".to_owned());
    let _ = styles.insert("color-purple-400".to_owned(), "color: var(--color-purple-400);".to_owned());
    let _ = styles.insert("color-purple-500".to_owned(), "color: var(--color-purple-500);".to_owned());
    let _ = styles.insert("color-purple-600".to_owned(), "color: var(--color-purple-600);".to_owned());
    let _ = styles.insert("color-purple-700".to_owned(), "color: var(--color-purple-700);".to_owned());
    let _ = styles.insert("color-purple-800".to_owned(), "color: var(--color-purple-800);".to_owned());
    let _ = styles.insert("color-purple-900".to_owned(), "color: var(--color-purple-900);".to_owned());
    let _ = styles.insert("color-fuchsia-50".to_owned(), "color: var(--color-fuchsia-50);".to_owned());
    let _ = styles.insert("color-fuchsia-100".to_owned(), "color: var(--color-fuchsia-100);".to_owned());
    let _ = styles.insert("color-fuchsia-200".to_owned(), "color: var(--color-fuchsia-200);".to_owned());
    let _ = styles.insert("color-fuchsia-300".to_owned(), "color: var(--color-fuchsia-300);".to_owned());
    let _ = styles.insert("color-fuchsia-400".to_owned(), "color: var(--color-fuchsia-400);".to_owned());
    let _ = styles.insert("color-fuchsia-500".to_owned(), "color: var(--color-fuchsia-500);".to_owned());
    let _ = styles.insert("color-fuchsia-600".to_owned(), "color: var(--color-fuchsia-600);".to_owned());
    let _ = styles.insert("color-fuchsia-700".to_owned(), "color: var(--color-fuchsia-700);".to_owned());
    let _ = styles.insert("color-fuchsia-800".to_owned(), "color: var(--color-fuchsia-800);".to_owned());
    let _ = styles.insert("color-fuchsia-900".to_owned(), "color: var(--color-fuchsia-900);".to_owned());
    let _ = styles.insert("color-pink-50".to_owned(), "color: var(--color-pink-50);".to_owned());
    let _ = styles.insert("color-pink-100".to_owned(), "color: var(--color-pink-100);".to_owned());
    let _ = styles.insert("color-pink-200".to_owned(), "color: var(--color-pink-200);".to_owned());
    let _ = styles.insert("color-pink-300".to_owned(), "color: var(--color-pink-300);".to_owned());
    let _ = styles.insert("color-pink-400".to_owned(), "color: var(--color-pink-400);".to_owned());
    let _ = styles.insert("color-pink-500".to_owned(), "color: var(--color-pink-500);".to_owned());
    let _ = styles.insert("color-pink-600".to_owned(), "color: var(--color-pink-600);".to_owned());
    let _ = styles.insert("color-pink-700".to_owned(), "color: var(--color-pink-700);".to_owned());
    let _ = styles.insert("color-pink-800".to_owned(), "color: var(--color-pink-800);".to_owned());
    let _ = styles.insert("color-pink-900".to_owned(), "color: var(--color-pink-900);".to_owned());
    let _ = styles.insert("color-rose-50".to_owned(), "color: var(--color-rose-50);".to_owned());
    let _ = styles.insert("color-rose-100".to_owned(), "color: var(--color-rose-100);".to_owned());
    let _ = styles.insert("color-rose-200".to_owned(), "color: var(--color-rose-200);".to_owned());
    let _ = styles.insert("color-rose-300".to_owned(), "color: var(--color-rose-300);".to_owned());
    let _ = styles.insert("color-rose-400".to_owned(), "color: var(--color-rose-400);".to_owned());
    let _ = styles.insert("color-rose-500".to_owned(), "color: var(--color-rose-500);".to_owned());
    let _ = styles.insert("color-rose-600".to_owned(), "color: var(--color-rose-600);".to_owned());
    let _ = styles.insert("color-rose-700".to_owned(), "color: var(--color-rose-700);".to_owned());
    let _ = styles.insert("color-rose-800".to_owned(), "color: var(--color-rose-800);".to_owned());
    let _ = styles.insert("color-rose-900".to_owned(), "color: var(--color-rose-900);".to_owned());
    
    styles
}

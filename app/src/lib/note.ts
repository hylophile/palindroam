const notes = import.meta.glob(['./notes/**/*.html', './notes/*.html'], {
	query: '?raw',
	import: 'default'
});

export function note(notePath: string) {
	const noteFunction = notes[`./notes/${notePath}.html`];
	if (noteFunction === undefined) {
		throw new Error(`${notePath} not found`);
	}
	// return import(`./notes/${notePath}.html?raw`);
	return noteFunction();
}

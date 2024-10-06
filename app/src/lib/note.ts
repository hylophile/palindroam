export function note(noteID: string) {
	return import(`./notes/${noteID}.html?raw`);
}

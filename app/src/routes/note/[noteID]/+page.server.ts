export async function load({ params }) {
	const noteID = params.noteID.endsWith('.html') ? params.noteID : `${params.noteID}.html`;
	const page = await import(`/src/lib/notes/${noteID}?raw`);
	return { content: page.default };
}

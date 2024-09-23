export async function GET({ params }) {
	// const number = Math.floor(Math.random() * 6) + 1;

	const noteID = params.noteID.endsWith('.html') ? params.noteID : `${params.noteID}.html`;
	const page = await import(`/src/lib/notes/${noteID}?raw`);

	return new Response(page.default, {
		headers: {
			'Content-Type': 'text/html'
		}
	});
}

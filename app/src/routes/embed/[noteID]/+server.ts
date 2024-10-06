import type { RouteParams } from './$types';

export async function GET({ params }: { params: RouteParams }) {
	const noteID = params.noteID.endsWith('.html') ? params.noteID : `${params.noteID}.html`;
	const page = await import(`/src/lib/notes/${noteID}?raw`);

	return new Response(page.default, {
		headers: {
			'Content-Type': 'text/html'
			// "cache-control": "max-age=60",
		}
	});
}

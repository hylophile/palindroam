import type { RouteParams } from './$types';
import { note } from '$lib/note';

export async function load({ params }: { params: RouteParams }) {
	const noteID = params.noteID.split('.html')[0];
	const page = await note(noteID);
	return { content: page.default };
}

import type { RouteParams } from './$types';
import { note } from '$lib/note';

export async function load({ params }: { params: RouteParams }) {
	const notePath = params.notePath.split('.html')[0];
	// if (notePath === undefined) { throw new Error() }
	const page = await note(notePath);
	return { content: page };
}

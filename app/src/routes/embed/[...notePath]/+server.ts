import type { RouteParams } from './$types';
import { note } from '$lib/note';

export async function GET({ params }: { params: RouteParams }) {
	const notePath = params.notePath.split('.html')[0];
	const page = await note(notePath);
	return new Response(page, {
		headers: {
			'Content-Type': 'text/html'
		}
	});
}

<script>
	/** @type {import('./$types').PageData} */
	export let data;

	import backlinks from '$lib/backlinks.json';
	import { page } from '$app/stores';
</script>

<div class="flex flex-grow h-full">
	<aside class="bg-slate-300 p-4 w-1/4">
		<h3>Notes</h3>
		<ul class="whitespace-nowrap text-ellipsis">
			{#each Object.keys(backlinks) as note}
				<li><a href={note}>{note.split('.html')[0]}</a></li>
			{/each}
		</ul>
	</aside>
	<div class="p-4 w-full">
		<div class="fl">{@html data.content}</div>
	</div>
	<aside class="bg-slate-300 p-4 w-1/2">
		<h3>Backlinks</h3>
		<ul>
			{#each backlinks[$page.params.noteID] ?? [] as backlink}
				<li><a href={backlink}>{backlink.split('.html')[0]}</a></li>
			{/each}
		</ul>
	</aside>
</div>

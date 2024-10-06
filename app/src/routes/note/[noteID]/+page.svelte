<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import Tooltip from './Tooltip.svelte';

	import backlinks from '$lib/backlinks.json';
	import { page } from '$app/stores';
	const id = $page.params.noteID;
	const currentBacklinks = (backlinks as Record<string, string[]>)[id] ?? [];
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
			{#each currentBacklinks as backlink}
				<li><a href={backlink}>{backlink.split('.html')[0]}</a></li>
			{/each}
		</ul>
	</aside>
</div>
<Tooltip />

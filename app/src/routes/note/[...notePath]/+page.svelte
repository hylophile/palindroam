<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import Tooltip from './Tooltip.svelte';

	import metadata from '$lib/metadata.json';
	import { page } from '$app/stores';
	const id = $page.params.notePath;
	const currentBacklinks = (metadata['backlinks'] as Record<string, string[]>)[id] ?? [];
</script>

<div class="h-full flex">
	<aside class="p-4 w-1/4 h-full">
		<h3>Notes</h3>
		<ul class="whitespace-nowrap text-ellipsis">
			{#each Object.keys(metadata['backlinks']) as note}
				<li><a href={'/note/' + note}>{note.split('.html')[0]}</a></li>
			{/each}
		</ul>
	</aside>
	<div class="bg-slate-100 p-4 h-full w-full overflow-scroll">
		<div class="fl">{@html data.content}</div>
	</div>
	<aside class="p-4 w-1/2">
		<h3>Backlinks</h3>
		<ul>
			{#each currentBacklinks as backlink}
				<li><a href={backlink}>{backlink.split('.html')[0]}</a></li>
			{/each}
		</ul>
	</aside>
</div>
<Tooltip />

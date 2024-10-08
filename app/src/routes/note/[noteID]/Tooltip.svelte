<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchNote } from './fetchnote';

	onMount(() => {
		document.body.addEventListener('mousemove', move);
		document.body.addEventListener('mouseover', hover);
	});

	let mx = $state(0);
	let my = $state(0);
	let hoveringLink = $state(false);
	let content = $state('');

	function move(event: MouseEvent) {
		mx = event.clientX;
		my = event.clientY;
	}

	function hover(event) {
		hoveringLink = event.target.tagName === 'A';
		if (hoveringLink) {
			const noteID = event.target.href.split('/note/')[1];
			if (!noteID) throw new Error();
			fetchNote(noteID).then((text) => {
				content = text;
			});
		} else {
			content = '';
		}
	}
</script>

{#if hoveringLink && content !== ''}
	<div class="absolute bg-slate-100 border p-4" style="top: {my + 10}px; left: {mx + 10}px">
		{@html content}
	</div>
{/if}

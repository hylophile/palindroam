<script>
	// import Header from './Header.svelte';
	import '../app.css';
</script>

<header class="min-h-24 bg-slate-300 flex items-center pl-8">
  <a href="/note/index.md" class="text-4xl font-medium underline text-slate-900 decoration-dotted decoration-4 decoration-cyan-500">
  hylo's notes
  </a>
</header>
<main class="bg-slate-200 flex-grow">
	<slot />
</main>
<footer class="bg-slate-300"></footer>

<style>
</style>

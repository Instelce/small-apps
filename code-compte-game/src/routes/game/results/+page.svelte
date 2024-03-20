<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-10">
	<div class="text-center">
		<span class="text-slate-500">Résultats</span>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">300 points</h3>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">3/10 erreurs</h3>
	</div>

    <div class="min-w-64 grid grid-cols-1 gap-2">
        <Button href="/game">Rejouer</Button>
        <Button variant="outline" disabled>Partager</Button>
    </div>
</div>

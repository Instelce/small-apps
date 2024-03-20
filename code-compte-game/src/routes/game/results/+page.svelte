<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { LastGameType } from '$lib/types';
	import type { PageData } from './$types';

	export let data: PageData;

	let lastGameData = JSON.parse(
		localStorage.getItem('lastGame') as string
	) as unknown as LastGameType;
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-10">
	<div class="text-center">
		<span class="text-gray-500">Résultats</span>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">
			<!-- {#if lastGameData.errors >= 8}
				
			{:else if lastGameData.errors >= 6}

			{/if} -->
		</h3>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">{lastGameData.score} points</h3>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">
			{lastGameData.errors}/10 erreurs
		</h3>
	</div>

	<div class="grid min-w-64 grid-cols-1 gap-2">
		<Button href="/game">Rejouer</Button>
		<Button variant="outline" disabled>Partager</Button>
	</div>
</div>

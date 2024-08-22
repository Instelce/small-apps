<script lang="ts">
	import type { PageData } from './$types';

	import * as Card from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';
	import { Plus } from 'lucide-svelte';
	import PageButton from '$lib/components/PageButton.svelte';

	export let data: PageData;
</script>

<div class="grid min-h-screen w-full place-items-center py-6">
	<Card.Root class="md:w-[500px]">
		<Card.Header class="border-b-1 sticky top-0 z-10 rounded-t-lg bg-card py-4">
			<div class="flex w-full items-center justify-between">
				<Card.Title>Pages</Card.Title>
				<a href="/pages/new"><Plus /></a>
			</div>
		</Card.Header>
		<Card.Content class="flex flex-col p-2">
			{#if data.pages}
				{#each data.pages as page}
					<PageButton {page} colors={data.colors} />
				{/each}
			{/if}
		</Card.Content>
	</Card.Root>
</div>

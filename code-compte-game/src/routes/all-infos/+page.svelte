<script lang="ts">
	import type { PageData } from './$types';
	import * as Table from '$lib/components/ui/table';

	export let data: PageData;
</script>

<div class="flex w-screen flex-col px-[26%]">
	<header class="flex justify-center py-44">
		<h2 class="scroll-m-20 pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0">
			Tous les comptes
		</h2>
	</header>

	{#each data.classes as classe}
		<div class="mb-20 [&:last-child()]:mb-0">
			<div class="mb-6">
				<h3 class="scroll-m-20 mb-2 text-2xl font-semibold tracking-tight">{classe.name}</h3>
				<p>
					{classe.description}
				</p>
			</div>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Numéro</Table.Head>
						<Table.Head>Nom</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each classe.comptes as compte}
						<Table.Row>
							<Table.Cell>N°{compte.num}</Table.Cell>
							<Table.Cell>{compte.nom}</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/each}
</div>

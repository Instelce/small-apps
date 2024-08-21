<script lang="ts">
	import type { PageData } from './$types';

	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Plus } from 'lucide-svelte';
	import { pages } from '$lib/api/pages';
	import * as Avatar from '$lib/components/ui/avatar';
	import { authStore } from '$lib/store/auth';

	export let data: PageData;

	let availableColors = ['bg-orange-400', 'bg-purple-400', 'bg-indigo-400', 'bg-yellow-400'];
	let colors: { [key: string]: string } = {};
	data.users.forEach((user) => {
		colors[user.name] = availableColors[Math.floor(Math.random() * availableColors.length)];
	});
</script>

<div class="grid h-[100vh] w-full place-items-center">
	<Card.Root class="md:w-[500px]">
		<Card.Header class="py-4">
			<div class="flex w-full items-center justify-between">
				<Card.Title>Pages</Card.Title>
				<a href="/pages/new"><Plus /></a>
			</div>
		</Card.Header>
		<Separator orientation="horizontal" />
		<Card.Content class="flex flex-col p-2">
			{#if data.pages}
				{#each data.pages as page}
					<a href={`/pages/${page.id}`}>
						<div
							class="flex w-full flex-1 items-center justify-between rounded border border-transparent p-4 hover:border hover:border-border"
						>
							<h3>{page.name}</h3>
							<div class="flex -space-x-4 transition-all hover:space-x-1">
								{#each page.collaborators as user}
									{#if user.name != data.auth.user}
										<Avatar.Root>
											<Avatar.Image src={user.avatar} alt="@shadcn" />
											<Avatar.Fallback>
												<div
													class={'grid h-10 w-10 place-items-center overflow-hidden rounded-full border border-card text-background shadow-sm ' +
														colors[user.name]}
												>
													{user.name.charAt(0).toUpperCase()}
												</div>
											</Avatar.Fallback>
										</Avatar.Root>
									{/if}
								{/each}
							</div>
						</div>
					</a>
				{/each}
			{/if}
		</Card.Content>
	</Card.Root>
</div>

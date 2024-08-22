<script lang="ts">
	import type { DetailPage } from '$lib/api/pages';
	import * as Avatar from '$lib/components/ui/avatar';

	export let page: DetailPage;
	export let colors: { [key: string]: string };
</script>

<a href={`/pages/${page.id}`}>
	<div
		class="flex w-full flex-1 items-center justify-between rounded border border-transparent p-4 hover:border hover:border-border"
	>
		<h3>{page.name}</h3>
		<div class="flex -space-x-4 hover:space-x-1">
			{#each page.collaborators as user}
				<Avatar.Root class="transition-[margin]">
					<Avatar.Image src={user.avatar} alt="@shadcn" />
					<Avatar.Fallback>
						<div
							class={'shadow-outline grid h-10 w-10 place-items-center overflow-hidden rounded-full text-background ' +
								colors[user.name]}
						>
							{user.name.charAt(0).toUpperCase()}
						</div>
					</Avatar.Fallback>
				</Avatar.Root>
			{/each}
		</div>
	</div>
</a>

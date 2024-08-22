<script lang="ts">
	import { Pencil } from 'lucide-svelte';
	import type { PageData } from './$types';
	import Editor from '$lib/components/editor/Editor.svelte';

	export let data: PageData;

	let page = data.page;
	let content = page ? (page.content ? page.content : '') : '';
</script>

{#if page}
	<div class="container py-4">
		<div class="flex items-center justify-between rounded-lg border bg-card px-4 py-4">
			<h1 class=" font-medium">{page.name}</h1>

			<Pencil size="20" />
		</div>

		<Editor {content} />
	</div>
{/if}

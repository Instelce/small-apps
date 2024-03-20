<script lang="ts">
	import '../app.pcss';
	import { Button } from '$lib/components/ui/button/index.js';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { toggleMode } from 'mode-watcher';
	import { page } from '$app/stores';
</script>

<div class="absolute w-full flex justify-center p-5 gap-2">
	<Button variant="ghost" href="/" class={$page.route.id == '/' ? 'bg-secondary' : ''}>
		Acceuil
	</Button>
	<Button variant="ghost">
		Tous
	</Button>
</div>

<slot />

<div class="absolute bottom-5 right-5">
	<Button on:click={toggleMode} variant="outline" size="icon">
		<Sun
			class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
		/>
		<Moon
			class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
		/>
		<span class="sr-only">Toggle theme</span>
	</Button>
</div>

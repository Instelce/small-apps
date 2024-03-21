<script lang="ts">
	import '../app.pcss';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { toggleMode } from 'mode-watcher';
	import { page } from '$app/stores';
	import { Braces, Github, Heart } from 'lucide-svelte';
</script>

<div class="absolute flex w-full justify-center gap-2 p-5">
	<Button variant="ghost" href="/" class={$page.route.id == '/' ? 'bg-secondary' : ''}>
		Acceuil
	</Button>
	<Button variant="ghost" href="/all-infos" class={$page.route.id == '/all-infos' ? 'bg-secondary' : ''}>
		Tous
	</Button>
	<Button on:click={toggleMode} variant="ghost" size="icon">
		<Sun
			class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
		/>
		<Moon
			class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
		/>
		<span class="sr-only">Toggle theme</span>
	</Button>
</div>

<slot />

<footer class="flex w-screen flex-col items-center justify-center gap-6 pb-4 pt-8 bg-slate-50 dark:bg-slate-900">
	<div class="flex gap-4">
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<Button
					size="icon"
					variant="outline"
					href="https://github.com/Instelce/small-apps"
					target="_blank"
					builders={[builder]}
				>
					<Github />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Repo github</p>
			</Tooltip.Content>
		</Tooltip.Root>
	
		<!-- <Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<Button
					size="icon"
					variant="outline"
					href="/api"
					builders={[builder]}
				>
					<Braces />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>API</p>
			</Tooltip.Content>
		</Tooltip.Root> -->
	</div>

	<p class="flex items-center gap-1">
		Made with <Heart size="20" fill="red" color="red" /> by
		<a href="https://instelce.vercel.app/" target="_blank" class="mt-1 border-b-2">Instelce</a>
	</p>
</footer>

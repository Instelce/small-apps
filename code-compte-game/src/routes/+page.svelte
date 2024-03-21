<script lang="ts">
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';
	import { onMount } from 'svelte';

	let difficulty = { value: 2, label: 'Medium' };

	onMount(() => {
		if (browser) {
			if (!localStorage.getItem('difficulty')) {
				localStorage.setItem('difficulty', JSON.stringify({ value: 1, label: 'Ez' }));
			}
			difficulty = JSON.parse(localStorage.getItem('difficulty') as string);
		}
	});
</script>

<div class="flex h-[95vh] w-screen flex-col items-center justify-center gap-8">
	<div class="flex flex-col items-center gap-4">
		<h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
			Apprendre les codes
		</h1>

		<p class="max-w-96 text-center">
			Si tu as envie d'apprendre les codes des comptes en comptabilité (mmh), alors tu es au bon
			endroit !
		</p>
	</div>

	<div class="grid min-w-64 grid-cols-1 gap-4">
		<div class="grid grid-cols-1 gap-2">
			<span class="text-center text-sm">Jouer à retrouver</span>
			<div class="grid grid-cols-2 gap-2">
				<Button href="/game?type=codes">Les codes</Button>
				<Button href="/game?type=names">Les noms</Button>
			</div>
		</div>

		<Select.Root
			selected={difficulty ? difficulty : { value: 2, label: 'Medium' }}
			onSelectedChange={(value) => {
				localStorage.setItem('difficulty', JSON.stringify(value));
			}}
		>
			<Select.Trigger>
				<Select.Value placeholder="Difficulté" />
			</Select.Trigger>
			<Select.Content>
				<Select.Label>Difficulté</Select.Label>
				<Select.Group>
					<Select.Item value={1} label="Ez" />
					<Select.Item value={2} label="Medium" />
					<Select.Item value={3} label="Hard" />
				</Select.Group>
			</Select.Content>
		</Select.Root>
	</div>
</div>

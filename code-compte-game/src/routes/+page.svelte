<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';

	let difficulty = localStorage.getItem("difficulty") as string
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-8">
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
		<Select.Root
			selected={difficulty
				? JSON.parse(difficulty)
				: { value: 1, label: 'Ez' }}
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
		<Button href="/game">Jouer</Button>
	</div>
</div>

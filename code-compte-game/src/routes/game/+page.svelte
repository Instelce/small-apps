<script lang="ts">
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import { arrayChoice, arrayMultipleChoice, arrayShuffle, randomInt } from '$lib/utils';

	import { Button, buttonVariants } from '$lib/components/ui/button';
	import AnswerButton from '$lib/components/AnswerButton.svelte';
	import { Progress } from '$lib/components/ui/progress';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Table from '$lib/components/ui/table';
	import type { ClasseType } from '$lib/types';

	export let data: PageData;

	let score = writable(0);
	let errors = writable(0);
	let maxProgress = 10;
	let progress = writable(0);

	let isAnswered = writable(false);

	let quizData = writable({
		compteClass: {} as ClasseType,
		question: 'chargement...',
		questionAnswer: '',
		propositions: ['1', '2', '3', '4']
	});

    // redirect to results page
    $: {
        if ($progress >= maxProgress) {
            goto('/game/results')
			localStorage.setItem("lastGame", JSON.stringify({
				score: $score,
				errors: $errors
			}))
        }
    }

    // generate next question
	function next() {
		let randomClass = arrayChoice(data.classes);
		let randomCompte = arrayChoice(randomClass.comptes);

		let exclude = [...$quizData.propositions];
		exclude.push(randomCompte.num.toString());
        exclude.push($quizData.questionAnswer)

        console.log("Exclude :", exclude);

		let comptes = data.classes
			.map((classe) => classe.comptes)
			.flat(1)
			.map((c) => c.num.toString());
		let comptesPropositions = arrayMultipleChoice(comptes, 3, exclude);
		comptesPropositions.push(randomCompte.num.toString());

		let propositions = arrayShuffle(comptesPropositions);

		console.log(propositions);

		quizData.set({
			compteClass: randomClass,
			question: randomCompte.nom,
			questionAnswer: randomCompte.num.toString(),
			propositions: propositions
		});
	}

	onMount(() => {
		next();
	});
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-6">
	<!-- question -->
	<div class="text-center">
		<span class="text-gray-500">Retrouve le <strong>code</strong> de ce compte</span>
		<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">{$quizData.question}</h3>
	</div>

	<!-- propositions -->
	<div class="grid grid-cols-2 gap-2">
		{#each $quizData.propositions as num (num)}
			<AnswerButton
				answer={num.toString()}
				goodAnswer={$quizData.questionAnswer.toString()}
				isAnswered={$isAnswered}
				onClick={() => {
					if (num === $quizData.questionAnswer) {
						score.update((s) => s + 100);
					} else {
						errors.update(e => e + 1)
					}
					isAnswered.set(true);
				}}
			/>
		{/each}

		<!-- buttons -->
		<div class="col-span-2 grid grid-cols-1 gap-2">
			{#if $isAnswered}
				<Button
					class="mt-4"
					on:click={() => {
						progress.update((p) => p + 1);

						isAnswered.set(false);
						next();
					}}>Suivant</Button
				>

				<!-- more infos dialog -->
				<Dialog.Root>
					<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>
						Plus d'infos
					</Dialog.Trigger>
					<Dialog.Content>
						<Dialog.Header>
							<Dialog.Title>Fait partie des {$quizData.compteClass.name.toLowerCase()}</Dialog.Title>
							<Dialog.Description>
								{$quizData.compteClass.description}
							</Dialog.Description>
						</Dialog.Header>

						<Table.Root>
							<Table.Header>
								<Table.Row>
									<Table.Head>Numéro</Table.Head>
									<Table.Head>Nom</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each $quizData.compteClass.comptes as compte}
									<Table.Row>
										<Table.Cell>N°{compte.num}</Table.Cell>
										<Table.Cell>{compte.nom}</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
                            <Table.Caption>Compte associé à la classe</Table.Caption>
						</Table.Root>
					</Dialog.Content>
				</Dialog.Root>
			{/if}
		</div>
	</div>

	<!-- progress bar -->
	<div class="absolute bottom-10 flex w-[40%] flex-col items-center gap-2">
		<p class="text-lg font-medium">{$score}</p>
		<Progress value={$progress} max={maxProgress} class="h-3 w-[100%]" />
	</div>
</div>

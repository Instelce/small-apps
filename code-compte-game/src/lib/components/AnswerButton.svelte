<script lang="ts">
	import { writable } from 'svelte/store';
	import { Button } from './ui/button';

	export let answer: string;
	export let goodAnswer: string;
	export let isAnswered: boolean;
	export let onClick: () => void = () => {};

	let isClicked = writable(false);
</script>

<Button
	variant="outline"
	size="lg"
	class={$isClicked || isAnswered
		? answer === goodAnswer
			? 'bg-green-300 hover:bg-green-200 dark:bg-green-900 hover:dark:bg-green-800'
			: $isClicked
				? 'bg-red-300 hover:bg-red-200 dark:bg-red-900 hover:dark:bg-red-800'
				: ''
		: ''}
	on:click={() => {
		onClick();
		if (!isAnswered) {
			isClicked.set(true);
		}
	}}
>
	<span class="text-lg">{answer.length > 4 ? '' : 'N°'}{answer}</span>
</Button>

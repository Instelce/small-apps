<script lang="ts">
	import type { PageData } from './$types';
	import * as Card from '$lib/components/ui/card';
	import * as Form from '$lib/components/ui/form';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { superForm } from 'sveltekit-superforms';

	export let data: PageData;

	let form = superForm(data.form, {
		dataType: 'json',
		resetForm: false
	});

	let { form: formData, enhance } = form;
</script>

<div class="grid h-[100vh] w-full place-items-center">
	<Card.Root class="md:w-[500px]">
		<Card.Header>
			<div class="flex w-full items-center justify-between">
				<Card.Title>New Page</Card.Title>
			</div>
		</Card.Header>
		<Card.Content>
			<form method="post" action="/pages/new" use:enhance class="">
				<Form.Field {form} name="name">
					<Form.Control let:attrs>
						<Form.Label>Name</Form.Label>
						<Input {...attrs} bind:value={$formData.name} />
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>

				<div>
					<Form.Button class="mt-2 w-full">Create</Form.Button>
				</div>
			</form>
		</Card.Content>
	</Card.Root>
</div>

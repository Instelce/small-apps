<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as Form from '$lib/components/ui/form';

	import { toast } from 'svelte-sonner';

	import { superForm, type Infer, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { join, joinSchema, type JoinSchema } from '$lib/api/auth';
	import { LoaderCircle } from 'lucide-svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	export let data: SuperValidated<Infer<JoinSchema>>;

	let form = superForm(data, {
		SPA: true,
		validators: zodClient(joinSchema),
		dataType: 'json',
		timeoutMs: 1000,
		onUpdate: ({ form }) => {
			if (form.valid) {
				toast.success('Your account has been created');
				let query = new URLSearchParams();
				query.set('from', 'join');
				setTimeout(() => goto(`?${query.toString()}`), 0);
			}
		}
	});

	let { form: formData, enhance, message, delayed } = form;
</script>

<form method="post" action="?/join" use:enhance class="space-y-2">
	{#if $message}
		<div>
			<div class="mb-2 text-center text-sm font-medium text-destructive">{$message}</div>
		</div>
	{/if}
	<Form.Field {form} name="name">
		<Form.Control let:attrs>
			<Form.Label>Name</Form.Label>
			<Input {...attrs} bind:value={$formData.name} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="email">
		<Form.Control let:attrs>
			<Form.Label>Email</Form.Label>
			<Input {...attrs} bind:value={$formData.email} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>Password</Form.Label>
			<Input {...attrs} bind:value={$formData.password} type="password" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<div>
		<Form.Button disabled={$delayed} class="mt-2 w-full">
			{#if $delayed}
				<LoaderCircle />
			{/if}
			Create an account
		</Form.Button>
	</div>
</form>

<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as Form from '$lib/components/ui/form';

	import { superForm, type Infer, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { login, loginSchema, type LoginSchema } from '$lib/api/auth';
	import { LoaderCircle } from 'lucide-svelte';

	export let data: SuperValidated<Infer<LoginSchema>>;

	let form = superForm(data, {
		SPA: true,
		validators: zodClient(loginSchema),
		dataType: 'json',
		resetForm: false,
		timeoutMs: 1000,
		onUpdate: async ({ form }) => {
			if (form.valid) {
				let response = await login(form.data);

				if (response.errors) {
					form.message = response.errors.description;
				}
			}
		}
	});

	let { form: formData, enhance, message, delayed } = form;
</script>

<form method="post" use:enhance class="space-y-2">
	{#if $message}
		<div>
			<div class="mb-2 text-center text-sm font-medium text-destructive">{$message}</div>
		</div>
	{/if}
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
			Log in
		</Form.Button>
	</div>
</form>

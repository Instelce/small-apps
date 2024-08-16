<script lang="ts">
import * as Form from "$lib/components/ui/form";
import {
  superForm,
  superValidate,
  type Infer,
  type SuperValidated,
} from "sveltekit-superforms";
import { login, loginSchema, type LoginSchema } from "$lib/api/auth";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { Input } from "$lib/components/ui/input";

export let data: SuperValidated<Infer<LoginSchema>>;

const form = superForm(data, {
  validators: zodClient(loginSchema),
  dataType: "json",
  resetForm: false,
});

const { form: formData, submitting, enhance } = form;
</script>

<form method="post" use:enhance on:submit|preventDefault={async (e) => {
  e.preventDefault();
  login(form);
  }}>
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
        <Input {...attrs} bind:value={$formData.password} />
        </Form.Control>
        <Form.FieldErrors />
    </Form.Field>
    <div>
        <Form.Button type="submit" class="mt-4 w-full" disabled={$submitting}>Submit</Form.Button>
    </div>
</form>
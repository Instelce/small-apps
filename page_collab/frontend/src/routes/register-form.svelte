<script lang="ts">
import {
  superForm,
  type Infer,
  type SuperValidated,
} from "sveltekit-superforms";
import { registerSchema, type RegisterSchema } from "$lib/api/auth";
import { zodClient } from "sveltekit-superforms/adapters";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";

export let data: SuperValidated<Infer<RegisterSchema>>;

const form = superForm(data, {
  validators: zodClient(registerSchema),
  dataType: "json",
});

const { form: formData, enhance } = form;
</script>

<form action="" method="post" use:enhance class="space-y-4">
    <Form.Field {form} name="username">
        <Form.Control let:attrs>
        <Form.Label>Name</Form.Label>
        <Input {...attrs} bind:value={$formData.username} />
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
        <Input {...attrs} bind:value={$formData.password} />
        </Form.Control>
        <Form.FieldErrors />
    </Form.Field>
    <div>
        <Form.Button class="mt-4 w-full">Submit</Form.Button>
    </div>
</form>

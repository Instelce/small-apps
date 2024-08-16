<script lang="ts">
import type { PageData } from "./$types";
import * as Tabs from "$lib/components/ui/tabs/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import RegisterForm from "./register-form.svelte";
import LoginForm from "./login-form.svelte";

export let data: PageData;
</script>

<div class="w-full h-[100vh] flex justify-center items-center">
  <Tabs.Root value="register" class="w-[400px]">
    <Tabs.List class="grid w-full grid-cols-2">
      <Tabs.Trigger value="register">Register</Tabs.Trigger>
      <Tabs.Trigger value="login">Login</Tabs.Trigger>
    </Tabs.List>

    <!-- Register -->
    <Tabs.Content value="register">
      <Card.Root>
        <Card.Header>
          <Card.Title>Welcome</Card.Title>
          <Card.Description>Create your account here.</Card.Description>
        </Card.Header>
        <Card.Content class="space-y-2">
          <RegisterForm data={data.registerForm} />
        </Card.Content>
      </Card.Root>
    </Tabs.Content>

    <!-- Login -->
    <Tabs.Content value="login">
      <Card.Root>
        <Card.Header>
          <Card.Title>Welcome back</Card.Title>
          <Card.Description>Happy to see you again !</Card.Description>
        </Card.Header>
        <Card.Content class="space-y-2">
          <LoginForm data={data.loginForm} />
        </Card.Content>
      </Card.Root>
    </Tabs.Content>

  </Tabs.Root>
</div>

<style>

</style>

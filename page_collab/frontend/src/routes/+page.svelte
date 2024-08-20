<script lang="ts">
	import type { PageData } from './$types';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';
	import FormLogin from './form-login.svelte';
	import FormJoin from './form-join.svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	export let data: PageData;

	console.log($page.url.searchParams);

	$: fromJoin = $page.url.searchParams.get('from') === 'join';
	$: console.log(fromJoin);

	$: if (!fromJoin) {
		let query = new URLSearchParams();
		query.set('from', '');
		goto(`?${query.toString()}`);
	}
</script>

<div class="grid h-[100vh] w-full place-items-center">
	<Tabs.Root value={fromJoin ? 'login' : 'join'} class="w-[400px]">
		<Tabs.List class="grid w-full grid-cols-2">
			<Tabs.Trigger value="join">Join</Tabs.Trigger>
			<Tabs.Trigger value="login">Login</Tabs.Trigger>
		</Tabs.List>

		<!-- Join -->
		<Tabs.Content value="join">
			<Card.Root>
				<Card.Header>
					<Card.Title>Welcome</Card.Title>
					<Card.Description>Create your account here.</Card.Description>
				</Card.Header>
				<Card.Content>
					<FormJoin data={data.joinForm} />
				</Card.Content>
			</Card.Root>
		</Tabs.Content>

		<!-- Login -->
		<Tabs.Content value="login">
			<Card.Root>
				<Card.Header>
					<Card.Title>Welcome back</Card.Title>
					<Card.Description>Really happy to see you again !</Card.Description>
				</Card.Header>
				<Card.Content>
					<FormLogin data={data.loginForm} />
				</Card.Content>
			</Card.Root>
		</Tabs.Content>
	</Tabs.Root>
</div>

<script lang="ts">
	import type { LayoutData } from './$types';
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner';

	export let data: LayoutData;
</script>

<Toaster richColors />

<slot></slot>

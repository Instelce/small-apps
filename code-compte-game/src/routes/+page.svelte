<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-8">
	<div class="flex flex-col items-center gap-4">
        <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
            Apprendre les codes
        </h1>

        <p class="max-w-96 text-center">
            Si tu as envie d'apprendre les codes des comptes en comptabilité (mmh), alors tu es au bon
            endroit !
        </p>
    </div>
	<Button class="min-w-64" href="/game">Jouer</Button>
</div>

<script lang="ts">
	import { invoke } from '@tauri-apps/api';
    import SignIn from '../components/sign-in.svelte';
    import Main from '../components/main.svelte';
	import axios from 'axios';
    import { store } from '$lib/store';
    import { onMount } from 'svelte';
	import { validated } from "$lib/stores/ValidateStore"

	onMount(() => {
		invoke('close_splashscreen');
	})

</script>

{#if $validated}
		<Main />
	{:else}
		<SignIn />
{/if}

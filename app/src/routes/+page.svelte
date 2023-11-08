<script lang="ts">
	import { invoke } from '@tauri-apps/api';
    import SignIn from '../components/sign-in.svelte';
    import Main from '../components/main.svelte';

	setTimeout(() => {
		invoke('close_splashscreen');
	}, 3000);

	let loggedIn: boolean = false;

</script>

{#if loggedIn}
		<Main />
	{:else}
		<SignIn />
{/if}

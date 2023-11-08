<script lang="ts">
	import { invoke } from '@tauri-apps/api';
    import SignIn from '../components/sign-in.svelte';
    import Main from '../components/main.svelte';
	import axios from 'axios';
    import { store } from '$lib/store';
    import { onMount } from 'svelte';
	
	let error = null;
	let validated = false;

	invoke('close_splashscreen');

	// async function getStoreToken() {
	// 	const val: String | null = await store.get("token");
	// 	if (!val) {
	// 		return null;
	// 	}
	// 	return val;
	// }

	// onMount(async () => {
	// 	try {
	// 		const token = getStoreToken();

	// 		if (!token) {
	// 			return null;
	// 		}

	// 		const config = {
    //         	headers: { Authorization: `Bearer ${token}` }
    //     	}

	// 		const res = await axios.get(("http://localhost:8080/auth/validate"), config)
	// 			.then(res => {
	// 				return true;
	// 			})
	// 			.catch(err => {
	// 				return false;
	// 				console.log(err);
	// 			})
	// 			.finally(() => {
	// 				invoke('close_splashscreen');
	// 			})
	// 	} catch(e) {
	// 		error = e;
	// 	}
	// })

</script>

{#if validated}
		<Main />
	{:else}
		<SignIn />
{/if}

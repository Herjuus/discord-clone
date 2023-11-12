<script lang="ts">
    const servers = [{}, {}, {}]
	import { Plus } from 'lucide-svelte'
	import { validated } from '$lib/stores/ValidateStore';

	function HandleSignout() {
		$validated = false;
	}
</script>

<main class="flex h-screen justify-between">
	<div class="w-[4rem] bg-surface-800 h-full p-2 space-y-2">
		<div class="h-full space-y-2 overflow-y-scroll hide-scrollbar">
			<div class="hover:cursor-pointer bg-primary-400 hover:rounded-2xl rounded-3xl duration-200 w-full aspect-square"></div>
			<div class="h-1 bg-surface-700"></div>
			{#each servers as server}
				<div class="hover:cursor-pointer bg-primary-400 hover:rounded-2xl rounded-3xl duration-200 w-full aspect-square"></div>
			{/each}
			<div class="h-1 bg-surface-700"></div>
			<div class="hover:cursor-pointer bg-secondary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square flex justify-center items-center"><Plus size=32 /></div>
		</div>
	</div>
	<div class="flex-1 h-full justify-between flex flex-row">
		<div class="bg-surface-800/50 w-64 h-full flex flex-col items-center">
			<div class="flex-1"></div>
			<button on:click={HandleSignout} class="btn bg-surface-900 w-full rounded-md w-3xl">Sign out</button>
		</div>
		<div class="flex-1 bg-surface-800 h-full"></div>
		<div class="w-60 bg-surface-800/50 h-full"></div>
	</div>
</main>
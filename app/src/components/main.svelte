<script lang="ts">
    const servers = [{}, {}, {}]
	import { Plus } from 'lucide-svelte'
</script>

<main class="flex h-screen justify-between">
	<div class="w-[4rem] h-full p-2 space-y-2">
		<!-- <div class="flex justify-between">
			<div class="w-3 aspect-square bg-red-500 hover:bg-red-600 rounded-full hover:cursor-pointer"></div>
			<div class="w-3 aspect-square bg-yellow-500 hover:bg-yellow-600 rounded-full hover:cursor-pointer"></div>
			<div class="w-3 aspect-square bg-green-500 hover:bg-green-600 rounded-full hover:cursor-pointer"></div>
		</div> -->
		<div class="h-full space-y-2 overflow-y-scroll hide-scrollbar">
			<div class="hover:cursor-pointer bg-primary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square"></div>
			<div class="h-1 bg-surface-700"></div>
			{#each servers as server}
				<div class="hover:cursor-pointer bg-primary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square"></div>
			{/each}
			<div class="h-1 bg-surface-700"></div>
			<div class="hover:cursor-pointer bg-secondary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square flex justify-center items-center"><Plus size=32 /></div>
		</div>
	</div>
	<div class="flex-1 h-full justify-between flex flex-row">
		<div class="bg-surface-500 w-64 h-full"><button></button></div>
		<div class="flex-1 bg-surface-600 h-full"></div>
		<div class="w-72 h-full"></div>
	</div>
</main>
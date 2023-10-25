<script>
	const servers = [{}, {}, {}]
</script>

<main class="flex h-screen justify-between">
	<div class="w-16 h-full p-2 space-y-2 overflow-y-scroll hide-scrollbar">
		<div class="hover:cursor-pointer bg-primary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square"></div>
		<div class="h-1 bg-surface-700"></div>
		{#each servers as server}
			<div class="hover:cursor-pointer bg-primary-400 hover:rounded-xl rounded-2xl duration-200 w-full aspect-square"></div>
		{/each}
	</div>
	<div class="flex-1 h-full justify-between flex flex-row">
		<div class="bg-surface-500 w-72 h-full"><button></button></div>
		<div class="flex-1 bg-surface-600 h-full"></div>
		<div class="w-72 h-full"></div>
	</div>
</main>
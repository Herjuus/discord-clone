<script lang="ts">
	import '../app.postcss';
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	import { invoke } from '@tauri-apps/api';

	setTimeout(() => {
		invoke('close_splashscreen');
	}, 3000);

	let loggedIn: boolean = false;
	let loading: boolean = false;

	function handleSignin(e: Event) {
		e.preventDefault();
		loading = true;
		setTimeout(() => {
			loading = false;
			// loggedIn = true;
		}, 1000);
	}
</script>

<main>
	{#if loggedIn}
		<slot />
	{:else}
		<div class="w-screen h-screen backdrop-blur-sm flex flex-col justify-center items-center">
			<form on:submit={handleSignin} class="card rounded p-4 w-96 space-y-5 flex flex-col">
				<div>
					<h3 class="h3">Login</h3>
					<span class="text-tertiary-600 text-sm">Sign into your account to continue</span>
				</div>
				<div class="flex flex-col">
					<input autocomplete="off" class="order-2 input focus:border-primary-500 invalid:border-error-500 invalid:focus:border-error-500 rounded text-sm peer" placeholder="name@example.com" type="email">
					<span class="order-1 text-tertiary-800 text-sm peer-focus:text-tertiary-600 duration-200">Email adress</span>
					<span class="order-2 hidden peer-invalid:block text-error-500 text-sm">
						Invalid email address
					</span>
				</div>
				<div class="flex flex-col">
					<input autocomplete="off" class="order-2 input focus:border-primary-500 invalid:border-error-500 invalid:focus:border-error-500 rounded text-sm peer" placeholder="********" type="password">
					<span class="order-1 text-tertiary-800 text-sm peer-focus:text-tertiary-600 duration-200">Password</span>
					<span class="order-2 hidden peer-invalid:block text-error-500 text-sm">
						Invalid password
					</span>
				</div>
				<button disabled={loading} type="submit" class="btn rounded variant-filled-primary text-tertiary-500 text-sm w-min">{#if loading}
					Signing in...
					{:else}
					Sign in
				{/if}</button>
			</form>
		</div>
	{/if}
</main>

<script lang="ts">
    import { getToastStore, type ToastSettings } from "@skeletonlabs/skeleton";
    import { invoke } from "@tauri-apps/api";

    const toastStore = getToastStore();

    let loading: boolean = false;

    let email = "";
    let password = "";

    async function handleSignin(e: Event) {
		e.preventDefault();
        loading = true;
        invoke('handle_sign_in', { email: email, password: password })
        .then((res) => {
            const t: ToastSettings = {
                message: res as string,
                background: 'variant-filled-success',
            };
            toastStore.trigger(t);
            console.log(res as string);
            loading = false;
        })
        .catch((err) => {
            console.log(err);
            const t: ToastSettings = {
                message: err as string,
                background: 'variant-filled-error',
            };
            toastStore.trigger(t);
            loading = false;
        }).finally(() => {
            loading = false;
        })
	}
</script>

<div class="w-screen h-screen backdrop-blur-sm flex flex-col justify-center items-center">
    <form on:submit={handleSignin} class="card rounded p-4 w-96 space-y-5 flex flex-col">
        <div>
            <h3 class="h3">Login</h3>
            <span class="text-tertiary-600 text-sm">Sign into your account to continue</span>
        </div>
        <div class="flex flex-col">
            <input bind:value={email} autocomplete="off" class="order-2 input focus:border-primary-500 invalid:border-error-500 invalid:focus:border-error-500 rounded text-sm peer" placeholder="name@example.com" type="email">
            <span class="order-1 text-tertiary-800 text-sm peer-focus:text-tertiary-600 duration-200">Email adress</span>
            <span class="order-2 hidden peer-invalid:block text-error-500 text-sm">
                Invalid email address
            </span>
        </div>
        <div class="flex flex-col">
            <input bind:value={password} autocomplete="off" class="order-2 input focus:border-primary-500 invalid:border-error-500 invalid:focus:border-error-500 rounded text-sm peer" placeholder="********" type="password">
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
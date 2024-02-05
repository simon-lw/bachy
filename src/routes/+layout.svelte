<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Bachyselector from '$lib/BachySelector.svelte';
	import '../app.postcss';
	import { AppShell, AppBar, initializeStores, Toast, getToastStore } from '@skeletonlabs/skeleton';

	initializeStores();
	const toastStore = getToastStore();

	async function saveClicked() {
		let res = await invoke('save_command');

		const toastString = {
			message: res,
			autohide: false
		};

		toastStore.trigger(toastString);

		console.log(res);
	}

	async function loadClicked() {
		let res = await invoke('load_command');

		if (!res) {
			console.log('did work');
		} else {
			console.log('did not work');
			const toastString = {
				message: res,
				autohide: false
			};
			toastStore.trigger(toastString);
		}

		console.log('clicked');
	}
	let titel = 'Backup1';
</script>

<Toast />

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">{titel}</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<button
					class="btn btn-sm variant-ghost-surface"
					on:click={() => {
						loadClicked();
					}}
					>Load<button />
				</button>
				<button
					class="btn btn-sm variant-ghost-surface"
					on:click={() => {
						saveClicked();
					}}
					>Save<button />
				</button>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<Bachyselector on:add on:remove on:selectionChanged />
	</svelte:fragment>
	<slot />
</AppShell>

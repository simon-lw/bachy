<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Bachyselector from '$lib/BachySelector.svelte';
	import '../app.postcss';
	import { AppShell, AppBar, initializeStores, Toast, getToastStore, localStorageStore} from '@skeletonlabs/skeleton';
	import {selectedStore, dataStore} from '$lib/DataStore' 

	initializeStores();
	const toastStore = getToastStore();

	// const selectedStore = localStorageStore('selectedBachy', []);
	// const removedStore = localStorageStore('lastRemovedBachy', []);
	// const bachyStore = localStorageStore('bachyStore', JSON.parse("{\"name\":\"\"}"));

	let titel =  $dataStore?.name ?? "Neues Backup";

	async function saveClicked() {
		let res = await invoke('save_command', {$dataStore});

		const toastString = {
			message: res,
			autohide: false
		};

		toastStore.trigger(toastString);
	}

	async function loadClicked() {
		invoke('load_command')
		.then((value)=>{
			$dataStore = JSON.parse(value);
		}).catch((err)=>{
			const toastString = {
				message: err,
				autohide: false
			};
			toastStore.trigger(toastString);
		});
	}

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
		<Bachyselector on:add on:remove on:selectionChanged/>
	</svelte:fragment>
	<slot />
</AppShell>

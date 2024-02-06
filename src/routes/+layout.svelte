<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import Bachyselector from '$lib/BachySelector.svelte';
	import '../app.postcss';
	import {
		AppShell,
		AppBar,
		initializeStores,
		Toast,
		Modal,
		getToastStore,
		getModalStore,

	} from '@skeletonlabs/skeleton';
	import { selectedStore, dataStore} from '$lib/DataStore';

	initializeStores();
	const toastStore = getToastStore();
	const modalStore = getModalStore();

	let titel = $dataStore?.name ?? 'Neues Backup';

	async function saveClicked() {
		let config = JSON.stringify($dataStore);
		console.log(config);
		let res = await invoke('save_command', { config });

		const toastString = {
			message: res,
			autohide: false
		};

		toastStore.trigger(toastString);
	}

	async function loadClicked() {
		let doLoad = () => {
					invoke('load_command')
			.then((value) => {
				$dataStore = JSON.parse(value);
			})
			.catch((err) => {
				const toastString = {
					message: err,
					autohide: false
				};
				toastStore.trigger(toastString);
			});
		}

		// if ($hasChangedStore) {
			/**
			 *  @type {import('@skeletonlabs/skeleton').ModalSettings}
			 */
			const modal = {
				type: 'confirm',
				// Data
				title: 'Please Confirm',
				body: 'Are you sure you wish to proceed? All unsaved changes will be lost!',
				// TRUE if confirm pressed, FALSE if cancel pressed
				response: (/** @type {boolean} */ confired) => {
					if(confired) {
						doLoad();
					} 
				} 
			};

			modalStore.trigger(modal);
		// }
		// else {
		// 	doLoad();
		// }
	}
</script>

<Toast />
<Modal />

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

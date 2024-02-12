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
		getModalStore
	} from '@skeletonlabs/skeleton';
	import { selectedStore, dataStore } from '$lib/DataStore';

	initializeStores();
	const toastStore = getToastStore();
	const modalStore = getModalStore();

	let title = $dataStore?.name ?? 'New Backup';

	async function saveClicked() {
		// save title
		if ($dataStore) {
			$dataStore.name = title;
		}

		let config = JSON.stringify($dataStore);
		console.log(config);
		let res = await invoke('save_command', { config });

		const toastString = {
			message: res,
		};

		toastStore.trigger(toastString);
	}

	async function loadClicked() {
		let doLoad = () => {
			invoke('load_command')
				.then((value) => {

					let parsed = JSON.parse(value);
					$dataStore = parsed; 
					$selectedStore = -1;
					title = parsed.name;
				})
				.catch((err) => {
					const toastString = {
						message: err,
					};
					toastStore.trigger(toastString);
				});
		};

		/**
		 *  @type {import('@skeletonlabs/skeleton').ModalSettings}
		 */
		const modal = {
			type: 'confirm',
			title: 'Please Confirm',
			body: 'Are you sure you wish to proceed? All unsaved changes will be lost!',
			response: (/** @type {boolean} */ confired) => {
				if (confired) {
					doLoad();
				}
			}
		};

		modalStore.trigger(modal);
	}
</script>

<Toast />
<Modal />

<AppShell>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<div class="flex flex-row space-x-4 items-center">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						fill="currentColor"
						class="w-6 h-6 opacity-25"
					>
						<path
							d="M21.731 2.269a2.625 2.625 0 0 0-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 0 0 0-3.712ZM19.513 8.199l-3.712-3.712-12.15 12.15a5.25 5.25 0 0 0-1.32 2.214l-.8 2.685a.75.75 0 0 0 .933.933l2.685-.8a5.25 5.25 0 0 0 2.214-1.32L19.513 8.2Z"
						/>
					</svg>
					<input type="text" class="bg-transparent border-0 text-xl h-full" bind:value={title} />
				</div>
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

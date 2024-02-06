<script>
	import { FileDropzone, Table } from '@skeletonlabs/skeleton';
	import { tableMapperValues } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';
	import { selectedStore, dataStore, defaultBachy } from '$lib/DataStore';
	import { listen } from '@tauri-apps/api/event';

	let fileHover = false;

	listen('tauri://file-drop', (event) => {
		fileHover = false;
		console.log(event);
	});

	listen('tauri://file-drop-hover', (event) => {
		fileHover = true;
		console.log(event);
	});

	listen('tauri://file-drop-cancelled', (event) => {
		fileHover = false;
		console.log(event);
	});

	const dispatch = createEventDispatcher();

	let currentBachy = defaultBachy;

	// let numNotSynced = currentBachy.files.map((x) => x.isSynced).filter((x) => !x).length;
	let tableSimple = {
		// A list of heading labels.
		head: ['Path', 'Last Backup', 'Is Synced'],
		// The data visibly shown in your table body UI.
		body: tableMapperValues(currentBachy.files, ['path', 'lastBackup', 'isSynced']),
		// Optional: The data returned when interactive is enabled and a row is clicked.
		// meta: tableMapperValues(currentBachy.files, ['id', 'path', 'lastBackup', 'isSynced']),
		foot: [
			'Total',
			''
			//`<code class="code"> ${numNotSynced ? numNotSynced : 'No'} Elements out of Sync</code>`
		]
	};

	selectedStore.subscribe((sel) => {
		console.log(sel);

		let filtered = $dataStore?.bachys?.filter((x) => x.id == sel);

		if (filtered != null && filtered.length > 0) {
			currentBachy = filtered[0];
		} else {
			currentBachy = defaultBachy;
		}

		updateTableData();
	});

	function updateStore() {
		console.log('update called');

		let index = $dataStore?.bachys?.findIndex((x) => x.id == currentBachy.id) ?? -1;

		if (index >= 0 && $dataStore != null) {
			$dataStore.bachys[index].name = currentBachy.name;
			$dataStore.bachys[index].icon = currentBachy.icon;
			$dataStore.bachys[index].target = currentBachy.target;
		}
	}

	function updateTableData() {
		tableSimple = {
			head: ['Path', 'Last Backup', 'Is Synced'],
			body: tableMapperValues(currentBachy.files, ['path', 'lastBackup', 'isSynced']),
			foot: ['Total', '']
		};
	}
</script>

<section class="flex flex-col h-full w-full overfolw-y-auto relative">
	<div class="absolute w-full h-full p-5 z-10 {fileHover && $selectedStore >= 0 ? '' : 'hidden'}">
		<div
			class="flex flex-col w-full h-full border border-4 border-surface-400 border-dashed
			variant-glass-surface rounded-lg justify-center"
		>
			<div class="flex-1 flex justify-center items-center max-h-40">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
					class="w-14 h-14"
				>
					<path
						d="M9.97.97a.75.75 0 0 1 1.06 0l3 3a.75.75 0 0 1-1.06 1.06l-1.72-1.72v3.44h-1.5V3.31L8.03 5.03a.75.75 0 0 1-1.06-1.06l3-3ZM9.75 6.75v6a.75.75 0 0 0 1.5 0v-6h3a3 3 0 0 1 3 3v7.5a3 3 0 0 1-3 3h-7.5a3 3 0 0 1-3-3v-7.5a3 3 0 0 1 3-3h3Z"
					/>
					<path
						d="M7.151 21.75a2.999 2.999 0 0 0 2.599 1.5h7.5a3 3 0 0 0 3-3v-7.5c0-1.11-.603-2.08-1.5-2.599v7.099a4.5 4.5 0 0 1-4.5 4.5H7.151Z"
					/>
				</svg>
			</div>
			<div class="flex-1 flex h-10 justify-center items-center max-h-40">
				<span class="text-xl">Add a Path by drag and drop!</span>
			</div>
			<div class="flex-1 flex h-10 justify-center items-center max-h-40">
				<span class="text-l">Links will not be followed!</span>
			</div>
		</div>
	</div>
	<div class="h-full flex flex-col p-3">
		<div class="flex flex-row space-x-5 p-3 sticky bottom-0">
			<input
				class="input basis-1/12 text-center"
				type="text"
				placeholder="Icon"
				bind:value={currentBachy.icon}
				on:change={updateStore}
				disabled={$selectedStore < 0}
			/>
			<input
				class="input"
				type="text"
				placeholder="Bachy Name"
				bind:value={currentBachy.name}
				on:change={updateStore}
				disabled={$selectedStore < 0}
			/>
		</div>

		<Table
			source={tableSimple}
			class="container flex-auto"
			element="table"
			regionBody=""
			regionHead="sticky top-0"
			regionFoot="sticky bottom-0"
		/>
		<div class="flex flex-row space-x-5 p-3 sticky bottom-0">
			<label class="label flex-auto w-80">
				<input
					class="input"
					type="text"
					placeholder="Path to Backup Target"
					bind:value={currentBachy.target}
					on:change={updateStore}
					disabled={$selectedStore < 0}
				/>
			</label>
			<button
				on:click={() => {
					dispatch('backupClicked', {});
				}}
				type="button"
				class="flex-auto w-1 bg-gradient-to-br variant-gradient-tertiary-secondary disabled:variant-filled-secondary text-l rounded-lg hover:variant-ghost-primary active:variant-filled-primary"
				disabled={$selectedStore < 0}>Backup</button
			>
		</div>
	</div>
</section>

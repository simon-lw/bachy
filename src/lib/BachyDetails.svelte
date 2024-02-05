<script>
	import { Table } from '@skeletonlabs/skeleton';
	import { tableMapperValues } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';
	import { selectedStore, dataStore , defaultBachy} from '$lib/DataStore';

	const dispatch = createEventDispatcher();

	let currentBachy = defaultBachy;

	selectedStore.subscribe((sel) => {

		let filtered = $dataStore?.bachys.filter((x)=>x.id==sel);

		if (filtered != null && filtered.length > 0) {
			currentBachy = filtered[0];
		}
		else {
			currentBachy = defaultBachy;
		}
	});

	// let numNotSynced = currentBachy.files.map((x) => x.isSynced).filter((x) => !x).length;

	const tableSimple = {
		// A list of heading labels.
		head: ['Path', 'Last Backup', 'Is Synced'],
		// The data visibly shown in your table body UI.
		body: tableMapperValues(currentBachy.files, ['path', 'lastBackup', 'isSynced']),
		// Optional: The data returned when interactive is enabled and a row is clicked.
		meta: tableMapperValues(currentBachy.files, ['id', 'path', 'lastBackup', 'isSynced']),
		// Optional: A list of footer labels.
		foot: [
			'Total',
			'',
			//`<code class="code"> ${numNotSynced ? numNotSynced : 'No'} Elements out of Sync</code>`
		]
	};

	function updateStore() {
		console.log("update called");
		
		let index = $dataStore?.bachys?.findIndex((x)=>x.id == currentBachy.id) ?? -1;

		if (index >= 0 && $dataStore != null)
		{
			$dataStore.bachys[index].name = currentBachy.name;
			$dataStore.bachys[index].icon = currentBachy.icon;
			$dataStore.bachys[index].target= currentBachy.target;

		}
	}
</script>

<section class="flex flex-col h-full p-3 w-full overfolw-y-auto">
	<div class="flex flex-row space-x-5 p-3 sticky bottom-0">
		<input
			class="input basis-1/12 center"
			type="text"
			placeholder="Icon"
			bind:value={currentBachy.icon}
			on:change={updateStore}
			disabled={$selectedStore < 0}
		/>
		<input class="input" type="text" placeholder="Bachy Name" bind:value={currentBachy.name} on:change={updateStore}
		disabled={$selectedStore < 0}/>
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
			class="flex-auto w-1 variant-filled-primary disabled:variant-filled-secondary text-l rounded-lg hover:variant-ghost-primary active:variant-filled-primary"
			disabled={$selectedStore < 0}
			>Backup</button
		>
	</div>
</section>

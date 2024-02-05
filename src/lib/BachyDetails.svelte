<script>
	import { Table } from '@skeletonlabs/skeleton';
	import { tableMapperValues } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let targetPath = '';
	export let fileInfo = [
		{ id: 0, path: 'C:/source', lastBackup: '12.04.2022', isSynced: true },
		{ id: 1, path: 'C:/source', lastBackup: '12.04.2022', isSynced: false }
	];

	for (let index = 2; index < 30; index++) {
		fileInfo.push({ id: index, path: 'C:/source', lastBackup: '12.04.2022', isSynced: false });
	}

	let numNotSynced = fileInfo.map((x) => x.isSynced).filter((x) => !x).length;
	let currentIcon = '';
	let currentName = '';

	const tableSimple = {
		// A list of heading labels.
		head: ['Path', 'Last Backup', 'Is Synced'],
		// The data visibly shown in your table body UI.
		body: tableMapperValues(fileInfo, ['path', 'lastBackup', 'isSynced']),
		// Optional: The data returned when interactive is enabled and a row is clicked.
		meta: tableMapperValues(fileInfo, ['id', 'path', 'lastBackup', 'isSynced']),
		// Optional: A list of footer labels.
		foot: [
			'Total',
			'',
			`<code class="code"> ${numNotSynced ? numNotSynced : 'No'} Elements out of Sync</code>`
		]
	};
</script>

<section class="flex flex-col h-full p-3 w-full overfolw-y-auto">
	<div class="flex flex-row space-x-5 p-3 sticky bottom-0">
		<input class="input basis-1/12 center" type="text" placeholder="Icon" bind:value={currentIcon} />
		<input class="input" type="text" placeholder="Bachy Name" bind:value={currentName} />
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
				bind:value={targetPath}
			/>
		</label>
		<button
			on:click={() => {
				dispatch('backupClicked', {});
			}}
			type="button"
			class="flex-auto w-1 variant-filled-primary text-l rounded-lg hover:variant-ghost-primary active:variant-filled-primary"
			>Backup</button
		>
	</div>
</section>

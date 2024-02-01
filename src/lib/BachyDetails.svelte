<script>
	import { Table } from '@skeletonlabs/skeleton';
	import { tableMapperValues } from '@skeletonlabs/skeleton';

	let fileInfo = [
		{ id: 0, path: 'C:/source', lastBackup: '12.04.2022', isSynced: false },
		{ id: 1, path: 'C:/source1', lastBackup: '12.04.2022', isSynced: true },
		{ id: 2, path: 'C:/source2', lastBackup: '12.04.2022', isSynced: false },
		{ id: 3, path: 'C:/folder/profile.png', lastBackup: '12.04.2022', isSynced: false },
		{ id: 4, path: 'C:/source', lastBackup: '12.04.2022', isSynced: false },
		{ id: 5, path: 'C:/source1', lastBackup: '12.04.2022', isSynced: true },
		{ id: 6, path: 'C:/source2', lastBackup: '12.04.2022', isSynced: false },
		{ id: 7, path: 'C:/folder/profile.png', lastBackup: '12.04.2022', isSynced: false }
	];

	for (let index = 0; index < 15; index++) {
		fileInfo.push(fileInfo[0]);
	}

	let numNotSynced = fileInfo.map((x) => x.isSynced).filter((x) => !x).length;
	const tableSimple = {
		// A list of heading labels.
		head: ['Path', 'Last Backup', 'Is Synced'],
		// The data visibly shown in your table body UI.
		body: tableMapperValues(fileInfo, ['path', 'lastBackup', 'isSynced']),
		// Optional: The data returned when interactive is enabled and a row is clicked.
		meta: tableMapperValues(fileInfo, ['id', 'path', 'lastBackup', 'isSynced']),
		// Optional: A list of footer labels.
		foot: ['Total', '', `<code class="code"> ${numNotSynced} Elements out of Sync</code>`]
	};
</script>

<section class="flex flex-col h-full p-3">
	<Table
		source={tableSimple}
		class="container h-auto flex-auto h-14"
		element="table"
		regionBody=""
		regionHead="sticky top-0"
		regionFoot="sticky bottom-0"
	/>
	<div class="flex flex-row space-x-5 p-3 sticky bottom-0">
		<label class="label flex-auto w-80">
			<input class="input" type="text" placeholder="Path to Backup Target" />
		</label>
		<button
			type="button"
			class="flex-auto w-1 variant-filled-primary text-l rounded-lg hover:variant-ghost-primary active:variant-filled-primary ">Backup</button
		>
	</div>
</section>

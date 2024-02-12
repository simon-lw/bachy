<script>
	import { selectedStore, dataStore } from '$lib/DataStore';

	let icons = ['❗', '📽️', '🖼️', '📃', '🧪'];

	/**
	 * @param {number} id
	 */
	function changeSelection(id) {
		$selectedStore = id;
	}

	function addClicked() {
		console.log($dataStore);
		let ids = $dataStore?.bachys?.map((x) => x.id);

		if (!ids || ids.length <= 0) {
			ids = [-1];
		}

		console.log(ids);

		let id = Math.max(...ids) + 1;
		$dataStore?.bachys?.push({
			id: id,
			name: 'New Bachy',
			icon: icons[id % icons.length],
			target: '',
			files: []
		});

		changeSelection(id);
		$dataStore = $dataStore;
	}

	function removeClicked() {
		if ($dataStore?.bachys != null) {
			$dataStore.bachys = $dataStore?.bachys?.filter((x) => x.id != $selectedStore);
			changeSelection(-1);

			$dataStore = $dataStore;
		}
	}
</script>

<section
	class="flex flex-col flex-start h-full text-token card px-4 space-y-4 overflow-auto w-48 resize-x"
>
	<div class="flex bg-inherit flex1 flex-row flex-start space-x-5 sticky top-0 max-h-16">
		<button
			on:click={() => addClicked()}
			type="button"
			class="btn-icon variant-soft-success text-xl min-w-0 flex-1 rounded-lg"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				class="w-6 h-6"
			>
				<path
					fill-rule="evenodd"
					d="M12 3.75a.75.75 0 0 1 .75.75v6.75h6.75a.75.75 0 0 1 0 1.5h-6.75v6.75a.75.75 0 0 1-1.5 0v-6.75H4.5a.75.75 0 0 1 0-1.5h6.75V4.5a.75.75 0 0 1 .75-.75Z"
					clip-rule="evenodd"
				/>
			</svg>
		</button>
		<button
			on:click={() => removeClicked()}
			type="button"
			class="btn-icon variant-soft-error text-xl min-w-0 flex-1 rounded-lg"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				class="w-6 h-6"
			>
				<path
					fill-rule="evenodd"
					d="M4.25 12a.75.75 0 0 1 .75-.75h14a.75.75 0 0 1 0 1.5H5a.75.75 0 0 1-.75-.75Z"
					clip-rule="evenodd"
				/>
			</svg>
		</button>
	</div>
	<div class="flex-9 flex flex-col flex-start items-stretch content-stretch space-y-1 h-full">
		{#if $dataStore?.bachys != null}
			{#each $dataStore.bachys as bachy, index (bachy.id)}
				<button
					type="button"
					on:click={() => changeSelection(bachy.id)}
					class="btn min-h-16 hover:variant-ghost-primary active:variant-filled-primary overflow-hidden
					{bachy.id == $selectedStore ? 'variant-filled-primary' : ''}"
				>
					<div class="flex flex-1 flex-row flex-start items-center">
						<span id="icon" class="flex-none text-2xl">{bachy.icon}</span>
						<span class="flex-1">{bachy.name}</span>
					</div>
				</button>
			{/each}
		{/if}
	</div>

	<ul class="list" />
</section>

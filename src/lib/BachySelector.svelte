<script>
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let bachies = [
		{ id: 0, name: 'Important', desc: 'Mein erster Bachy', icon: '❗' },
		{ id: 1, name: 'Videos', desc: 'Mein erster Bachy', icon: '📽️' },
		{ id: 2, name: 'Pics', desc: 'Mein erster Bachy', icon: '🖼️' },
		{ id: 3, name: 'Documents', desc: 'mein erster bachy', icon: '📃' },
		{ id: 4, name: 'Test', desc: 'mein erster bachy', icon: '🧪' },
		{ id: 5, name: 'Important', desc: 'Mein erster Bachy', icon: '❗' },
		{ id: 6, name: 'Videos', desc: 'Mein erster Bachy', icon: '📽️' },
		{ id: 7, name: 'Pics', desc: 'Mein erster Bachy', icon: '🖼️' },
		{ id: 8, name: 'Documents', desc: 'mein erster bachy', icon: '📃' },
		{ id: 9, name: 'Test', desc: 'mein erster bachy', icon: '🧪' }
	];


	let icons = ['❗','📽️','🖼️','📃','🧪'];

	export let selectedBachy = 0;

	/**
	 * @param {number} id
	 */
	function changeSelection(id) {
		selectedBachy = id;
		console.log(id);
		dispatch('selectionChanged', { selected: selectedBachy });
	}

	function addClicked() {
		dispatch('add', {});
		let id = Math.max(...bachies.map(x => x.id), -1)+1;
		bachies.push({ id: id, name: 'Test', desc: 'mein erster bachy', icon: icons[id%icons.length] });
		bachies = bachies;
		console.log("add clicked");
	}

	function removeClicked() {
		dispatch('remove', { selected: selectedBachy });
		bachies = bachies.filter(x=>x.id != selectedBachy)
		
		console.log("remove clicked");
		console.log(selectedBachy);
		selectedBachy = -1;
	}
</script>

<section class="flex flex-col flex-start w-full h-full text-token card px-4 space-y-4 overflow-auto">
	<div class="flex bg-inherit flex1 flex-row flex-start space-x-5 sticky top-0 ">
		<button
			on:click={() => addClicked()}
			type="button"
			class="btn-icon hover:variant-ghost-success text-xl min-w-0 flex-1 rounded-lg">➕</button
		>
		<button
			on:click={() => removeClicked()}
			type="button"
			class="btn-icon hover:variant-ghost-error text-xl min-w-0 flex-1 rounded-lg">❌</button
		>
	</div>
	<div class="flex-9 flex flex-col flex-start items-stretch content-stretch space-y-1 h-full">
		{#each bachies as bachy, index (bachy.id)}
			<button
				type="button"
				on:click={() => changeSelection(bachy.id)}
				class="btn min-h-16 hover:variant-ghost-primary active:variant-filled-primary
				{bachy.id == selectedBachy ? 'variant-filled-primary' : ''}"
			>
				<div class="flex flex-1 flex-row flex-start items-center">
					<span id="icon" class="flex-none text-2xl">{bachy.icon}</span>
					<span class="flex-1">{bachy.name}</span>
				</div>
			</button>
		{/each}
	</div>

	<ul class="list" />
</section>

<script>
	export let bachies = [
		{ id:0, name: 'Important', desc: 'Mein erster Bachy', icon: '❗' },
		{ id:1, name: 'Videos', desc: 'Mein erster Bachy', icon: '📽️' },
		{ id:2, name: 'Pics', desc: 'Mein erster Bachy', icon: '🖼️' },
		{ id:3, name: 'Documents', desc: 'mein erster bachy', icon: '📃' },
		{ id:4, name: 'Test', desc: 'mein erster bachy', icon: '🧪' },
	];
	export let selectedBachy = 0;

	/**
	 * @param {number} id
	 */
	function handleClicked(id){
		selectedBachy = id;
		console.log(id);
	}
	 
</script>

<section class="flex flex-col w-full h-full text-token card p-4 space-y-4">
	<div class="flex flex1 flex-row flex-start space-x-5">
		<button type="button" class="btn-icon hover:variant-ghost-success text-xl min-w-0 flex-1 rounded-lg"
			>➕</button
		>
		<button type="button" class="btn-icon hover:variant-ghost-error text-xl min-w-0 flex-1 rounded-lg"
			>❌</button
		>
	</div>

	<div
		class="flex-9 flex flex-col flex-start items-stretch content-stretch space-y-3 h-full"
	>
		{#each bachies as bachy, index (bachy.id) }
			<button type="button" on:click={()=> handleClicked(index)} class="btn min-h-16 hover:variant-ghost-primary active:variant-filled-primary
				{index==selectedBachy ? 'variant-filled-primary' : ''}">
				<div class="flex flex-1 flex-row flex-start items-center">
					<span id="icon" class="flex-none text-3xl">{bachy.icon}</span>
					<span class="flex-1">{bachy.name}</span>
				</div>
			</button>
		{/each}
	</div>

	<ul class="list" />
</section>

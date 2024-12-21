<script lang="ts">
	import Key from '$lib/components/keyboard/key.svelte'
	import { fade, fly } from 'svelte/transition'
	import { Keyboard } from '$lib/states/keyboard.svelte'
	import { setContext } from 'svelte'

	let { name, placeholder } = $props()

	const keyboard: Keyboard = new Keyboard()
	setContext('keyboard', keyboard)
</script>

<input
	{name}
	{placeholder}
	type="text"
	readonly
	value={keyboard.input}
	onfocus={() => (keyboard.isOpen = true)} />

{#if keyboard.isOpen}
	<div
		role="button"
		tabindex="0"
		class="outside-keyboard"
		transition:fade
		onclick={() => (keyboard.isOpen = false)}
		onkeydown={() => {}}>
	</div>

	<div class="keyboard" transition:fly={{ y: 200, duration: 300 }}>
		<div class="keys">
			{#each keyboard.keys as key}
				<Key {key} />
			{/each}
		</div>
	</div>
{/if}

<style>
	.keyboard {
		position: fixed;
		left: 0;
		bottom: 0;
		width: 100%;
		padding: 5px 0;
		background: #ececec;
		box-shadow: 0 0 50px rgba(0, 0, 0, 0.5);
	}

	.keys {
		text-align: center;
		background-color: #ececec;
	}

	.outside-keyboard {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		background-color: rgba(0, 0, 0, 0.2);
	}
</style>

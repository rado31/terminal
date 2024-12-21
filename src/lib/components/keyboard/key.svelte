<script lang="ts">
	import Icon from '@iconify/svelte'
	import { Keyboard } from '$lib/states/keyboard.svelte'
	import { getContext } from 'svelte'

	let { key } = $props()

	const keyboard: Keyboard = getContext('keyboard')

	const insertLineBreak = (key: string) => {
		const breakers = ['BACKSPACE', 'P', 'ENTER', '?']
		return breakers.includes(key)
	}

	const backspace = () => (keyboard.input = keyboard.input.slice(0, -1))
	const space = () => (keyboard.input += ' ')
	const writeDefaultKey = () => (keyboard.input += key)

	const wideClass = 'key wide-key'
	const extraWideClass = 'key extra-wide-key'
</script>

{#if key === 'BACKSPACE'}
	<button type="button" onclick={backspace} class={wideClass}>
		<Icon icon="ic:round-backspace" width="24" height="24" />
	</button>
{:else if key === 'SPACE'}
	<button type="button" onclick={space} class={extraWideClass}>
		<Icon icon="ri:space" width="24" height="24" />
	</button>
{:else if key === 'ENTER'}
	<button type="button" class={wideClass}>
		<Icon icon="fluent:arrow-enter-left-24-filled" width="24" height="24" />
	</button>
{:else if key === 'SHIFT'}
	<button type="button" class={wideClass}>
		<Icon icon="streamline:shift-solid" width="24" height="24" />
	</button>
{:else}
	<button type="button" onclick={writeDefaultKey} class="key">
		{key}
	</button>
{/if}

{#if insertLineBreak(key)}<br />{/if}

<style>
	.key {
		height: 65px;
		width: 7%;
		margin: 3px;
		border-radius: 5px;
		border: none;
		background-color: white;
		color: black;
		box-shadow: 0 0 3px -1px rgba(0, 0, 0, 0.3);
		border-bottom: 1px solid #b5b5b5;
		font-size: 1.05rem;
		outline: none;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		vertical-align: top;
		padding: 0;
		position: relative;
	}

	.key:active {
		background: rgba(255, 255, 255, 0.12);
	}

	.wide-key {
		width: 12%;
	}

	.extra-wide-key {
		width: 50%;
	}
</style>

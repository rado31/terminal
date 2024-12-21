<script lang="ts">
	import Header from './header.svelte'
	import Days from './days.svelte'
	import Button from './button.svelte'
	import Footer from './footer.svelte'
	import { Datepicker } from '$lib/states/datepicker.svelte'
	import { fly } from 'svelte/transition'
	import { setContext } from 'svelte'
	import { ButtonType } from '$lib/types/datepicker'

	let { name } = $props()
	const dp = new Datepicker()

	setContext('datepicker', dp)
</script>

<input
	{name}
	type="text"
	readonly
	value={dp.formatedDateInput()}
	onfocus={() => (dp.isOpen = true)} />

{#if dp.isOpen}
	<div
		role="button"
		tabindex="0"
		class="outside-datepicker"
		onclick={() => (dp.isOpen = false)}
		onkeydown={null}>
	</div>

	<div class="datepicker" transition:fly={{ y: -200, duration: 300 }}>
		<Header />
		<Days />

		<div class="datepicker-buttons">
			{#each dp.prevMonthButtons as button}
				<Button {button} type={ButtonType.Prev} />
			{/each}

			{#each dp.currMonthButtons as button}
				<Button {button} />
			{/each}

			{#each dp.nextMonthButtons as button}
				<Button {button} type={ButtonType.Next} />
			{/each}
		</div>

		<Footer />
	</div>
{/if}

<style>
	.datepicker {
		position: absolute;
		z-index: 1;
		width: 351px;
		background: #ececec;
		padding: 8px;
		border-radius: 8px;
		box-shadow:
			0 6px 16px 0 rgba(0, 0, 0, 0.08),
			0 3px 6px -4px rgba(0, 0, 0, 0.12),
			0 9px 28px 8px rgba(0, 0, 0, 0.05);
		background-color: white;
	}

	.datepicker-buttons {
		display: grid;
		justify-content: center;
		grid-template-columns: repeat(7, 30px);
		gap: 15px;
		margin: 10px 0px;
	}

	.outside-datepicker {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
	}
</style>

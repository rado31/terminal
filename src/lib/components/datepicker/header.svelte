<script lang="ts">
	import Icon from '@iconify/svelte'
	import { getContext } from 'svelte'
	import { Datepicker } from '$lib/states/datepicker.svelte'

	const dp: Datepicker = getContext('datepicker')
</script>

<div class="datepicker-header">
	<button type="button" onclick={() => dp.prevMonth()}>
		<Icon
			icon="fluent:arrow-circle-left-32-regular"
			width="32"
			height="32" />
	</button>
	<div class="month">{dp.getMonthTitleByID()}</div>
	<div class="year">{dp.year}</div>
	<button type="button" onclick={() => dp.nextMonth()}>
		<Icon
			icon="fluent:arrow-circle-right-32-regular"
			width="32"
			height="32" />
	</button>
</div>

<style>
	button {
		background-color: white;
		border: none;
		color: rgba(0, 0, 0, 0.6);
	}

	button:hover {
		cursor: pointer;
		color: black;
	}

	.datepicker-header {
		width: auto;
		display: flex;
		align-items: center;
		justify-content: space-between;
		font: 14px Arial;
		font-weight: 600;
		border-bottom: 1px solid rgba(5, 5, 5, 0.06);
		padding-bottom: 8px;
	}
</style>

<script lang="ts">
	import type { Datepicker } from '$lib/states/datepicker.svelte'
	import { ButtonType, type ButtonProp } from '$lib/types/datepicker'
	import { getContext } from 'svelte'

	let { button, type }: ButtonProp = $props()

	const dp: Datepicker = getContext('datepicker')
	let isToday = $derived(
		dp.currDate.getDate() === button.text &&
			dp.currDate.getMonth() === dp.month &&
			dp.currDate.getFullYear() === dp.year,
	)
	let isSelected = $derived(dp.input.getTime() === button.date.getTime())
	let isDisabled = $derived(
		button.date.getTime() < dp.currDate.getTime() && !isToday,
	)
	let isPrevMonth = type === ButtonType.Prev ? true : false
	let isNextMonth = type === ButtonType.Next ? true : false

	const changeDate = () => {
		dp.input = button.date
		dp.isOpen = false
	}
</script>

<button
	type="button"
	class="datepicker-btn"
	onclick={changeDate}
	disabled={isDisabled}
	class:isDisabled
	class:isSelected
	class:isToday
	class:isPrevMonth
	class:isNextMonth>
	{button.text}
</button>

<style>
	.datepicker-btn {
		cursor: pointer;
		border: none;
		border-radius: 4px;
		background-color: white;
		font: 14px Arial;
		height: 30px;
	}

	.isDisabled {
		pointer-events: none;
		background-color: #f5f5f5;
		color: rgba(0, 0, 0, 0.25);
	}

	.isToday {
		border: 1px solid #0018a9;
		color: black;
	}

	.isNextMonth,
	.isPrevMonth {
		color: rgba(0, 0, 0, 0.25);
	}

	.isSelected {
		background: #0018a9;
		color: white;
	}
</style>

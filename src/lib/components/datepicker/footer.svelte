<script lang="ts">
	import type { Datepicker } from '$lib/states/datepicker.svelte'
	import { getContext } from 'svelte'

	const dp: Datepicker = getContext('datepicker')
</script>

<div>
	<button type="button" onclick={() => dp.initOrReset()}>Şugün</button>
</div>

<style>
	div {
		text-align: center;
		padding: 15px 0px;
		border-top: 1px solid rgba(5, 5, 5, 0.06);
	}

	button {
		border: none;
		background-color: white;
		font: 14px Arial;
		color: #1677ff;
	}

	button:hover {
		color: #69b1ff;
		cursor: pointer;
	}
</style>

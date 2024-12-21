<script lang="ts">
	import Icon from '@iconify/svelte'
	import '../app.css'

	let { children } = $props()
</script>

<header>
	<div class="container">
		<div class="img-container">
			<img src="railways.svg" alt="" />
		</div>

		<div class="version">1.0.51</div>

		<div class="info">
			<div class="phone">383275</div>
			<div class="img-container">
				<img src="call-center.png" alt="" />
			</div>
			<div class="phone">383279</div>
		</div>

		<div class="actions">
			<button class="lang">
				<Icon icon="twemoji:flag-turkmenistan" width="36" height="36" />
			</button>
			<button class="lang">
				<Icon icon="twemoji:flag-russia" width="36" height="36" />
			</button>
			<button type="button">
				<Icon icon="fluent:power-20-filled" width="36" height="36" />
			</button>
		</div>
	</div>
</header>

{@render children()}

<style>
	header {
		background-color: #0018a9;
	}

	.container {
		height: 80px;
		max-width: 1200px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin: 0 auto;
		padding: 0px 20px;
	}

	.img-container {
		height: 50px;
	}

	img {
		height: 100%;
	}

	.version {
		color: white;
	}

	.info,
	.actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 20px;
	}

	.phone {
		color: white;
		font-size: 20px;
	}

	button {
		border: none;
		background-color: transparent;
		color: white;
	}

	button:hover {
		cursor: pointer;
	}
</style>

<script lang="ts">
	import { logged_in } from '$lib/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	// Can't get cookies to work for the life of me
	onMount(async () => {
		if ($logged_in) {
			goto('/');
		}
	});
</script>

<div class="login">
	<a rel="external" href="/login"><button type="button">Login</button></a>
</div>

<style>
	.login {
		margin: auto;
		width: 50%;
		border: 3px solid green;
		padding: 10px;
	}
</style>

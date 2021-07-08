<script lang="ts">
	import { goto } from '$app/navigation';
	import { logged_in, keys } from '$lib/stores';
	import { onMount } from 'svelte';
	import { notifier, Notifications } from 'smelte';

	import BlobSign from '$lib/components/BlobSign.svelte';
	import NewKey from '$lib/components/NewKey.svelte';
	import BlobVerify from '$lib/components/BlobVerify.svelte';
	import VCSign from '$lib/components/VCSign.svelte';
	import VCVerify from '$lib/components/VCVerify.svelte';
	import ISCCSign from '$lib/components/ISCCSign.svelte';
	import ISCCVerify from '$lib/components/ISCCVerify.svelte';

	onMount(async () => {
		if (!$logged_in) {
			goto('/');
		}
		const res = await fetch('/keys');
		if (res.ok) {
			$keys = await res.json();
		} else if (res.status == 404) {
			$keys = [];
		} else {
			const res = await fetch('/user');
			if (!res.ok) {
				// TODO that's not a great way to manage the session
				logged_in.set(false);
				goto('/');
			}
			const { message } = await res.json();
			notifier.error(message);
		}
	});
</script>

<h2>Keys</h2>
{#each $keys as key}
	<div class="key-div">
		<h4>{key}</h4>
		<BlobSign keyName={key} />
		<BlobVerify keyName={key} />
		<VCSign keyName={key} />
		<VCVerify keyName={key} />
		<ISCCSign keyName={key} />
		<ISCCVerify keyName={key} />
	</div>
{/each}
<NewKey />

<Notifications />

<style>
	.key-div {
		margin: auto;
		width: 50%;
		border: 3px solid green;
		padding: 10px;
	}
</style>

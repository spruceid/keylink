<script lang="ts">
	import { Dialog, Button, TextField, notifier, Notifications } from 'smelte';
	import { keys } from '$lib/stores';

	let showDialog = false;
	let keyName = null;

	let submit = async () => {
		let res = await fetch('/keys', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name: keyName })
		});
		if (res.ok) {
			$keys = [...$keys, keyName];
			showDialog = false;
			keyName = null;
		} else {
			notifier.error(await res.text());
		}
	};
</script>

<Dialog bind:value={showDialog}>
	<h5 slot="title">New Key</h5>
	<p>{JSON.stringify($keys)}</p>
	<h6>Name</h6>
	<TextField bind:value={keyName} type="string" min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog = false)}>Cancel</Button>
		<Button text outlined disabled={!keyName} on:click={submit}>Create</Button>
	</div>
</Dialog>

<div class="py-2">
	<Button on:click={() => (showDialog = true)}>New Key</Button>
</div>

<Notifications />

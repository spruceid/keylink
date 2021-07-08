<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';
	import { keys } from '$lib/stores';

	let showDialog = false;
	let keyName = null;

	let submit = async () => {
		const form = new URLSearchParams();
		form.append('name', keyName);
		let res = await fetch('/keys', {
			method: 'POST',
			body: form
		});
		if (res.ok) {
			$keys.push(keyName);
			showDialog = false;
		} else {
			throw new Error(await res.text());
		}
	};
</script>

<Dialog bind:value={showDialog}>
	<h5 slot="title">Name</h5>
	<TextField bind:value={keyName} type="string" min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog = false)}>Cancel</Button>
		<Button text outlined disabled={!keyName} on:click={async () => await submit()}>Create</Button>
	</div>
</Dialog>

<div class="py-2">
	<Button on:click={() => (showDialog = true)}>New Key</Button>
</div>

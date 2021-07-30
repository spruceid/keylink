<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	import Highlight from 'svelte-highlight';
	import json from 'svelte-highlight/src/languages/json';
	import github from 'svelte-highlight/src/styles/github';
	let code = '';

	let showDialog1 = false;
	let showDialog2 = false;
	export let keyName;
	let credential = null;

	let submit = async () => {
		let res = await fetch('/credentials/issue', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				key: keyName,
				credential: JSON.parse(credential)
			})
		});
		if (res.ok) {
			code = JSON.stringify(await res.json(), null, 2);
			showDialog2 = true;
			showDialog1 = false;
		} else {
			throw new Error(await res.text());
		}
	};
</script>

<svelte:head>
	{@html github}
</svelte:head>

<Dialog bind:value={showDialog1}>
	<h5 slot="title">Issue</h5>
	<h6>Verifiable Credential</h6>
	<TextField bind:value={credential} textarea outlined min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!credential} on:click={async () => await submit()}>Issue</Button
		>
	</div>
</Dialog>

<Dialog bind:value={showDialog2}>
	<h5 slot="title">VC</h5>
	<Highlight language={json} {code} />
	<div slot="actions">
		<Button text on:click={() => (showDialog2 = false)}>OK</Button>
	</div>
</Dialog>

<!-- <div class="py-2"> -->
<Button on:click={() => (showDialog1 = true)}>Issue VC</Button>
<!-- </div> -->

<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	let showDialog1 = false;
	let showDialog2 = false;
	// TODO should this verify the VC was issued by this key -- or should we have only a signle button to verify VCs
	export let keyName;
	let credential = null;
	let valid = null;

	let submit = async () => {
		let res = await fetch('/credentials/verify', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				credential: JSON.parse(credential)
			})
		});
		if (res.ok) {
			valid = true;
			showDialog2 = true;
			showDialog1 = false;
		} else {
			valid = false;
		}
	};
</script>

<Dialog bind:value={showDialog1}>
	<h5 slot="title">Verify</h5>
	<h6>Verifiable Credential</h6>
	<TextField bind:value={credential} textarea outlined min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!credential} on:click={async () => await submit()}
			>Verify</Button
		>
	</div>
</Dialog>

<Dialog bind:value={showDialog2}>
	{#if valid}
		<p>Valid</p>
	{:else}
		<p>Not valid</p>
	{/if}
	<div slot="actions">
		<Button text on:click={() => (showDialog2 = false)}>OK</Button>
	</div>
</Dialog>

<!-- <div class="py-2"> -->
<Button on:click={() => (showDialog1 = true)}>Verify VC</Button>
<!-- </div> -->

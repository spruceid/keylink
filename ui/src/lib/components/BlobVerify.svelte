<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	let showDialog1 = false;
	let showDialog2 = false;
	export let keyName;
	let doc = null;
	let signature = null;
	let valid = null;

	let submit = async () => {
		const form = new URLSearchParams();
		form.append('key', keyName);
		form.append('doc', doc);
		form.append('sig', signature);
		let res = await fetch('/bytes/verify', {
			method: 'POST',
			body: form
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
	<h6>Document</h6>
	<TextField bind:value={doc} textarea outlined min-length="1" />
	<h6>Signature</h6>
	<TextField bind:value={signature} textarea outlined min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!doc} on:click={async () => await submit()}>Verify</Button>
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
<Button on:click={() => (showDialog1 = true)}>Verify Bytes</Button>
<!-- </div> -->

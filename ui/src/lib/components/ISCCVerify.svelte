<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	let showDialog1 = false;
	let showDialog2 = false;
	export let keyName;
	let credential = null;
	let valid = null;
	let files;

	let submit = async () => {
		const formData = new FormData();
		formData.append('credential', credential);
		formData.append('doc', files[0]);
		let res = await fetch('/iscc/verify', {
			method: 'POST',
			body: formData
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
	<h5 slot="title">Verify ISCC VC</h5>
	<h6>Verifiable Credential</h6>
	<TextField bind:value={credential} textarea outlined min-length="1" />
	<h6>File</h6>
	<input type="file" bind:files />
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!credential || !files} on:click={async () => await submit()}
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
<Button on:click={() => (showDialog1 = true)}>Verify ISCC VC</Button>
<!-- </div> -->

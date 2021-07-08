<script lang="ts">
	import { Dialog, Button, TextField } from 'smelte';

	let showDialog1 = false;
	let showDialog2 = false;
	export let keyName;
	let doc = null;
	let signature = null;

	let submit = async () => {
		const form = new URLSearchParams();
		form.append('key', keyName);
		form.append('doc', doc);
		let res = await fetch('/bytes/sign', {
			method: 'POST',
			body: form
		});
		if (res.ok) {
			signature = await res.json();
			showDialog2 = true;
			showDialog1 = false;
		} else {
			throw new Error(await res.text());
		}
	};
</script>

<Dialog bind:value={showDialog1}>
	<h5 slot="title">Document</h5>
	<TextField bind:value={doc} textarea outlined min-length="1" />
	<div slot="actions">
		<Button text on:click={() => (showDialog1 = false)}>Cancel</Button>
		<Button text outlined disabled={!doc} on:click={async () => await submit()}>Sign</Button>
	</div>
</Dialog>

<Dialog bind:value={showDialog2}>
	<h5 slot="title">Signature</h5>
	<pre>
		<code>
		  {signature}
		</code>
	  </pre>
	<div slot="actions">
		<Button text on:click={() => (showDialog2 = false)}>OK</Button>
	</div>
</Dialog>

<!-- <div class="py-2"> -->
<Button on:click={() => (showDialog1 = true)}>Sign Bytes</Button>
<!-- </div> -->
